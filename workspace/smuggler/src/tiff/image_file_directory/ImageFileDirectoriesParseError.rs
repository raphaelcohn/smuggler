// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


/// A parse error.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) enum ImageFileDirectoriesParseError
{
	#[allow(missing_docs)]
	ImageFileDirectoryParse
	{
		cause: ImageFileDirectoryParseError,
		
		image_file_directory_pointer: ImageFileDirectoryPointer,
		
		index: usize,
	},
	
	#[allow(missing_docs)]
	CouldNotAllocateMemoryForImageFileDirectoryStorage(TryReserveError),
	
	#[allow(missing_docs)]
	CyclicImageFileDirectoryPointer(ImageFileDirectoryPointer),
	
	#[allow(missing_docs)]
	CouldNotAllocateMemoryForImageFileDirectoryPointer(TryReserveError),
}

impl Display for ImageFileDirectoriesParseError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for ImageFileDirectoriesParseError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use ImageFileDirectoriesParseError::*;
		
		match self
		{
			ImageFileDirectoryParse { cause, .. } => Some(cause),
			
			CouldNotAllocateMemoryForImageFileDirectoryStorage(cause) => Some(cause),
			
			CouldNotAllocateMemoryForImageFileDirectoryPointer(cause) => Some(cause),
		}
	}
}
