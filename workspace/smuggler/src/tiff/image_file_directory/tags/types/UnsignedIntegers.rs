// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


/// Unsigned integers.
#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct UnsignedIntegers<'tiff_bytes, UINT: UnsignedIntegerNormalizedType>(UnsignedIntegerValues<'tiff_bytes>, PhantomData<UINT>);

impl<'tiff_bytes, UINT: UnsignedIntegerNormalizedType> From<UnsignedIntegerValues> for UnsignedIntegers<UINT>
{
	#[inline(always)]
	fn from(value: UnsignedIntegerValues<'tiff_bytes>) -> Self
	{
		Self(value, PhantomData)
	}
}
