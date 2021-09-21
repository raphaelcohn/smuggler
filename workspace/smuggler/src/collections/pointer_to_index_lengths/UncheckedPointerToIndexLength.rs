// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub(super) struct UncheckedPointerToIndexLength;

impl PointerToIndexLength for UncheckedPointerToIndexLength
{
	type CheckOutcome = ();
	
	#[inline(always)]
	fn check_inner(file_length: FileLength, index: u64, size: u64) -> Self::CheckOutcome
	{
		if cfg!(debug_assertions)
		{
			let end_pointer = index.checked_add(size).unwrap();
			debug_assert!(end_pointer <= file_length);
		}
		()
	}
}
