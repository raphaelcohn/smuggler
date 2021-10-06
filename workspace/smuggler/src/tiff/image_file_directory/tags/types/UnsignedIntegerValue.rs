// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


/// Unsigned integer value.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum UnsignedIntegerValue
{
	/// A specialized variant used when a tag's count is zero.
	U0,
	
	#[allow(missing_docs)]
	U8(u8),
	
	#[allow(missing_docs)]
	U16(u16),
	
	#[allow(missing_docs)]
	U32(u32),
	
	#[allow(missing_docs)]
	U64(u64),
}

impl Default for UnsignedIntegerValue
{
	#[inline(always)]
	fn default() -> Self
	{
		UnsignedIntegerValue::U0
	}
}

impl Into<u8> for UnsignedIntegerValue
{
	#[inline(always)]
	fn into(self) -> u8
	{
		self.callback(|value| value, |value| value as u8, |value| value as u8, |value| value as u8)
	}
}

impl Into<u16> for UnsignedIntegerValue
{
	#[inline(always)]
	fn into(self) -> u16
	{
		self.callback(|value| value as u16, |value| value, |value| value as u16, |value| value as u16)
	}
}

impl Into<u32> for UnsignedIntegerValue
{
	#[inline(always)]
	fn into(self) -> u32
	{
		self.callback(|value| value as u32, |value| value as u32, |value| value, |value| value as u32)
	}
}

impl Into<u64> for UnsignedIntegerValue
{
	#[inline(always)]
	fn into(self) -> u64
	{
		self.callback(|value| value as u64, |value| value as u64, |value| value as u64, |value| value)
	}
}

impl UnsignedIntegerValue
{
	#[inline(always)]
	fn callback<R>(self, u8: impl FnOnce(u8) -> R, u16: impl FnOnce(u16) -> R, u32: impl FnOnce(u32) -> R, u64: impl FnOnce(u64) -> R) -> R
	{
		use UnsignedIntegerValue::*;
		
		match self
		{
			U0 => u8(0),
			
			U8(value) => u8(value),
			
			U16(value) => u16(value),
			
			U32(value) => u32(value),
			
			U64(value) => u64(value),
		}
	}
	
	#[inline(always)]
	pub(in crate::tiff::image_file_directory::tags) fn parse_offset_or_value(tag_type: TagType, slice: NonNull<[u8]>, byte_order: ByteOrder) -> Result<Self, SpecificTagParseError>
	{
		use TagType::*;
		use UnsignedIntegerValue::*;
		
		match tag_type
		{
			BYTE => Ok(U8(slice.as_non_null_ptr().read_unaligned())),
			
			ASCII => TagType::invalid(),
			
			SHORT => Self::read_unaligned_and_byte_swap_as_appropriate(slice, byte_order, U16),
			
			LONG => Self::read_unaligned_and_byte_swap_as_appropriate(slice, byte_order, U32),
			
			RATIONAL => TagType::invalid(),
			
			SBYTE => TagType::invalid(),
			
			UNDEFINED => TagType::invalid(),
			
			SSHORT => TagType::invalid(),
			
			SLONG => TagType::invalid(),
			
			SRATIONAL => TagType::invalid(),
			
			FLOAT => TagType::invalid(),
			
			DOUBLE => TagType::invalid(),
			
			IFD => TagType::invalid(),
			
			LONG8 => Self::read_unaligned_and_byte_swap_as_appropriate(slice, byte_order, U64),
			
			SLONG8 => TagType::invalid(),
			
			IFD8 => TagType::invalid(),
		}
	}
	
	#[inline(always)]
	fn read_unaligned_and_byte_swap_as_appropriate<CBU: CanBeUnaligned>(slice: NonNull<[u8]>, byte_order: ByteOrder, constructor: impl FnOnce(CBU) -> Self) -> Result<Self, SpecificTagParseError>
	{
		Ok(constructor(CBU::read_unaligned_and_byte_swap_as_appropriate(slice.as_non_null_ptr().cast(), byte_order)))
	}
}
