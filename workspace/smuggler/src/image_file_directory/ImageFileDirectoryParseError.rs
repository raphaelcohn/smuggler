// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


/// A parse error.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ImageFileDirectoryParseError
{
	#[allow(missing_docs)]
	NotEnoughBytesForNumberOfDirectoryElements(OverflowError),
	
	#[allow(missing_docs)]
	ThereAreNoDirectoryEntries,
	
	#[allow(missing_docs)]
	NumberOfDirectoryEntryBytesOverflows
	{
		number_of_directory_entries: NonZeroU64,
	},
	
	#[allow(missing_docs)]
	DirectoryEntriesArrayStartIndexWouldOverflow
	{
		number_of_directory_entries: NonZeroU64,
	},
	
	#[allow(missing_docs)]
	NextImageFileDirectoryPointerStartIndexWouldOverflow
	{
		number_of_directory_entry_bytes: NonZeroU64,
	},
	
	#[allow(missing_docs)]
	EntriesAndImageFileDirectoryPointerRequireTooManyBytes
	{
		number_of_directory_entries: NonZeroU64,
	},
	
	#[allow(missing_docs)]
	NextImageFileDirectoryPointerParse(ImageFileDirectoryPointerParseError),
	
	#[allow(missing_docs)]
	CouldNotAllocateMemoryForDirectoryEntries(TryReserveError),
	
	#[allow(missing_docs)]
	TagParse(TagParseError),
	
	#[allow(missing_docs)]
	FinishTagParse(FinishTagParseError),
}

impl Display for ImageFileDirectoryParseError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for ImageFileDirectoryParseError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use ImageFileDirectoryParseError::*;
		
		match self
		{
			NotEnoughBytesForNumberOfDirectoryElements(cause) => Some(cause),
			
			NextImageFileDirectoryPointerParse(cause) => Some(cause),
			
			CouldNotAllocateMemoryForDirectoryEntries(cause) => Some(cause),
			
			TagParse(cause) => Some(cause),
			
			FinishTagParse(cause) => Some(cause),
			
			_ => None,
		}
	}
}

impl From<TagParseError> for ImageFileDirectoryParseError
{
	#[inline(always)]
	fn from(cause: TagParseError) -> Self
	{
		ImageFileDirectoryParseError::TagParse(cause)
	}
}

impl From<FinishTagParseError> for ImageFileDirectoryParseError
{
	#[inline(always)]
	fn from(cause: FinishTagParseError) -> Self
	{
		ImageFileDirectoryParseError::FinishTagParse(cause)
	}
}
