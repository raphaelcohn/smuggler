// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct EnumSignedInteger<SINT: SignedIntegerNormalizedType, SE: SignedEnum>(SignedInteger<SINT>, PhantomData<SE>);

impl<SINT: SignedIntegerNormalizedType, SE: SignedEnum> From<SignedIntegerValue> for EnumSignedInteger<SINT, SE>
{
	#[inline(always)]
	fn from(value: SignedIntegerValue) -> Self
	{
		Self(SignedInteger::from(value), PhantomData)
	}
}

impl<SINT: SignedIntegerNormalizedType, SE: SignedEnum> EnumSignedInteger<SINT, SE>
{
	#[inline(always)]
	fn try_into(self) -> Result<SE, SignedIntegerValue>
	{
		use SignedIntegerValue::*;
		
		let signed_integer = self.0;
		match signed_integer.0
		{
			I8(value) => SE::try_from_i8(value).map_err(I8),
			
			I16(value) => SE::try_from_i16(value).map_err(I16),
			
			I32(value) => SE::try_from_i32(value).map_err(I32),
			
			I64(value) => SE::try_from_i64(value).map_err(I64),
		}
	}
}
