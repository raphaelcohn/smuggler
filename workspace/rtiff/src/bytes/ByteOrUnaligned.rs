// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


pub(crate) trait ByteOrUnaligned: Sized
{
	fn read_unsigned_integer_value(slice: &[Self], index: usize) -> u64;
}

impl<B: Byte + Into<u64>> ByteOrUnaligned for B
{
	#[inline(always)]
	fn read_unsigned_integer_value(slice: &[Self], index: usize) -> u64
	{
		slice.get_unchecked_value_safe(index).into()
	}
}

impl<CBU: CanBeUnaligned + Into<u64>> ByteOrUnaligned for Unaligned<CBU>
{
	#[inline(always)]
	fn read_unsigned_integer_value(slice: &[Self], index: usize) -> u64
	{
		let unaligned = slice.get_unchecked_safe(index);
		unaligned.read_assuming_is_native_endian().into()
	}
}
