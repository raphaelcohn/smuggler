// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


/// Unsigned integer values (a slice).
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(u8)]
pub enum UnsignedIntegerValues<'tiff_bytes>
{
	#[allow(missing_docs)]
	U8(&'tiff_bytes [u8]),
	
	#[allow(missing_docs)]
	U16(&'tiff_bytes [Unaligned<u16>]),
	
	#[allow(missing_docs)]
	U32(&'tiff_bytes [Unaligned<u16>]),
	
	#[allow(missing_docs)]
	U64(&'tiff_bytes [Unaligned<u16>]),
}

impl<'tiff_bytes> IntegerValues for UnsignedIntegerValues<'tiff_bytes>
{
	#[inline(always)]
	fn len(self) -> usize
	{
		use UnsignedIntegerValues::*;
		
		match self
		{
			U8(slice) => slice.len(),
			
			U16(slice) => slice.len(),
			
			U32(slice) => slice.len(),
			
			U64(slice) => slice.len(),
		}
	}
}

impl<'tiff_bytes> UnsignedIntegerValues<'tiff_bytes>
{
	#[inline(always)]
	pub(in crate::image_file_directory::tags) fn parse<'allocator, TB: TiffBytes, A: Allocator + Clone, Unit: Version6OrBigTiffUnit>(common: &mut TagParserCommon<'tiff_bytes, 'allocator, TB, A, Unit>, tag_type: TagType, raw_tag_value: RawTagValue<'tiff_bytes>) -> Result<Self, IntegerValuesParseError>
	{
		use TagType::*;
		use UnsignedIntegerValues::*;
		
		#[inline(always)]
		const fn invalid<'tiff_bytes>() -> Result<UnsignedIntegerValues<'tiff_bytes>, IntegerValuesParseError>
		{
			Err(IntegerValuesParseError::TagTypeInvalid)
		}
		
		match tag_type
		{
			BYTE => Ok(U8(raw_tag_value.byte_slice())),
			
			ASCII => invalid(),
			
			SHORT => Ok(U16(raw_tag_value.unaligned_slice(common))),
			
			LONG => Ok(U32(raw_tag_value.unaligned_slice(common))),
			
			RATIONAL => invalid(),
			
			SBYTE => invalid(),
			
			UNDEFINED => invalid(),
			
			SSHORT => invalid(),
			
			SLONG => invalid(),
			
			SRATIONAL => invalid(),
			
			FLOAT => invalid(),
			
			DOUBLE => invalid(),
			
			IFD => invalid(),
			
			LONG8 => Ok(U64(raw_tag_value.unaligned_slice(common))),
			
			SLONG8 => invalid(),
			
			IFD8 => invalid(),
		}
	}
}
