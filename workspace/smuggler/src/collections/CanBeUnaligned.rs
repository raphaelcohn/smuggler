// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


/// Can be unaligned.
pub trait CanBeUnaligned: Default + Debug + Copy + PartialEq + PartialOrd
{
	#[inline(always)]
	fn read_unaligned_and_byte_swap_as_appropriate(this: NonNull<Self>, byte_order: ByteOrder) -> Self
	{
		use ByteOrder::*;
		
		ByteOrder::target_endian
		(
			Self::read_unaligned_if_native_endian_is_little_endian(this, byte_order),
			
			Self::read_unaligned_if_native_endian_is_big_endian(this, byte_order)
		)
	}
	
	#[inline(always)]
	fn read_unaligned_if_native_endian_is_little_endian(this: NonNull<Self>, byte_order: ByteOrder) -> Self
	{
		use ByteOrder::*;
		
		match byte_order
		{
			LittleEndian => this.read_unaligned(),
			
			BigEndian => Self::read_unaligned_byte_swapped(this),
		}
	}
	
	#[inline(always)]
	fn read_unaligned_if_native_endian_is_big_endian(this: NonNull<Self>, byte_order: ByteOrder) -> Self
	{
		use ByteOrder::*;
		
		match byte_order
		{
			LittleEndian => Self::read_unaligned_byte_swapped(this),
			
			BigEndian => this.read_unaligned(),
		}
	}
	
	fn read_unaligned_byte_swapped(this: NonNull<Self>) -> Self;
}

impl CanBeUnaligned for u16
{
	#[inline(always)]
	fn read_unaligned_byte_swapped(this: NonNull<Self>) -> Self
	{
		Unaligned16::read_unaligned_byte_swapped(this)
	}
}

impl CanBeUnaligned for u32
{
	#[inline(always)]
	fn read_unaligned_byte_swapped(this: NonNull<Self>) -> Self
	{
		Unaligned32::read_unaligned_byte_swapped(this)
	}
}

impl CanBeUnaligned for u64
{
	#[inline(always)]
	fn read_unaligned_byte_swapped(this: NonNull<Self>) -> Self
	{
		Unaligned64::read_unaligned_byte_swapped(this)
	}
}

impl CanBeUnaligned for i16
{
	#[inline(always)]
	fn read_unaligned_byte_swapped(this: NonNull<Self>) -> Self
	{
		let unsigned = Unaligned16::read_unaligned_byte_swapped(this.cast());
		unsafe { transmute(unsigned) }
	}
}

impl CanBeUnaligned for i32
{
	#[inline(always)]
	fn read_unaligned_byte_swapped(this: NonNull<Self>) -> Self
	{
		let unsigned = Unaligned32::read_unaligned_byte_swapped(this.cast());
		unsafe { transmute(unsigned) }
	}
}

impl CanBeUnaligned for i64
{
	#[inline(always)]
	fn read_unaligned_byte_swapped(this: NonNull<Self>) -> Self
	{
		let unsigned = Unaligned64::read_unaligned_byte_swapped(this.cast());
		unsafe { transmute(unsigned) }
	}
}

impl CanBeUnaligned for f32
{
	#[inline(always)]
	fn read_unaligned_byte_swapped(this: NonNull<Self>) -> Self
	{
		let unsigned = Unaligned32::read_unaligned_byte_swapped(this.cast());
		f32::from_bits(unsigned)
	}
}

impl CanBeUnaligned for f64
{
	#[inline(always)]
	fn read_unaligned_byte_swapped(this: NonNull<Self>) -> Self
	{
		let unsigned = Unaligned64::read_unaligned_byte_swapped(this.cast());
		f64::from_bits(unsigned)
	}
}
