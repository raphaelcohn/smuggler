// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(in crate::tiff::image_file_directory) struct PublicTagParser;

impl TagParser for PublicTagParser
{
	type Tag<'a, A: Allocator> = PublicTag<'a, A>;
	
	#[inline(always)]
	fn parse<'a, A, Unit: Version6OrBigTiffUnit, TB: TiffBytes>(&self, recursion_guard: &RecursionGuard, allocator: A, tiff_bytes: &'a mut TB, tag_identifier: TagIdentifier, tag_type: TagType, count: u64, byte_order: ByteOrder, slice: NonNull<[u8]>) -> Result<Self::Tag<'a, A>, SpecificTagParseError>
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
						
						1 => UnsignedIntegerValue::parse_offset_or_value(tag_type, slice, byte_order)?,
						
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
						
						1 => UnsignedIntegerValue::parse_offset_or_value(tag_type, slice, byte_order)?,
						
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
						
						1 => UnsignedIntegerValue::parse_offset_or_value(tag_type, slice, byte_order)?,
						
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
						// TODO: DO we want to support this sort of any-type leniency?
						0 => UnsignedIntegerValue::U0,
						
						1 => UnsignedIntegerValue::parse_offset_or_value(tag_type, slice, byte_order)?,
						
						_ => return Err(CountExceedsOne)
					}
				)
			),
			
			// TODO: should be an array of short. do we want to be lenient and specify an array of any unsigned type?
			BitsPerSample => PublicTag::BitsPerSample
			(
				match tag_type
				{
					TagType::SHORT => u16::slice_unaligned_and_byte_swap_as_appropriate(count, byte_order, slice),
					
					_ => return TagType::invalid()
				}
			),
			
			_ => PublicTag::Unrecognized(UnrecognizedTag::parse(recursion_guard, allocator, tiff_bytes, tag_identifier, tag_type, count, byte_order, slice)?),
		};
		
		Ok(public_tag)
	}
}
