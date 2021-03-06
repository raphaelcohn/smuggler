// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


/// A parse error.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TiffParseError
{
	#[allow(missing_docs)]
	HeaderParse(HeaderParseError),
	
	#[allow(missing_docs)]
	FreeSpaceOutOfMemory(FreeSpaceOutOfMemoryError),
	
	#[allow(missing_docs)]
	ImageFileDirectoriesParse(ImageFileDirectoriesParseError),
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
			
			FreeSpaceOutOfMemory(cause) => Some(cause),
			
			ImageFileDirectoriesParse(cause) => Some(cause),
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

impl From<FreeSpaceOutOfMemoryError> for TiffParseError
{
	#[inline(always)]
	fn from(cause: FreeSpaceOutOfMemoryError) -> Self
	{
		TiffParseError::FreeSpaceOutOfMemory(cause)
	}
}

impl From<ImageFileDirectoriesParseError> for TiffParseError
{
	#[inline(always)]
	fn from(cause: ImageFileDirectoriesParseError) -> Self
	{
		TiffParseError::ImageFileDirectoriesParse(cause)
	}
}
