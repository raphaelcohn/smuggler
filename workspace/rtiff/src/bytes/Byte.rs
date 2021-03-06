// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


/// A byte such as u8 or i8.
pub trait Byte: Default + Debug + Copy + Ord + Eq + Hash
{
	#[doc(hidden)]
	#[inline(always)]
	fn byte_slice<'tiff_bytes>(slice: NonNull<[u8]>) -> &'tiff_bytes [Self]
	{
		unsafe { from_raw_parts(slice.as_mut_ptr() as *const Self, slice.len()) }
	}
}

impl Byte for u8
{
}

impl Byte for i8
{
}
