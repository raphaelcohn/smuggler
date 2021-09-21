// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


/// An unsigned integer tag type code.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(u16)]
pub enum UnsignedIntegerTagType
{
	/// Legacy from before TIFF version 6.
	BYTE = TagType::Byte,
	
	/// Legacy from before TIFF version 6.
	SHORT = TagType::Short,
	
	/// Legacy from before TIFF version 6.
	LONG = TagType::Long,
	
	/// Defined in TIFF version BigTiff.
	LONG8 = TagType::Long8,
}

impl UnsignedIntegerTagType
{
	#[inline(always)]
	pub(in crate::tiff::image_file_directory) fn parse(tag_type: u16, tag_identifier: TagIdentifier) -> Result<Self, TagParseError>
	{
		use UnsignedIntegerTagType::*;
		
		match tag_type
		{
			TagType::Unrecognized0 => TagType::unrecognized(tag_type),
			
			TagType::Byte => Ok(BYTE),
			
			TagType::Ascii => TagType::invalid(tag_type, tag_identifier),
			
			TagType::Short => Ok(SHORT),
			
			TagType::Long => Ok(LONG),
			
			TagType::Rational ..= TagType::Ifd => TagType::invalid(tag_type, tag_identifier),
			
			TagType::Unrecognized14 ..= TagType::Unrecognized15 => TagType::unrecognized(tag_type),
			
			TagType::Long8 => Ok(LONG8),
			
			_ => TagType::unrecognized(tag_type)
		}
	}
	
	#[inline(always)]
	pub(in crate::tiff::image_file_directory) fn extract_bit_field_value<Unit: Version6OrBigTiffUnit>(self, tiff_bytes: &impl Bytes, offset_or_value_union_index: u64, byte_order: ByteOrder) -> Result<u64, TagParseError>
	{
		use UnsignedIntegerTagType::*;
		
		match self
		{
			BYTE => Ok(tiff_bytes.u8_unchecked(offset_or_value_union_index) as u64),
			
			SHORT => Ok(tiff_bytes.unaligned_u16_value_unchecked(offset_or_value_union_index) as u64),
			
			LONG => Ok(tiff_bytes.unaligned_u32_value_unchecked(offset_or_value_union_index) as u64),
			
			LONG8 => Unit::long8_offset_or_value(tiff_bytes, offset_or_value_union_index, byte_order),
		}
	}
}
