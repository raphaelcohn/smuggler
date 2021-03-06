// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


/// Signed integer value.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(u8)]
pub enum SignedIntegerValue
{
	#[allow(missing_docs)]
	I8(i8),
	
	#[allow(missing_docs)]
	I16(i16),
	
	#[allow(missing_docs)]
	I32(i32),
	
	#[allow(missing_docs)]
	I64(i64),
}

impl Into<i8> for SignedIntegerValue
{
	#[inline(always)]
	fn into(self) -> i8
	{
		self.callback(|value| value, |value| value as i8, |value| value as i8, |value| value as i8)
	}
}

impl Into<i16> for SignedIntegerValue
{
	#[inline(always)]
	fn into(self) -> i16
	{
		self.callback(|value| value as i16, |value| value, |value| value as i16, |value| value as i16)
	}
}

impl Into<i32> for SignedIntegerValue
{
	#[inline(always)]
	fn into(self) -> i32
	{
		self.callback(|value| value as i32, |value| value as i32, |value| value, |value| value as i32)
	}
}

impl Into<i64> for SignedIntegerValue
{
	#[inline(always)]
	fn into(self) -> i64
	{
		self.callback(|value| value as i64, |value| value as i64, |value| value as i64, |value| value)
	}
}

impl SignedIntegerValue
{
	#[inline(always)]
	fn callback<R>(self, i8: impl FnOnce(i8) -> R, i16: impl FnOnce(i16) -> R, i32: impl FnOnce(i32) -> R, i64: impl FnOnce(i64) -> R) -> R
	{
		use SignedIntegerValue::*;
		
		match self
		{
			I8(value) => i8(value),
			
			I16(value) => i16(value),
			
			I32(value) => i32(value),
			
			I64(value) => i64(value),
		}
	}
	
	#[inline(always)]
	pub(in crate::image_file_directory::tags) fn parse(byte_order: ByteOrder, tag_type: TagType, raw_tag_value: RawTagValue) -> Result<Self, IntegerValueParseError>
	{
		if unlikely!(raw_tag_value.count != 1)
		{
			return Err(IntegerValueParseError::CountShouldBeOne)
		}
		Self::parse_offset_or_value(tag_type, raw_tag_value.slice, byte_order)
	}
	
	#[inline(always)]
	fn parse_offset_or_value(tag_type: TagType, slice: NonNull<[u8]>, byte_order: ByteOrder) -> Result<Self, IntegerValueParseError>
	{
		use TagType::*;
		use SignedIntegerValue::*;
		
		#[inline(always)]
		const fn invalid() -> Result<SignedIntegerValue, IntegerValueParseError>
		{
			Err(IntegerValueParseError::TagTypeInvalid)
		}
		
		match tag_type
		{
			BYTE => invalid(),
			
			ASCII => invalid(),
			
			SHORT => invalid(),
			
			LONG => invalid(),
			
			RATIONAL => invalid(),
			
			SBYTE => Ok(I8(slice.as_non_null_ptr().cast().read_unaligned())),
			
			UNDEFINED => invalid(),
			
			SSHORT => Self::read_unaligned_and_byte_swap_as_appropriate(slice, byte_order, I16),
			
			SLONG => Self::read_unaligned_and_byte_swap_as_appropriate(slice, byte_order, I32),
			
			SRATIONAL => invalid(),
			
			FLOAT => invalid(),
			
			DOUBLE => invalid(),
			
			IFD => invalid(),
			
			LONG8 => invalid(),
			
			SLONG8 => Self::read_unaligned_and_byte_swap_as_appropriate(slice, byte_order, I64),
			
			IFD8 => invalid(),
		}
	}
	
	#[inline(always)]
	fn read_unaligned_and_byte_swap_as_appropriate<CBU: CanBeUnaligned>(slice: NonNull<[u8]>, byte_order: ByteOrder, constructor: impl FnOnce(CBU) -> Self) -> Result<Self, IntegerValueParseError>
	{
		Ok(constructor(CBU::read_unaligned_and_byte_swap_as_appropriate(slice.as_non_null_ptr().cast(), byte_order)))
	}
}
