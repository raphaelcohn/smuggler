// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


pub(in crate::tiff::image_file_directory) trait TagParser<'tiff_bytes, 'recursion: 'recursion_guard, 'recursion_guard, A: Allocator + Copy, TEH: TagEventHandler<T>, T: Tag>: Default
{
	fn finish<TB: TiffBytes, Unit: Version6OrBigTiffUnit>(self, common: &TagParserCommon<'tiff_bytes, 'recursion, 'recursion_guard, TB, A>, tag_event_handler: &mut TEH) -> Result<(), SpecificTagParseError>;
	
	#[inline(always)]
	fn parse_tag<TB: TiffBytes, Unit: Version6OrBigTiffUnit>(&mut self, common: &TagParserCommon<'tiff_bytes, 'recursion, 'recursion_guard, TB, A>, tag_event_handler: &mut TEH, tag_identifier: TagIdentifier, tag_type: TagType, tag_type_size_in_bytes: u64, count: u64, offset_or_value_union_index: Index) -> Result<(), SpecificTagParseError>
	{
		let raw_tag_value = RawTagValue::parse(common, tag_type_size_in_bytes, count, offset_or_value_union_index)?;
		
		self.parse::<TB, Unit>(common, tag_event_handler, tag_identifier, tag_type, raw_tag_value)
	}
	
	fn parse<TB: TiffBytes, Unit: Version6OrBigTiffUnit>(&mut self, common: &TagParserCommon<'tiff_bytes, 'recursion, 'recursion_guard, TB, A>, tag_event_handler: &mut TEH, tag_identifier: TagIdentifier, tag_type: TagType, raw_tag_value: RawTagValue<'tiff_bytes>) -> Result<(), SpecificTagParseError>;
}
