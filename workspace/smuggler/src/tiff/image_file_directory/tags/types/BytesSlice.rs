// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


pub(super) trait BytesSlice: TiffBytes
{
	#[inline(always)]
	fn short_slice<Unit: Version6OrBigTiffUnit>(&mut self, count: u64, offset_or_value_union_index: Index, byte_order: ByteOrder, recursion_guard: &RecursionGuard) -> Result<&[Unaligned<u16>], SpecificTagParseError>
	{
		unsafe { transmute(self.byte_swapped_slice::<Unaligned16>(count, offset_or_value_union_index, Unit::Count16, byte_order, recursion_guard)?) }
	}
	
	#[inline(always)]
	fn long_slice<Unit: Version6OrBigTiffUnit>(&mut self, count: u64, offset_or_value_union_index: Index, byte_order: ByteOrder, recursion_guard: &RecursionGuard) -> Result<&[Unaligned<u32>], SpecificTagParseError>
	{
		unsafe { transmute(self.byte_swapped_slice::<Unaligned32>(count, offset_or_value_union_index, Unit::Count32, byte_order, recursion_guard)?) }
	}
	
	#[inline(always)]
	fn rational_slice<Unit: Version6OrBigTiffUnit>(&mut self, count: u64, offset_or_value_union_index: Index, byte_order: ByteOrder, recursion_guard: &RecursionGuard) -> Result<&[Unaligned<RationalFraction<u32>>], SpecificTagParseError>
	{
		self.rational_slice_internal::<Unit, u32>(count, offset_or_value_union_index, byte_order, recursion_guard)
	}
	
	#[inline(always)]
	fn sshort_slice<Unit: Version6OrBigTiffUnit>(&mut self, count: u64, offset_or_value_union_index: Index, byte_order: ByteOrder, recursion_guard: &RecursionGuard) -> Result<&[Unaligned<i16>], SpecificTagParseError>
	{
		unsafe { transmute(self.byte_swapped_slice::<Unaligned16>(count, offset_or_value_union_index, Unit::Count16, byte_order, recursion_guard)?) }
	}
	
	#[inline(always)]
	fn slong_slice<Unit: Version6OrBigTiffUnit>(&mut self, count: u64, offset_or_value_union_index: Index, byte_order: ByteOrder, recursion_guard: &RecursionGuard) -> Result<&[Unaligned<i32>], SpecificTagParseError>
	{
		unsafe { transmute(self.byte_swapped_slice::<Unaligned32>(count, offset_or_value_union_index, Unit::Count32, byte_order, recursion_guard)?) }
	}
	
	#[inline(always)]
	fn srational_slice<Unit: Version6OrBigTiffUnit>(&mut self, count: u64, offset_or_value_union_index: Index, byte_order: ByteOrder, recursion_guard: &RecursionGuard) -> Result<&[Unaligned<RationalFraction<i32>>], SpecificTagParseError>
	{
		self.rational_slice_internal::<Unit, i32>(count, offset_or_value_union_index, byte_order, recursion_guard)
	}
	
	#[inline(always)]
	fn float_slice<Unit: Version6OrBigTiffUnit>(&mut self, count: u64, offset_or_value_union_index: Index, byte_order: ByteOrder, recursion_guard: &RecursionGuard) -> Result<&[Unaligned<f32>], SpecificTagParseError>
	{
		unsafe { transmute(self.byte_swapped_slice::<Unaligned32>(count, offset_or_value_union_index, Unit::Count32, byte_order, recursion_guard)?) }
	}
	
	#[inline(always)]
	fn double_slice<Unit: Version6OrBigTiffUnit>(&mut self, count: u64, offset_or_value_union_index: Index, byte_order: ByteOrder, recursion_guard: &RecursionGuard) -> Result<&[Unaligned<f64>], SpecificTagParseError>
	{
		unsafe { transmute(self.byte_swapped_slice::<Unaligned64>(count, offset_or_value_union_index, Unit::Count64, byte_order, recursion_guard)?) }
	}
	
	#[inline(always)]
	fn long8_slice<Unit: Version6OrBigTiffUnit>(&mut self, count: u64, offset_or_value_union_index: Index, byte_order: ByteOrder, recursion_guard: &RecursionGuard) -> Result<&[Unaligned<u64>], SpecificTagParseError>
	{
		unsafe { transmute(self.byte_swapped_slice::<Unaligned64>(count, offset_or_value_union_index, Unit::Count64, byte_order, recursion_guard)?) }
	}
	
	#[inline(always)]
	fn slong8_slice<Unit: Version6OrBigTiffUnit>(&mut self, count: u64, offset_or_value_union_index: Index, byte_order: ByteOrder, recursion_guard: &RecursionGuard) -> Result<&[Unaligned<i64>], SpecificTagParseError>
	{
		unsafe { transmute(self.byte_swapped_slice::<Unaligned64>(count, offset_or_value_union_index, Unit::Count64, byte_order, recursion_guard)?) }
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn rational_slice_internal<Unit: Version6OrBigTiffUnit, RFA: RationalFractionAtor>(&mut self, count: u64, offset_or_value_union_index: Index, byte_order: ByteOrder, recursion_guard: &RecursionGuard) -> Result<&[Unaligned<RationalFraction<RFA>>], SpecificTagParseError>
	{
		let rational_fraction_slice = self.slice_from_offset_or_value::<Unaligned<RationalFraction<u32>>>(count, offset_or_value_union_index, Unit::Count64, recursion_guard)?;
		
		{
			let doubled_count = count.checked_mul(2).ok_or(SpecificTagParseError::CountIsTooLargeForTargetArchitecture)?;
			Self::guard_count_fits_in_usize(doubled_count)?;
			let unaligned32_slice = unsafe { from_raw_parts_mut(rational_fraction_slice.as_mut_ptr() as *mut Unaligned32, doubled_count as usize) };
			byte_order.byte_swap(unaligned32_slice);
		}
		
		Ok(rational_fraction_slice)
	}
	
	// `Count` should be a generic const but Rust does not like that it is a const member of the Unit trait.
	#[doc(hidden)]
	#[inline(always)]
	fn byte_swapped_slice<U: byte_swap::Unaligned>(&mut self, count: u64, offset_or_value_union_index: Index, Count: u64, byte_order: ByteOrder, recursion_guard: &RecursionGuard) -> Result<&[U], SpecificTagParseError>
	{
		let slice = self.slice_from_offset_or_value::<U>(count, offset_or_value_union_index, Count, recursion_guard)?;
		byte_order.byte_swap(slice);
		Ok(slice)
	}
	
	#[inline(always)]
	fn guard_count_fits_in_usize(count: u64) -> Result<(), SpecificTagParseError>
	{
		if cfg!(not(target_pointer_width = "64"))
		{
			if unlikely!(count > (usize::MAX as u64))
			{
				return Err(SpecificTagParseError::CountIsTooLargeForTargetArchitecture)
			}
		}
		Ok(())
	}
}

impl<TB: TiffBytes> BytesSlice for TB
{
}
