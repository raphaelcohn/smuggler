// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(in crate::tiff::image_file_directory) struct PublicTagParser;

impl TagParser for PublicTagParser
{
	type Tag<'a, A: Allocator> = PublicTag<'a, A>;
	
	#[inline(always)]
	fn parse<'a, A, Unit: Version6OrBigTiffUnit, B: Bytes>(&self, allocator: A, tiff_bytes: &'a B, tag_identifier: TagIdentifier, tag_type: u16, count: u64, byte_order: ByteOrder, offset_or_value_union_index: u64) -> Result<PublicTag<'a, A>, TagParseError>
	{
		/// See <https://www.loc.gov/preservation/digital/formats/content/tiff_tags.shtml>.
		/// See <https://www.awaresystems.be/imaging/tiff/tifftags.html>.
		match tag_identifier
		{
			// <https://www.awaresystems.be/imaging/tiff/tifftags/newsubfiletype.html>.
			// A single bit-field, parse any unsigned type as the value (although supposedly LONG).
			// In version BigTiff, will always fit in the offset field.
			// In version 6, should always fit in the offset field unless LONG8 is used (which, technically, is not a Version6 type but we should be lenient).
			// So - count should always be 1, tag_type should be restricted.
			0x00FE =>
			{
				
				let bits = match count
				{
					// Leniency: can get away with any tag_type, even unrecognized ones.
					0 => 0u64,
					
					1 =>
					{
						// TODO: We match twice - once to parse, once to use the parsed value. Optimize!
						let unsigned_integer_tag_type = UnsignedIntegerTagType::parse(tag_type, tag_identifier)?;
						unsigned_integer_tag_type.extract_bit_field_value::<Unit>(tiff_bytes, offset_or_value_union_index, byte_order)?;
					},
					
					_ => return Err(TagParseError::CountExceedsOne(tag_identifier))
				};
				
				
				NewSubFileType
			},
			
			0x00FF => SubFileType,
			
			0x0100 => ImageWidth,
			
			0x0101 => ImageLength,
		}
	}
}
