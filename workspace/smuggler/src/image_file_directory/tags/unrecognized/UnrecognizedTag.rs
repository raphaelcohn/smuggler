// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


/// An unrecognized tag.
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct UnrecognizedTag<'tiff_bytes, A: Allocator>(pub TagIdentifier, pub UnrecognizedTagValue<'tiff_bytes, A>);

impl<'tiff_bytes, A: Allocator> Tag for UnrecognizedTag<'tiff_bytes, A>
{
	/// Key type.
	type Key = TagIdentifier;
	
	/// Obtain key.
	fn key(&self) -> Self::Key
	{
		self.0
	}
}

impl<'tiff_bytes, A: Allocator + Clone> UnrecognizedTag<'tiff_bytes, A>
{
	#[inline(always)]
	pub(in crate::image_file_directory) fn parse<'allocator, TB: TiffBytes, Version: 'tiff_bytes + Version6OrBigTiffVersion>(common: &mut TagParserCommon<'tiff_bytes, 'allocator, TB, A, Version>, tag_identifier: TagIdentifier, tag_type: TagType, raw_tag_value: RawTagValue<'tiff_bytes>) -> Result<UnrecognizedTag<'tiff_bytes, A>, UnrecognizedTagParseError>
	{
		let unrecognized_tag_value = UnrecognizedTagValue::parse::<_, Version>(common, tag_type, raw_tag_value)?;
		Ok(UnrecognizedTag(tag_identifier, unrecognized_tag_value))
	}
}
