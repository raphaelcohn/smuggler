// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


#[allow(missing_docs)]
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

impl<UINT: UnsignedIntegerNormalizedType, UE: UnsignedEnum> EnumUnsignedInteger<UINT, UE>
{
	#[allow(missing_docs)]
	#[inline(always)]
	pub fn try_into(self) -> Result<UE, UnsignedIntegerValue>
	{
		use UnsignedIntegerValue::*;
		
		match (self.0).0
		{
			U8(value) => UE::try_from_u8(value).map_err(|u64| U8(u64 as u8)),
			
			U16(value) => UE::try_from_u16(value).map_err(|u64| U16(u64 as u16)),
			
			U32(value) => UE::try_from_u32(value).map_err(|u64| U32(u64 as u32)),
			
			U64(value) => UE::try_from_u64(value).map_err(U64),
		}
	}
}
