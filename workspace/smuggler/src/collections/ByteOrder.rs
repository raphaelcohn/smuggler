// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(u8)]
pub(crate) enum ByteOrder
{
	/// Known as 'Intel' in EXIF.
	LittleEndian,
	
	/// Known as 'Motorola' in EXIF.
	BigEndian,
}

impl Default for ByteOrder
{
	#[inline(always)]
	fn default() -> Self
	{
		use ByteOrder::*;
		Self::target_endian(|| LittleEndian, || BigEndian)
	}
}

impl ByteOrder
{
	#[inline(always)]
	pub(crate) fn byte_swap(self, slice: &mut [impl byte_swap::Unaligned])
	{
		use ByteOrder::*;
		
		match self
		{
			LittleEndian => slice.byte_swap_from_little_endian_to_native_endian(),
			
			BigEndian => slice.byte_swap_from_big_endian_to_native_endian(),
		}
	}
	
	#[inline(always)]
	fn target_endian<R>(little_endian: impl FnOnce() -> R, big_endian: impl FnOnce() -> R) -> R
	{
		if cfg!(target_endian = "little")
		{
			little_endian()
		}
		else if cfg!(target_endian = "big")
		{
			big_endian()
		}
		else
		{
			panic!("Unknown target_endian")
		}
	}
}
