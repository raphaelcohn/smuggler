// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


/// Signed integers.
#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SignedIntegers<'tiff_bytes, SINT: SignedIntegerNormalizedType>(SignedIntegerValues<'tiff_bytes>, PhantomData<SINT>);

impl<'tiff_bytes, SINT: SignedIntegerNormalizedType> From<SignedIntegerValues<'tiff_bytes>> for SignedIntegers<'tiff_bytes, SINT>
{
	#[inline(always)]
	fn from(value: SignedIntegerValues<'tiff_bytes>) -> Self
	{
		Self(value, PhantomData)
	}
}

impl<'tiff_bytes, SINT: SignedIntegerNormalizedType> Into<SignedIntegerValues<'tiff_bytes>> for SignedIntegers<'tiff_bytes, SINT>
{
	#[inline(always)]
	fn into(self) -> SignedIntegerValues<'tiff_bytes>
	{
		self.0
	}
}
