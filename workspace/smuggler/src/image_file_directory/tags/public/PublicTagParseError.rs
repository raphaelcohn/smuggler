// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


/// A parse error.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PublicTagParseError
{
	#[allow(missing_docs)]
	IntegerValueParse(IntegerValueParseError),
	
	#[allow(missing_docs)]
	IntegerValuesParse(IntegerValuesParseError),
	
	#[allow(missing_docs)]
	OffsetsArrayParse(OffsetsArrayParseError),
	
	#[allow(missing_docs)]
	UnrecognizedTagParse(UnrecognizedTagParseError),
}

impl Display for PublicTagParseError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for PublicTagParseError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use PublicTagParseError::*;
		
		match self
		{
			IntegerValueParse(cause) => Some(cause),
			
			IntegerValuesParse(cause) => Some(cause),
			
			OffsetsArrayParse(cause) => Some(cause),
			
			UnrecognizedTagParse(cause) => Some(cause),
		}
	}
}

impl From<IntegerValueParseError> for PublicTagParseError
{
	#[inline(always)]
	fn from(cause: IntegerValueParseError) -> Self
	{
		PublicTagParseError::IntegerValueParse(cause)
	}
}

impl From<IntegerValuesParseError> for PublicTagParseError
{
	#[inline(always)]
	fn from(cause: IntegerValuesParseError) -> Self
	{
		PublicTagParseError::IntegerValuesParse(cause)
	}
}

impl From<OffsetsArrayParseError> for PublicTagParseError
{
	#[inline(always)]
	fn from(cause: OffsetsArrayParseError) -> Self
	{
		PublicTagParseError::OffsetsArrayParse(cause)
	}
}

impl From<UnrecognizedTagParseError> for PublicTagParseError
{
	#[inline(always)]
	fn from(cause: UnrecognizedTagParseError) -> Self
	{
		PublicTagParseError::UnrecognizedTagParse(cause)
	}
}
