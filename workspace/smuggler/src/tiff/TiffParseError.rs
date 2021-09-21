// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


/// A parse error.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) enum TiffParseError
{
	#[allow(missing_docs)]
	HeaderParse(HeaderParseError),
	
	#[allow(missing_docs)]
	ImageFileDirectoriesParse
	{
		cause: ImageFileDirectoriesParseError,
	
		header: Header,
	}
}

impl Display for TiffParseError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for TiffParseError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use TiffParseError::*;
		
		match self
		{
			HeaderParse(cause) => Some(cause),
			
			_ => None,
		}
	}
}

impl From<HeaderParseError> for TiffParseError
{
	#[inline(always)]
	fn from(cause: HeaderParseError) -> Self
	{
		TiffParseError::HeaderParse(cause)
	}
}
