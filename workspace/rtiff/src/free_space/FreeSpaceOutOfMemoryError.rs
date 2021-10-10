// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


/// An out-of-memory condition when trying to record free space usage.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FreeSpaceOutOfMemoryError
{
	#[allow(missing_docs)]
	Creation(TryReserveError),
	
	/// This is almost certainly never going to occur for 2 reasons:-
	///
	/// * This is the very first attempt to record used space, so memory will be available;
	/// * Since it starts from index 0, there is no need to do an insert in the underlying `FreeSpaceVector` data structure using Scenario 1d.
	RecordingHeaderSize(TryReserveError),

	#[allow(missing_docs)]
	RecordingImageFileDirectoryNumberOfDirectoryEntries(TryReserveError),

	#[allow(missing_docs)]
	RecordingNextImageFileDirectoryPointer(TryReserveError),

	#[allow(missing_docs)]
	RecordingDirectoryEntryIncludingCount(TryReserveError),

	#[allow(missing_docs)]
	RecordingSlice(TryReserveError),
}

impl Display for FreeSpaceOutOfMemoryError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for FreeSpaceOutOfMemoryError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use FreeSpaceOutOfMemoryError::*;
		
		match self
		{
			Creation(cause) => Some(cause),
			
			RecordingHeaderSize(cause) => Some(cause),
			
			RecordingImageFileDirectoryNumberOfDirectoryEntries(cause) => Some(cause),
			
			RecordingDirectoryEntryIncludingCount(cause) => Some(cause),
			
			RecordingSlice(cause) => Some(cause),
			
			_ => None,
		}
	}
}
