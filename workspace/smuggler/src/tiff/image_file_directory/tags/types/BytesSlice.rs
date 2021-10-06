// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


pub(super) trait BytesSlice: TiffBytes
{
	#[inline(always)]
	fn byte_slice<Unit: Version6OrBigTiffUnit>(&mut self, count: u64, offset_or_value_union_index: Index) -> Result<&[u8], SpecificTagParseError>
	{
		unsafe { transmute(self.slice_from_offset_or_value::<u8>(count, offset_or_value_union_index, Unit::Count8)?) }
	}
	
	#[inline(always)]
	fn ascii_strings<'a, Unit: Version6OrBigTiffUnit, A: Allocator>(&'a mut self, count: u64, offset_or_value_union_index: Index, allocator: A) -> Result<AsciiStrings<'a, A>, SpecificTagParseError>
	{
		#[inline(always)]
		fn u8_to_ascii(string: &[u8]) -> &[NonZeroU8]
		{
			unsafe { transmute(string) }
		}
		
		#[inline(always)]
		fn try_push_string<'a>(strings: &mut Vec<&'a [NonZeroU8]>, string: &'a [u8]) -> Result<(), SpecificTagParseError>
		{
			strings.try_push(u8_to_ascii(string)).map_err(SpecificTagParseError::CouldNotAllocateMemoryForAsciiStringReference)
		}
		
		let mut strings: Vec<&'a [NonZeroU8]> = Vec::new_in(allocator);
		let mut remaining_bytes = self.byte_slice::<Unit>(count, offset_or_value_union_index)?;
		loop
		{
			match memchr(0x00, remaining_bytes)
			{
				None =>
				{
					try_push_string(&mut strings, remaining_bytes)?;
					
					return Ok(AsciiStrings::new(strings, true))
				}
				
				Some(index) =>
				{
					try_push_string(&mut strings, remaining_bytes.get_unchecked_range_safe(.. index))?;
					
					let next = index + 1;
					if next == remaining_bytes.len()
					{
						return Ok(AsciiStrings::new(strings, false))
					}
					remaining_bytes = remaining_bytes.get_unchecked_range_safe(next ..);
				}
			}
		}
	}
	
	#[inline(always)]
	fn short_slice<Unit: Version6OrBigTiffUnit>(&mut self, count: u64, offset_or_value_union_index: Index, byte_order: ByteOrder) -> Result<&[Unaligned<u16>], SpecificTagParseError>
	{
		unsafe { transmute(self.byte_swapped_slice::<Unaligned16>(count, offset_or_value_union_index, Unit::Count16, byte_order)?) }
	}
	
	#[inline(always)]
	fn long_slice<Unit: Version6OrBigTiffUnit>(&mut self, count: u64, offset_or_value_union_index: Index, byte_order: ByteOrder) -> Result<&[Unaligned<u32>], SpecificTagParseError>
	{
		unsafe { transmute(self.byte_swapped_slice::<Unaligned32>(count, offset_or_value_union_index, Unit::Count32, byte_order)?) }
	}
	
	#[inline(always)]
	fn rational_slice<Unit: Version6OrBigTiffUnit>(&mut self, count: u64, offset_or_value_union_index: Index, byte_order: ByteOrder) -> Result<&[Unaligned<RationalFraction<u32>>], SpecificTagParseError>
	{
		self.rational_slice_internal::<Unit, u32>(count, offset_or_value_union_index, byte_order)
	}
	
	#[inline(always)]
	fn sbyte_slice<Unit: Version6OrBigTiffUnit>(&mut self, count: u64, offset_or_value_union_index: Index, byte_order: ByteOrder) -> Result<&[i8], SpecificTagParseError>
	{
		unsafe { transmute(self.slice_from_offset_or_value::<i8>(count, offset_or_value_union_index, Unit::Count8)?) }
	}
	
	#[inline(always)]
	fn undefined_slice<Unit: Version6OrBigTiffUnit>(&mut self, count: u64, offset_or_value_union_index: Index, byte_order: ByteOrder) -> Result<&[u8], SpecificTagParseError>
	{
		self.byte_slice(count, offset_or_value_union_index, byte_order)
	}
	
	#[inline(always)]
	fn sshort_slice<Unit: Version6OrBigTiffUnit>(&mut self, count: u64, offset_or_value_union_index: Index, byte_order: ByteOrder) -> Result<&[Unaligned<i16>], SpecificTagParseError>
	{
		unsafe { transmute(self.byte_swapped_slice::<Unaligned16>(count, offset_or_value_union_index, Unit::Count16, byte_order)?) }
	}
	
	#[inline(always)]
	fn slong_slice<Unit: Version6OrBigTiffUnit>(&mut self, count: u64, offset_or_value_union_index: Index, byte_order: ByteOrder) -> Result<&[Unaligned<i32>], SpecificTagParseError>
	{
		unsafe { transmute(self.byte_swapped_slice::<Unaligned32>(count, offset_or_value_union_index, Unit::Count32, byte_order)?) }
	}
	
	#[inline(always)]
	fn srational_slice<Unit: Version6OrBigTiffUnit>(&mut self, count: u64, offset_or_value_union_index: Index, byte_order: ByteOrder) -> Result<&[Unaligned<RationalFraction<i32>>], SpecificTagParseError>
	{
		self.rational_slice_internal::<Unit, i32>(count, offset_or_value_union_index, byte_order)
	}
	
	#[inline(always)]
	fn float_slice<Unit: Version6OrBigTiffUnit>(&mut self, count: u64, offset_or_value_union_index: Index, byte_order: ByteOrder) -> Result<&[Unaligned<f32>], SpecificTagParseError>
	{
		unsafe { transmute(self.byte_swapped_slice::<Unaligned32>(count, offset_or_value_union_index, Unit::Count32, byte_order)?) }
	}
	
	#[inline(always)]
	fn double_slice<Unit: Version6OrBigTiffUnit>(&mut self, count: u64, offset_or_value_union_index: Index, byte_order: ByteOrder) -> Result<&[Unaligned<f64>], SpecificTagParseError>
	{
		unsafe { transmute(self.byte_swapped_slice::<Unaligned64>(count, offset_or_value_union_index, Unit::Count64, byte_order)?) }
	}
	
	#[inline(always)]
	fn long8_slice<Unit: Version6OrBigTiffUnit>(&mut self, count: u64, offset_or_value_union_index: Index, byte_order: ByteOrder) -> Result<&[Unaligned<u64>], SpecificTagParseError>
	{
		unsafe { transmute(self.byte_swapped_slice::<Unaligned64>(count, offset_or_value_union_index, Unit::Count64, byte_order)?) }
	}
	
	#[inline(always)]
	fn slong8_slice<Unit: Version6OrBigTiffUnit>(&mut self, count: u64, offset_or_value_union_index: Index, byte_order: ByteOrder) -> Result<&[Unaligned<i64>], SpecificTagParseError>
	{
		unsafe { transmute(self.byte_swapped_slice::<Unaligned64>(count, offset_or_value_union_index, Unit::Count64, byte_order)?) }
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn rational_slice_internal<Unit: Version6OrBigTiffUnit, RFA: RationalFractionAtor>(&mut self, count: u64, offset_or_value_union_index: Index, byte_order: ByteOrder) -> Result<&[Unaligned<RationalFraction<RFA>>], SpecificTagParseError>
	{
		let rational_fraction_slice = self.slice_from_offset_or_value::<Unaligned<RationalFraction<u32>>>(count, offset_or_value_union_index, Unit::Count64, byte_order)?;
		
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
	fn byte_swapped_slice<U: byte_swap::Unaligned>(&mut self, count: u64, offset_or_value_union_index: Index, Count: u64, byte_order: ByteOrder) -> Result<&[U], SpecificTagParseError>
	{
		let slice = self.slice_from_offset_or_value::<U>(count, offset_or_value_union_index, Count)?;
		byte_order.byte_swap(slice);
		Ok(slice)
	}
	
	// `Count` should be a generic const but Rust does not like that it is a const member of the Unit trait.
	#[doc(hidden)]
	#[inline(always)]
	fn slice_from_offset_or_value<X>(&mut self, count: u64, offset_or_value_union_index: Index, Count: u64) -> Result<&mut [X], SpecificTagParseError>
	{
		use SpecificTagParseError::*;
		
		let slice = if count <= Count
		{
			let pointer = self.non_null_to_index_unchecked_mut::<X>(offset_or_value_union_index);
			unsafe { from_raw_parts_mut(pointer.as_ptr(), count as usize) }
		}
		else
		{
			Self::guard_count_fits_in_usize(count)?;
			let offset = Offset::parse_offset_value(tiff_bytes, offset_or_value_union_index).map_err(SliceOffsetParse)?;
			self.slice_mut_checked::<X>(offset.index(), count).map_err(OffsetIsTooLargeForTargetArchitecture)?
		};
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
		Ok()
	}
}

impl<TB: TiffBytes> BytesSlice for TB
{
}
