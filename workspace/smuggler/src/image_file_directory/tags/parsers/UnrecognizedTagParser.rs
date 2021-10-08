// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(in crate::image_file_directory) struct UnrecognizedTagParser;

impl<'tiff_bytes, 'allocator, A: Allocator + Clone, TEH: TagEventHandler<UnrecognizedTag<'tiff_bytes, A>>> TagParser<'tiff_bytes, 'allocator, A, TEH, UnrecognizedTag<'tiff_bytes, A>> for UnrecognizedTagParser
{
	#[inline(always)]
	fn finish<TB: TiffBytes, Unit: Version6OrBigTiffUnit>(self, _common: &mut TagParserCommon<'tiff_bytes, 'allocator, TB, A, Unit>, _tag_event_handler: &mut TEH) -> Result<(), FinishTagParseError>
	{
		Ok(())
	}
	
	#[inline(always)]
	fn parse<TB: TiffBytes, Unit: 'tiff_bytes + Version6OrBigTiffUnit>(&mut self, common: &mut TagParserCommon<'tiff_bytes, 'allocator, TB, A, Unit>, tag_event_handler: &mut TEH, tag_identifier: TagIdentifier, tag_type: TagType, raw_tag_value: RawTagValue<'tiff_bytes>) -> Result<(), SpecificTagParseError>
	{
		tag_event_handler.handle_tag_event(UnrecognizedTag::parse::<_, Unit>(common, tag_identifier, tag_type, raw_tag_value)?);
		Ok(())
	}
}
