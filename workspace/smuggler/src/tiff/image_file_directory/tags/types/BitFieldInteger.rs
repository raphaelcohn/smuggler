// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct BitFieldInteger<UINT: UnsignedIntegerNormalizedType, BF: BitField>(UnsignedInteger<UINT>, PhantomData<BF>);

impl<UINT: UnsignedIntegerNormalizedType, BF: BitField> TryInto<BF> for BitFieldInteger<UINT, BF>
{
	type Error = (BF, UnsignedIntegerValue);
	
	#[inline(always)]
	fn try_into(self) -> Result<BF, Self::Error>
	{
		use UnsignedIntegerValue::*;
		
		let unsigned_integer = self.0;
		match unsigned_integer.0
		{
			U0 => Ok(BF::try_from_u0()),
			
			U8(bits) => BF::try_from_u8(bits).map_err(|(bit_field, unrecognized_bits)| (bit_field, U8(unrecognized_bits as u8))),
			
			U16(bits) => BF::try_from_u16(bits).map_err(|(bit_field, unrecognized_bits)| (bit_field, U16(unrecognized_bits as u16))),
			
			U32(bits) => BF::try_from_u32(bits).map_err(|(bit_field, unrecognized_bits)| (bit_field, U32(unrecognized_bits as u32))),
			
			U64(bits) => BF::try_from_u64(bits).map_err(|(bit_field, unrecognized_bits)| (bit_field, U64(unrecognized_bits))),
		}
	}
}

impl<UINT: UnsignedIntegerNormalizedType, BF: BitField> From<UnsignedIntegerValue> for BitFieldInteger<UINT, BF>
{
	#[inline(always)]
	fn from(value: UnsignedIntegerValue) -> Self
	{
		Self(UnsignedInteger::from(value), PhantomData)
	}
}
