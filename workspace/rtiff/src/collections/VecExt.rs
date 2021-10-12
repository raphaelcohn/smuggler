// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


/// A vector extension trait.
pub trait VecExt<T, A: Allocator>: Sized
{
	/// New with capacity.
	fn new_with_capacity<AUI: AsUsizeIndex>(length: AUI, allocator: A) -> Result<Self, TryReserveError>;
	
	/// New buffer.
	fn new_buffer<AUI: AsUsizeIndex>(length: AUI, allocator: A) -> Result<Self, TryReserveError>;
	
	/// New from values.
	fn new_from(values: &[T], allocator: A) -> Result<Self, TryReserveError> where T: Copy;
	
	/// Try to push.
	fn try_push(&mut self, value: T) -> Result<(), TryReserveError>;
	
	/// Push without checking capacity.
	fn push_unchecked(&mut self, value: T);
	
	/// Try to insert.
	fn try_insert_with_optimization_bias_to_push<AUI: AsUsizeIndex>(&mut self, index: AUI, value: T) -> Result<(), TryReserveError>;
}

impl<T, A: Allocator> VecExt<T, A> for Vec<T, A>
{
	#[inline(always)]
	fn new_with_capacity<AUI: AsUsizeIndex>(length: AUI, allocator: A) -> Result<Self, TryReserveError>
	{
		let mut buffer = Vec::new_in(allocator);
		buffer.try_reserve_exact(length.as_usize())?;
		Ok(buffer)
	}
	
	#[inline(always)]
	fn new_buffer<AUI: AsUsizeIndex>(length: AUI, allocator: A) -> Result<Self, TryReserveError>
	{
		let length = length.as_usize();
		let mut buffer = Self::new_with_capacity(length, allocator)?;
		unsafe { buffer.set_len(length) };
		Ok(buffer)
	}
	
	#[inline(always)]
	fn new_from(values: &[T], allocator: A) -> Result<Self, TryReserveError> where T: Copy
	{
		let length = values.len();
		let mut this = Self::new_buffer(length, allocator)?;
		unsafe { this.as_mut_ptr().copy_from_nonoverlapping(values.as_ptr(), length) };
		Ok(this)
	}
	
	#[inline(always)]
	fn try_push(&mut self, value: T) -> Result<(), TryReserveError>
	{
		self.try_reserve(1)?;
		self.push_unchecked(value);
		Ok(())
	}
	
	#[inline(always)]
	fn push_unchecked(&mut self, value: T)
	{
		let length = self.len();
		unsafe
		{
			let end = self.as_mut_ptr().add(length);
			ptr::write(end, value);
			self.set_len(length + 1);
		}
	}
	
	#[inline(always)]
	fn try_insert_with_optimization_bias_to_push<AUI: AsUsizeIndex>(&mut self, index: AUI, value: T) -> Result<(), TryReserveError>
	{
		self.try_reserve(1)?;
		
		let index = index.as_usize();
		let length = self.len();
		debug_assert!(index <= length);
		
		let pointer = self.as_mut_ptr();
		let insert_at_pointer = unsafe { pointer.add(index) };
		
		// Whilst `ptr::copy()` handles a `tail_length` of `0` perfectly, it still involves, in assembly, a function call.
		// The majority of the time, especially for free space handling, insertions will always occur at the end of the `Vec`.
		if unlikely!(index != length)
		{
			let tail_length = length - index;
			unsafe
			{
				let move_tail_to_pointer = insert_at_pointer.add(1);
				ptr::copy(insert_at_pointer, move_tail_to_pointer, tail_length);
			}
		}
		
		unsafe
		{
			ptr::write(insert_at_pointer, value);
			self.set_len(length + 1)
		}
		
		Ok(())
	}
}
