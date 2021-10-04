// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


/// An enumerated value.
pub trait UnsignedEnum: Sized + Copy + Eq + Ord + Hash
{
	/// Constructs an instance or returns an error if the value is unrecognized.
	#[inline(always)]
	fn try_from_u0() -> Result<Self, u64>
	{
		Self::try_from_u64(0)
	}
	
	/// Constructs an instance or returns the value if the value is unrecognized.
	#[inline(always)]
	fn try_from_u8(value: u8) -> Result<Self, u64>
	{
		Self::try_from_u64(value as u64)
	}
	
	/// Constructs an instance or returns the value if the value is unrecognized.
	#[inline(always)]
	fn try_from_u16(value: u16) -> Result<Self, u64>
	{
		Self::try_from_u64(value as u64)
	}
	
	/// Constructs an instance or returns the value if the value is unrecognized.
	#[inline(always)]
	fn try_from_u32(value: u32) -> Result<Self, u64>
	{
		Self::try_from_u64(value as u64)
	}
	
	/// Constructs an instance or returns the value if the value is unrecognized.
	fn try_from_u64(value: u64) -> Result<Self, u64>;
}
