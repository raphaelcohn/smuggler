// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(in crate::image_file_directory) struct TagIdentifierParser
{
	previous_tag_identifier: Option<TagIdentifier>,
}

impl TagIdentifierParser
{
	#[inline(always)]
	pub(in crate::image_file_directory) fn parse(&mut self, tag_identifier: TagIdentifier) -> Result<TagIdentifier, TagParseError>
	{
		if let Some(previous_tag_identifier) = self.previous_tag_identifier.replace(tag_identifier)
		{
			if unlikely!(previous_tag_identifier >= tag_identifier)
			{
				return Err(TagParseError::OutOfSequenceTagIdentifier { tag_identifier, previous_tag_identifier })
			}
		}
		Ok(tag_identifier)
	}
}
