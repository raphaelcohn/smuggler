// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(in crate::tiff::image_file_directory) struct PublicTagParser;

impl TagParser for PublicTagParser
{
	type Tag<'a, A: Allocator> = PublicTag<'a, A>;
	
	#[inline(always)]
	fn parse<'a, A, Unit: Version6OrBigTiffUnit, B: Bytes>(&self, allocator: A, tiff_bytes: &'a mut B, tag_identifier: TagIdentifier, tag_type: u16, count: u64, byte_order: ByteOrder, offset_or_value_union_index: u64) -> Result<PublicTag<'a, A>, TagParseError>
	{
		self.parse_inner(allocator, tiff_bytes, tag_identifier, tag_type, count, byte_order, offset_or_value_union_index).map_err(|cause| TagParseError::SpecificTagParse { cause, tag_identifier, tag_type, count, offset_or_value_union_index })
	}
}

impl PublicTagParser
{
	fn parse_inner<'a, A, Unit: Version6OrBigTiffUnit, B: Bytes>(&self, allocator: A, tiff_bytes: &'a mut B, tag_identifier: TagIdentifier, tag_type: u16, count: u64, byte_order: ByteOrder, offset_or_value_union_index: u64) -> Result<PublicTag<'a, A>, SpecificTagParseError>
	{
		use SpecificTagParseError::*;
		
		let public_tag = match tag_identifier
		{
			// <https://www.awaresystems.be/imaging/tiff/tifftags/newsubfiletype.html>.
			NewSubfileType => PublicTag::NewSubfileType
			(
				BitFieldInteger::from
				(
					match count
					{
						0 => UnsignedIntegerValue::U0,
						
						1 => UnsignedIntegerValue::parse_offset_or_value::<Unit>(tag_type, tiff_bytes, offset_or_value_union_index, byte_order)?,
						
						_ => return Err(CountExceedsOne)
					}
				)
			),
			
			SubfileType => PublicTag::SubfileType
			(
				EnumUnsignedInteger::from
				(
					match count
					{
						0 => UnsignedIntegerValue::U0,
						
						1 => UnsignedIntegerValue::parse_offset_or_value::<Unit>(tag_type, tiff_bytes, offset_or_value_union_index, byte_order)?,
						
						_ => return Err(CountExceedsOne)
					}
				)
			),
			
			ImageWidth => PublicTag::ImageWidth
			(
				UnsignedInteger::from
				(
					match count
					{
						0 => UnsignedIntegerValue::U0,
						
						1 => UnsignedIntegerValue::parse_offset_or_value::<Unit>(tag_type, tiff_bytes, offset_or_value_union_index, byte_order)?,
						
						_ => return Err(CountExceedsOne)
					}
				)
			),
			
			ImageLength => PublicTag::ImageLength
			(
				UnsignedInteger::from
				(
					match count
					{
						0 => UnsignedIntegerValue::U0,
						
						1 => UnsignedIntegerValue::parse_offset_or_value::<Unit>(tag_type, tiff_bytes, offset_or_value_union_index, byte_order)?,
						
						_ => return Err(CountExceedsOne)
					}
				)
			),
			
			BitsPerSample => PublicTag::BitsPerSample
			(
				// should be an array of short. do we want to be lenient and specify and an array of any unsigned type?
			),
			
			
			
			_ => PublicTag::Unrecognized(tag_identifier, UnrecognizedTagValue::parse(tag_type, tiff_bytes, count, byte_order, offset_or_value_union_index)?),
		};
		
		Ok(public_tag)
	}
}