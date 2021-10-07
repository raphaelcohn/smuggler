// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


/// An enumerated value.
pub trait SignedEnum: Sized + Copy + Eq + Ord + Hash
{
	/// Constructs an instance or returns an error if the value is unrecognized.
	#[inline(always)]
	fn try_from_i8(value: i8) -> Result<Self, i64>
	{
		Self::try_from_i64(value as i64)
	}
	
	/// Constructs an instance or returns an error if the value is unrecognized.
	#[inline(always)]
	fn try_from_i16(value: i16) -> Result<Self, i64>
	{
		Self::try_from_i64(value as i64)
	}
	
	/// Constructs an instance or returns an error if the value is unrecognized.
	#[inline(always)]
	fn try_from_i32(value: i32) -> Result<Self, i64>
	{
		Self::try_from_i64(value as i64)
	}
	
	/// Constructs an instance or returns an error if the value is unrecognized.
	fn try_from_i64(value: i64) -> Result<Self, i64>;
}
