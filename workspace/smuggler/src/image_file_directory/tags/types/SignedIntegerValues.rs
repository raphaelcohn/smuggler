// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


/// Signed integer values (a slice).
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(u8)]
pub enum SignedIntegerValues<'tiff_bytes>
{
	#[allow(missing_docs)]
	I8(&'tiff_bytes [i8]),
	
	#[allow(missing_docs)]
	I16(&'tiff_bytes [Unaligned<i16>]),
	
	#[allow(missing_docs)]
	I32(&'tiff_bytes [Unaligned<i16>]),
	
	#[allow(missing_docs)]
	I64(&'tiff_bytes [Unaligned<i16>]),
}

impl<'tiff_bytes> SignedIntegerValues<'tiff_bytes>
{
	#[inline(always)]
	pub(in crate::image_file_directory::tags) fn parse<'allocator, TB: TiffBytes, A: Allocator + Clone, Unit: Version6OrBigTiffUnit>(common: &mut TagParserCommon<'tiff_bytes, 'allocator, TB, A, Unit>, tag_type: TagType, raw_tag_value: RawTagValue<'tiff_bytes>) -> Result<Self, SpecificTagParseError>
	{
		use TagType::*;
		use SignedIntegerValues::*;
		
		match tag_type
		{
			BYTE => TagType::invalid(),
			
			ASCII => TagType::invalid(),
			
			SHORT => TagType::invalid(),
			
			LONG => TagType::invalid(),
			
			RATIONAL => TagType::invalid(),
			
			SBYTE => Ok(I8(raw_tag_value.byte_slice())),
			
			UNDEFINED => TagType::invalid(),
			
			SSHORT => Ok(I16(raw_tag_value.unaligned_slice(common))),
			
			SLONG => Ok(I32(raw_tag_value.unaligned_slice(common))),
			
			SRATIONAL => TagType::invalid(),
			
			FLOAT => TagType::invalid(),
			
			DOUBLE => TagType::invalid(),
			
			IFD => TagType::invalid(),
			
			LONG8 => TagType::invalid(),
			
			SLONG8 => Ok(I64(raw_tag_value.unaligned_slice(common))),
			
			IFD8 => TagType::invalid(),
		}
	}
}
