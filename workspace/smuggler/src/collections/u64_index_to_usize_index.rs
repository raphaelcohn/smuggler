// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


#[inline(always)]
fn u64_index_to_usize_index(value: u64) -> usize
{
	if cfg!(target_pointer_width = "64")
	{
		value as usize
	}
	else
	{
		value.try_into().expect("Can not handle indices larger than pointer width on a 32-bit or 16-bit CPU")
	}
}
