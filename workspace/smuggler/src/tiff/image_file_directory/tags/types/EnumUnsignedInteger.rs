// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct EnumUnsignedInteger<UINT: UnsignedIntegerNormalizedType, UE: UnsignedEnum>(UnsignedInteger<UINT>, PhantomData<UE>);

impl<UINT: UnsignedIntegerNormalizedType, UE: UnsignedEnum> From<UnsignedIntegerValue> for EnumUnsignedInteger<UINT, UE>
{
	#[inline(always)]
	fn from(value: UnsignedIntegerValue) -> Self
	{
		Self(UnsignedInteger::from(value), PhantomData)
	}
}

impl<UINT: UnsignedIntegerNormalizedType, UE: UnsignedEnum> TryInto<UE> for EnumUnsignedInteger<UINT, UE>
{
	type Error = UnsignedIntegerValue;
	
	#[inline(always)]
	fn try_into(self) -> Result<UE, Self::Eror>
	{
		use UnsignedIntegerValue::*;
		
		match (self.0).0
		{
			U0 => UE::try_from_u0().map_err(|_| U0),
			
			U8(value) => UE::try_from_u8(value).map_err(U8),
			
			U16(value) => UE::try_from_u16(value).map_err(U16),
			
			U32(value) => UE::try_from_u32(value).map_err(U32),
			
			U64(value) => UE::try_from_u64(value).map_err(U64),
		}
	}
}
