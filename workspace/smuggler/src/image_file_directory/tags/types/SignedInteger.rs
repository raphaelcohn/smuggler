// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


/// Signed integer.
#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SignedInteger<SINT: SignedIntegerNormalizedType>(SignedIntegerValue, PhantomData<SINT>);

impl Default for SignedInteger<i8>
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::from(SignedIntegerValue::I8(0))
	}
}

impl Default for SignedInteger<i16>
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::from(SignedIntegerValue::I16(0))
	}
}

impl Default for SignedInteger<i32>
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::from(SignedIntegerValue::I32(0))
	}
}

impl Default for SignedInteger<i64>
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::from(SignedIntegerValue::I64(0))
	}
}

impl<SINT: SignedIntegerNormalizedType> From<SignedIntegerValue> for SignedInteger<SINT>
{
	#[inline(always)]
	fn from(value: SignedIntegerValue) -> Self
	{
		Self(value, PhantomData)
	}
}
