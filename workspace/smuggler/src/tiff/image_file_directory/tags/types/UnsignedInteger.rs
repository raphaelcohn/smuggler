// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


/// Unsigned integer.
#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct UnsignedInteger<UINT: UnsignedIntegerNormalizedType>(UnsignedIntegerValue, PhantomData<UINT>);

impl Default for UnsignedInteger<u8>
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::from(UnsignedIntegerValue::U8(0))
	}
}

impl Default for UnsignedInteger<u16>
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::from(UnsignedIntegerValue::U16(0))
	}
}

impl Default for UnsignedInteger<u32>
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::from(UnsignedIntegerValue::U32(0))
	}
}

impl Default for UnsignedInteger<u64>
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::from(UnsignedIntegerValue::U64(0))
	}
}

impl<UINT: UnsignedIntegerNormalizedType> From<UnsignedIntegerValue> for UnsignedInteger<UINT>
{
	#[inline(always)]
	fn from(value: UnsignedIntegerValue) -> Self
	{
		Self(value, PhantomData)
	}
}
