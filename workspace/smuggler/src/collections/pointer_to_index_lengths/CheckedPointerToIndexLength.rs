// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub(super) struct CheckedPointerToIndexLength;

impl PointerToIndexLength for CheckedPointerToIndexLength
{
	type CheckOutcome = Result<(), OverflowError>;
	
	#[inline(always)]
	fn check_inner(index: u64, size: u64, file_length: FileLength) -> Self::CheckOutcome
	{
		use OverflowError::*;
		
		let end_pointer = match index.checked_add(size)
		{
			None => return Err(SizeOverflowsIndex { index, size }),
			
			Some(end_pointer) => end_pointer,
		};
		
		if unlikely!(end_pointer > file_length)
		{
			return Err(PointerOverflowsFileLength { index, size, file_length })
		}
		
		Ok(())
	}
}