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
	pub(in crate::tiff::image_file_directory::tags) fn parse_offset_or_value<Unit: Version6OrBigTiffUnit>(tag_type: u16, tiff_bytes: &impl TiffBytes, offset_or_value_union_index: Index, byte_order: ByteOrder) -> Result<Self, SpecificTagParseError>
	{
		use UnsignedIntegerValue::*;
		
		match tag_type
		{
			TagType::Unrecognized0 => TagType::unrecognized(),
			
			TagType::Byte => Ok(U8(tiff_bytes.byte_unchecked(offset_or_value_union_index))),
			
			TagType::Ascii => TagType::invalid(tag_type),
			
			TagType::Short => Ok(U16(tiff_bytes.unaligned_unchecked(offset_or_value_union_index))),
			
			TagType::Long => Ok(U32(tiff_bytes.unaligned_unchecked(offset_or_value_union_index))),
			
			TagType::Rational => TagType::invalid(tag_type),
			
			TagType::Sbyte => TagType::invalid(tag_type),
			
			TagType::Undefined => TagType::invalid(tag_type),
			
			TagType::Sshort => TagType::invalid(tag_type),
			
			TagType::Slong => TagType::invalid(tag_type),
			
			TagType::Srational => TagType::invalid(tag_type),
			
			TagType::Float => TagType::invalid(tag_type),
			
			TagType::Double => TagType::invalid(tag_type),
			
			TagType::Ifd => TagType::invalid(tag_type),
			
			TagType::Unrecognized14 => TagType::unrecognized(),
			
			TagType::Unrecognized15 => TagType::unrecognized(),
			
			TagType::Long8 => Ok(U64(Unit::long8_offset_or_value(tiff_bytes, offset_or_value_union_index, byte_order)?)),
			
			TagType::Slong8 => TagType::invalid(tag_type),
			
			TagType::Ifd8 => TagType::invalid(tag_type),
			
			_ => TagType::unrecognized()
		}
	}
}
