// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


pub(crate) trait NonNullExt<T: Sized>
{
	fn read(self) -> T;
	
	fn read_unaligned(self) -> T;
}

impl<T: Sized> NonNullExt<T> for NonNull<T>
{
	#[inline(always)]
	fn read(self) -> T
	{
		unsafe { self.as_ptr().read() }
	}
	
	#[inline(always)]
	fn read_unaligned(self) -> T
	{
		unsafe { self.as_ptr().read_unaligned() }
	}
}
