// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


/// A parse error.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SpecificTagParseError
{
	#[allow(missing_docs)]
	MaximumDescentReached,
	
	#[allow(missing_docs)]
	UnrecognizedTagType,
	
	#[allow(missing_docs)]
	InvalidTagTypeForTagIdentifier,
	
	#[allow(missing_docs)]
	SliceOffsetParse(OffsetParseError),
	
	#[allow(missing_docs)]
	Long8OffsetParse(OffsetParseError),
	
	#[allow(missing_docs)]
	Long8Overflow(OverflowError),
	
	#[allow(missing_docs)]
	Slong8OffsetParse(OffsetParseError),
	
	#[allow(missing_docs)]
	Slong8Overflow(OverflowError),
	
	#[allow(missing_docs)]
	CountShouldBeOne,
	
	#[allow(missing_docs)]
	CountIsTooLargeForTargetArchitecture,
	
	#[allow(missing_docs)]
	OffsetIsTooLargeForTargetArchitecture(OverflowError),
	
	#[allow(missing_docs)]
	CouldNotAllocateMemoryForAsciiStringReference(TryReserveError),
	
	#[allow(missing_docs)]
	CouldNotAllocateMemoryForImageFileDirectories(TryReserveError),
	
	#[allow(missing_docs)]
	ImageFileDirectoryPointerParse(ImageFileDirectoryPointerParseError),
	
	#[allow(missing_docs)]
	ImageFileDirectoryPointerIsNull,
	
	/// Can occur when parsing a sub image file directory.
	CouldNotAllocateMemoryForImageFileDirectoriesParseError(AllocError),
	
	/// Can occur when parsing a sub image file directory.
	ImageFileDirectoriesParse(Box<ImageFileDirectoriesParseError, Global>),
}

impl Display for SpecificTagParseError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for SpecificTagParseError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use SpecificTagParseError::*;
		
		match self
		{
			SliceOffsetParse(cause) => Some(cause),
			
			Long8OffsetParse(cause) => Some(cause),
			
			Long8Overflow(cause) => Some(cause),
			
			Slong8OffsetParse(cause) => Some(cause),
			
			Slong8Overflow(cause) => Some(cause),
			
			OffsetIsTooLargeForTargetArchitecture(cause) => Some(cause),
			
			CouldNotAllocateMemoryForAsciiStringReference(cause) => Some(cause),
			
			CouldNotAllocateMemoryForImageFileDirectories(cause) => Some(cause),
			
			ImageFileDirectoryPointerParse(cause) => Some(cause),
			
			CouldNotAllocateMemoryForImageFileDirectoriesParseError(cause) => Some(cause),
			
			_ => None,
		}
	}
}
