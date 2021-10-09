// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


/// Can be unaligned.
pub trait CanBeUnaligned: Default + Debug + Copy + PartialEq + PartialOrd
{
	#[doc(hidden)]
	type U: byte_swap::Unaligned;
	
	#[doc(hidden)]
	const FieldCount: NonZeroUsize = new_non_zero_usize(1);
	
	#[doc(hidden)]
	#[inline(always)]
	fn slice_unaligned_and_byte_swap_as_appropriate<'a>(count: u64, byte_order: ByteOrder, slice: NonNull<[u8]>) -> &'a [Unaligned<Self>]
	{
		debug_assert_eq!(slice.len() % size_of::<Self>(), 0);
		debug_assert_eq!((slice.len() / size_of::<Self>()) as u64, count);
		
		let count = count as usize;
		
		let slice = unsafe { from_raw_parts_mut(slice.as_mut_ptr() as *mut Self::U, count * Self::FieldCount.get()) };
		byte_order.byte_swap(slice);
		unsafe { from_raw_parts(slice.as_mut_ptr() as *const Unaligned<Self>, count) }
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn read_unaligned_and_byte_swap_as_appropriate(this: NonNull<Self>, byte_order: ByteOrder) -> Self
	{
		ByteOrder::target_endian
		(
			|| Self::read_unaligned_if_native_endian_is_little_endian(this, byte_order),
			
			|| Self::read_unaligned_if_native_endian_is_big_endian(this, byte_order)
		)
	}
	
	#[doc(hidden)]
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
	
	#[doc(hidden)]
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
	
	#[doc(hidden)]
	fn read_unaligned_byte_swapped(this: NonNull<Self>) -> Self;
}

impl CanBeUnaligned for u16
{
	type U = Unaligned16;
	
	#[inline(always)]
	fn read_unaligned_byte_swapped(this: NonNull<Self>) -> Self
	{
		Unaligned16::read_unaligned_byte_swapped(this)
	}
}

impl CanBeUnaligned for u32
{
	type U = Unaligned32;
	
	#[inline(always)]
	fn read_unaligned_byte_swapped(this: NonNull<Self>) -> Self
	{
		Unaligned32::read_unaligned_byte_swapped(this)
	}
}

impl CanBeUnaligned for u64
{
	type U = Unaligned64;
	
	#[inline(always)]
	fn read_unaligned_byte_swapped(this: NonNull<Self>) -> Self
	{
		Unaligned64::read_unaligned_byte_swapped(this)
	}
}

impl CanBeUnaligned for i16
{
	type U = Unaligned16;
	
	#[inline(always)]
	fn read_unaligned_byte_swapped(this: NonNull<Self>) -> Self
	{
		let unsigned = Unaligned16::read_unaligned_byte_swapped(this.cast());
		unsafe { transmute(unsigned) }
	}
}

impl CanBeUnaligned for i32
{
	type U = Unaligned32;
	
	#[inline(always)]
	fn read_unaligned_byte_swapped(this: NonNull<Self>) -> Self
	{
		let unsigned = Unaligned32::read_unaligned_byte_swapped(this.cast());
		unsafe { transmute(unsigned) }
	}
}

impl CanBeUnaligned for i64
{
	type U = Unaligned64;
	
	#[inline(always)]
	fn read_unaligned_byte_swapped(this: NonNull<Self>) -> Self
	{
		let unsigned = Unaligned64::read_unaligned_byte_swapped(this.cast());
		unsafe { transmute(unsigned) }
	}
}

impl CanBeUnaligned for f32
{
	type U = Unaligned32;
	
	#[inline(always)]
	fn read_unaligned_byte_swapped(this: NonNull<Self>) -> Self
	{
		let unsigned = Unaligned32::read_unaligned_byte_swapped(this.cast());
		f32::from_bits(unsigned)
	}
}

impl CanBeUnaligned for f64
{
	type U = Unaligned64;
	
	#[inline(always)]
	fn read_unaligned_byte_swapped(this: NonNull<Self>) -> Self
	{
		let unsigned = Unaligned64::read_unaligned_byte_swapped(this.cast());
		f64::from_bits(unsigned)
	}
}
