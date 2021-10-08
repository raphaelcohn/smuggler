// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(in crate::tiff::image_file_directory) struct UnrecognizedTagParser;

impl<'tiff_bytes, 'recursion: 'recursion_guard, 'recursion_guard, A: Allocator + Copy, TEH: TagEventHandler<UnrecognizedTag<'tiff_bytes, A>>> TagParser<'tiff_bytes, 'recursion, 'recursion_guard, A, TEH, UnrecognizedTag<'tiff_bytes, A>> for UnrecognizedTagParser
{
	#[inline(always)]
	fn finish<TB: TiffBytes, Unit: Version6OrBigTiffUnit>(self, _common: &TagParserCommon<'tiff_bytes, 'recursion, 'recursion_guard, TB, A>, _tag_event_handler: &mut TEH) -> Result<(), SpecificTagParseError>
	{
		Ok(())
	}
	
	#[inline(always)]
	fn parse<TB: TiffBytes, Unit: Version6OrBigTiffUnit>(&mut self, common: &TagParserCommon<'tiff_bytes, 'recursion, 'recursion_guard, TB, A>, tag_event_handler: &mut TEH, tag_identifier: TagIdentifier, tag_type: TagType, raw_tag_value: RawTagValue<'tiff_bytes>) -> Result<(), SpecificTagParseError>
	{
		tag_event_handler.handle_tag_event(UnrecognizedTag::parse(common, tag_identifier, tag_type, raw_tag_value)?);
		Ok(())
	}
}
