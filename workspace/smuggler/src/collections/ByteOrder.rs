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

macro_rules! to_native_endian
{
	($self: ident, $value: ident) =>
	{
		{
			use ByteOrder::*;
			
			let value = $value;
			
			ByteOrder::target_endian
			(
				||
				{
					match $self
					{
						LittleEndian => value,
						
						BigEndian => value.swap_bytes(),
					}
				},
				||
				{
					match $self
					{
						LittleEndian => value.swap_bytes(),
						
						BigEndian => value,
					}
				}
			)
		}
	}
}

macro_rules! read_value
{
	($self: ident, $pointer_to_index: ident, $read: ident) =>
	{
		{
			use ByteOrder::*;
			
			let value = unsafe { $pointer_to_index.$read() };
			to_native_endian!($self, value)
		}
	}
}

impl ByteOrder
{
	#[inline(always)]
	pub(crate) fn read_unaligned_u16(self, pointer_to_index: *const u16) -> u16
	{
		read_value!(self, pointer_to_index)
	}
	
	#[inline(always)]
	pub(crate) fn read_unaligned_u32(self, pointer_to_index: *const u32) -> u32
	{
		read_value!(self, pointer_to_index)
	}
	
	#[inline(always)]
	pub(crate) fn read_unaligned_u64(self, pointer_to_index: *const u64) -> u64
	{
		read_value!(self, pointer_to_index)
	}
	
	#[inline(always)]
	pub(crate) fn read_unaligned_i16(self, pointer_to_index: *const i16) -> i16
	{
		read_value!(self, pointer_to_index)
	}
	
	#[inline(always)]
	pub(crate) fn read_unaligned_i32(self, pointer_to_index: *const i32) -> i32
	{
		read_value!(self, pointer_to_index)
	}
	
	#[inline(always)]
	pub(crate) fn read_unaligned_i64(self, pointer_to_index: *const i64) -> i64
	{
		read_value!(self, pointer_to_index)
	}
	
	#[inline(always)]
	pub(crate) fn read_unaligned_f32(self, pointer_to_index: *const f32) -> f32
	{
		let bits = self.read_unaligned_u32(pointer_to_index as *const u32);
		f32::from_bits(bits)
	}
	
	#[inline(always)]
	pub(crate) fn read_unaligned_f64(self, pointer_to_index: *const f64) -> f64
	{
		let bits = self.read_unaligned_u64(pointer_to_index as *const u64);
		f64::from_bits(bits)
	}
	
	#[inline(always)]
	pub(crate) fn read_unaligned_rational_fraction<RFA: RationalFractionAtor>(self, pointer_to_index: *const RationalFraction<RFA>) -> RationalFraction<RFA>
	{
		let pointer_to_index = pointer_to_index as *const RFA;
		let numerator = RFA::read_unaligned(self, pointer_to_index);
		let denominator = RFA::read_unaligned(self, unsafe { pointer_to_index.add(1) });
		RationalFraction::new(numerator, denominator)
	}
	
	#[inline(always)]
	pub(crate) fn byte_swap(self, slice: &mut [impl Unaligned])
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
