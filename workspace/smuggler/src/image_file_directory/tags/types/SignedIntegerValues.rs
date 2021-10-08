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

impl<'tiff_bytes> IntegerValues for SignedIntegerValues<'tiff_bytes>
{
	#[inline(always)]
	fn len(self) -> usize
	{
		use SignedIntegerValues::*;
		
		match self
		{
			I8(slice) => slice.len(),
			
			I16(slice) => slice.len(),
			
			I32(slice) => slice.len(),
			
			I64(slice) => slice.len(),
		}
	}
}

impl<'tiff_bytes> SignedIntegerValues<'tiff_bytes>
{
	#[inline(always)]
	pub(in crate::image_file_directory::tags) fn parse<'allocator, TB: TiffBytes, A: Allocator + Clone, Unit: Version6OrBigTiffUnit>(common: &mut TagParserCommon<'tiff_bytes, 'allocator, TB, A, Unit>, tag_type: TagType, raw_tag_value: RawTagValue<'tiff_bytes>) -> Result<Self, IntegerValuesParseError>
	{
		use TagType::*;
		use SignedIntegerValues::*;
		
		#[inline(always)]
		const fn invalid<'tiff_bytes>() -> Result<SignedIntegerValues<'tiff_bytes>, IntegerValuesParseError>
		{
			Err(IntegerValuesParseError::TagTypeInvalid)
		}
		
		match tag_type
		{
			BYTE => invalid(),
			
			ASCII => invalid(),
			
			SHORT => invalid(),
			
			LONG => invalid(),
			
			RATIONAL => invalid(),
			
			SBYTE => Ok(I8(raw_tag_value.byte_slice())),
			
			UNDEFINED => invalid(),
			
			SSHORT => Ok(I16(raw_tag_value.unaligned_slice(common))),
			
			SLONG => Ok(I32(raw_tag_value.unaligned_slice(common))),
			
			SRATIONAL => invalid(),
			
			FLOAT => invalid(),
			
			DOUBLE => invalid(),
			
			IFD => invalid(),
			
			LONG8 => invalid(),
			
			SLONG8 => Ok(I64(raw_tag_value.unaligned_slice(common))),
			
			IFD8 => invalid(),
		}
	}
}
