// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


/// A parse error.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FinishParseError
{
	#[allow(missing_docs)]
	Public(PublicTagFinishParseError),
	
	#[allow(missing_docs)]
	Unrecognized(UnrecognizedTagFinishParseError),
}

impl Display for FinishParseError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for FinishParseError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use FinishParseError::*;
		
		match self
		{
			Public(cause) => Some(cause),
			
			Unrecognized(cause) => Some(cause),
		}
	}
}

impl From<PublicTagFinishParseError> for FinishParseError
{
	#[inline(always)]
	fn from(cause: PublicTagFinishParseError) -> Self
	{
		FinishParseError::Public(cause)
	}
}

impl From<UnrecognizedTagFinishParseError> for FinishParseError
{
	#[inline(always)]
	fn from(cause: UnrecognizedTagFinishParseError) -> Self
	{
		FinishParseError::Unrecognized(cause)
	}
}
