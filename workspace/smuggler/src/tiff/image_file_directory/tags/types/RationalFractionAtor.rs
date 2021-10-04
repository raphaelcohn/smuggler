// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


/// A rational fraction 'ator': a numerator or a denominator.
pub trait RationalFractionAtor: Default + Debug + Copy + Eq + Ord + Hash
{
	fn read_unaligned(byte_order: ByteOrder, pointer_to_index: *const Self) -> Self;
}

impl RationalFractionAtor for u32
{
	#[inline(always)]
	fn read_unaligned(byte_order: ByteOrder, pointer_to_index: *const Self) -> Self
	{
		byte_order.read_unaligned_u32(pointer_to_index)
	}
}

impl RationalFractionAtor for i32
{
	#[inline(always)]
	fn read_unaligned(byte_order: ByteOrder, pointer_to_index: *const Self) -> Self
	{
		byte_order.read_unaligned_i32(pointer_to_index)
	}
}
