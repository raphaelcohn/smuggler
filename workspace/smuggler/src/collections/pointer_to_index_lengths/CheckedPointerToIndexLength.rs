// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub(super) struct CheckedPointerToIndexLength;

impl PointerToIndexLength for CheckedPointerToIndexLength
{
	type CheckOutcome = Result<(), OverflowError>;
	
	#[inline(always)]
	fn check_inner(index: Index, size_in_bytes: u64, file_length: FileLength) -> Self::CheckOutcome
	{
		use OverflowError::*;
		
		let end_pointer = match index.checked_add(size_in_bytes)
		{
			None => return Err(SizeOverflowsIndex { index, size_in_bytes }),
			
			Some(end_pointer) => end_pointer,
		};
		
		if unlikely!(end_pointer > file_length)
		{
			return Err(PointerOverflowsFileLength { index, size_in_bytes, file_length })
		}
		
		#[cfg(not(target_pointer_width = "64"))]
		{
			const Maximum: u64 = usize::MAX as u64;
			if unlikely!(index > Maximum)
			{
				return Err(IndexExceedsUsize { index })
			}
			
			if unlikely!(size_in_bytes > Maximum)
			{
				return Err(SizeExceedsUsize { size_in_bytes })
			}
			
			const MaximumPlusOne: u64 = Maximum + 1;
			if unlikely!(end_pointer > Maximum)
			{
				return Err(EndPointerExceedsUsizePlusOne { index, size_in_bytes })
			}
		}
		
		Ok(())
	}
}
