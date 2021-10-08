// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


/// A bit field.
pub trait BitField: Default + Debug + Copy + Eq + Ord + Hash + Into<u64>
{
	type Bit: EnumCount + IntoEnumIterator + Into<&'static str> + Into<u8> + ToString + FromStr;
	
	#[inline(always)]
	fn has_bit(self, bit: Self::Bit) -> bool
	{
		let bit_value: u8 = bit.into();
		debug_assert!(bit_value < 64);
		
		let value = self.into();
		value & (1 << (bit_value as u64)) != 0
	}
	
	/// Constructs an instance by converting bits, returning a valid bit field and (just) the bits that were unrecognized.
	#[inline(always)]
	fn try_from_u8(bits: u8) -> Result<Self, (Self, u64)>
	{
		Self::try_from_u64(bits as u64)
	}
	
	/// Constructs an instance by converting bits, returning a valid bit field and (just) the bits that were unrecognized.
	#[inline(always)]
	fn try_from_u16(bits: u16) -> Result<Self, (Self, u64)>
	{
		Self::try_from_u64(bits as u64)
	}
	
	/// Constructs an instance by converting bits, returning a valid bit field and (just) the bits that were unrecognized.
	#[inline(always)]
	fn try_from_u32(bits: u32) -> Result<Self, (Self, u64)>
	{
		Self::try_from_u64(bits as u64)
	}
	
	/// Constructs an instance by converting bits, returning a valid bit field and (just) the bits that were unrecognized.
	fn try_from_u64(bits: u64) -> Result<Self, (Self, u64)>;
}
