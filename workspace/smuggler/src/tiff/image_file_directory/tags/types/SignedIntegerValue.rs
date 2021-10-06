// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


/// Signed integer value.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum SignedIntegerValue
{
	/// A specialized variant used when a tag's count is zero.
	I0,
	
	#[allow(missing_docs)]
	I8(i8),
	
	#[allow(missing_docs)]
	I16(i16),
	
	#[allow(missing_docs)]
	I32(i32),
	
	#[allow(missing_docs)]
	I64(i64),
}

impl Default for SignedIntegerValue
{
	#[inline(always)]
	fn default() -> Self
	{
		SignedIntegerValue::I8(0)
	}
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
			I0 => i8(0),
			
			I8(value) => i8(value),
			
			I16(value) => i16(value),
			
			I32(value) => i32(value),
			
			I64(value) => i64(value),
		}
	}
	
	#[inline(always)]
	pub(in crate::tiff::image_file_directory::tags) fn parse_offset_or_value<Unit: Version6OrBigTiffUnit>(tag_type: u16, tiff_bytes: &impl TiffBytes, offset_or_value_union_index: Index, byte_order: ByteOrder) -> Result<Self, SpecificTagParseError>
	{
		use SignedIntegerValue::*;
		
		match tag_type
		{
			TagType::Unrecognized0 => TagType::unrecognized(),
			
			TagType::Byte => TagType::invalid(tag_type),
			
			TagType::Ascii => TagType::invalid(tag_type),
			
			TagType::Short => TagType::invalid(tag_type),
			
			TagType::Long => TagType::invalid(tag_type),
			
			TagType::Rational => TagType::invalid(tag_type),
			
			TagType::Sbyte => Ok(I8(tiff_bytes.byte_unchecked(offset_or_value_union_index))),
			
			TagType::Undefined => TagType::invalid(tag_type),
			
			TagType::Sshort => Ok(I16(tiff_bytes.unaligned_unchecked(offset_or_value_union_index))),
			
			TagType::Slong => Ok(I32(tiff_bytes.unaligned_unchecked(offset_or_value_union_index))),
			
			TagType::Srational => TagType::invalid(tag_type),
			
			TagType::Float => TagType::invalid(tag_type),
			
			TagType::Double => TagType::invalid(tag_type),
			
			TagType::Ifd => TagType::invalid(tag_type),
			
			TagType::Unrecognized14 => TagType::unrecognized(),
			
			TagType::Unrecognized15 => TagType::unrecognized(),
			
			TagType::Long8 => TagType::invalid(tag_type),
			
			TagType::Slong8 => Ok(I64(Unit::slong8_offset_or_value(tiff_bytes, offset_or_value_union_index, byte_order)?)),
			
			TagType::Ifd8 => TagType::invalid(tag_type),
			
			_ => TagType::unrecognized()
		}
	}
}
