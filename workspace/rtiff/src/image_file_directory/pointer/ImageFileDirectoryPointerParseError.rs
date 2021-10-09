// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


/// A parse error.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum ImageFileDirectoryPointerParseError
{
	#[allow(missing_docs)]
	OffsetParse(OffsetParseError),
	
	#[allow(missing_docs)]
	NotAlignedToA16BitWordBoundary
	{
		raw_offset: u64,
	},
}

impl Display for ImageFileDirectoryPointerParseError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for ImageFileDirectoryPointerParseError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use ImageFileDirectoryPointerParseError::*;
		
		match self
		{
			OffsetParse(cause) => Some(cause),
			
			_ => None,
		}
	}
}
