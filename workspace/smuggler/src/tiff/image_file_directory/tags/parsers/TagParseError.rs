// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


/// A parse error.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum TagParseError
{
	#[allow(missing_docs)]
	OutOfSequenceTagIdentifier
	{
		tag_identifier: TagIdentifier,
		
		previous_tag_identifier: TagIdentifier,
	},
	
	#[allow(missing_docs)]
	SpecificTagParse
	{
		cause: SpecificTagParseError,
		
		tag_identifier: TagIdentifier,
		
		tag_type: u16,
		
		count: u64,
		
		offset_or_value_union_index: u64,
	}
}

impl Display for TagParseError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for TagParseError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use TagParseError::*;
		
		match self
		{
			SpecificTagParse { cause, .. } => Some(cause),
			
			_ => None,
		}
	}
}
