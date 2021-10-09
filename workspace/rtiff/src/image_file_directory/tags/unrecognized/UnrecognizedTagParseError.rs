// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


/// A parse error.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum UnrecognizedTagParseError
{
	#[allow(missing_docs)]
	AsciiStringsParse(AsciiStringsParseError),
	
	#[allow(missing_docs)]
	ChildImageFileDirectoriesParse(ChildImageFileDirectoriesParseError),
}

impl Display for UnrecognizedTagParseError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for UnrecognizedTagParseError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use UnrecognizedTagParseError::*;
		
		match self
		{
			AsciiStringsParse(cause) => Some(cause),
			
			ChildImageFileDirectoriesParse(cause) => Some(cause),
		}
	}
}

impl From<AsciiStringsParseError> for UnrecognizedTagParseError
{
	#[inline(always)]
	fn from(cause: AsciiStringsParseError) -> Self
	{
		UnrecognizedTagParseError::AsciiStringsParse(cause)
	}
}

impl From<ChildImageFileDirectoriesParseError> for UnrecognizedTagParseError
{
	#[inline(always)]
	fn from(cause: ChildImageFileDirectoriesParseError) -> Self
	{
		UnrecognizedTagParseError::ChildImageFileDirectoriesParse(cause)
	}
}
