// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct SpaceRange
{
	inclusive_from: Index,
	
	exclusive_to: Index,
}

impl SpaceRange
{
	#[inline(always)]
	fn from_slice(index: Index, size_in_bytes: u64) -> Self
	{
		Self
		{
			inclusive_from: index,
		
			exclusive_to: index + size_in_bytes
		}
	}
	
	#[inline(always)]
	fn into_byte_slice<'tiff_bytes, TB: TiffBytes>(&self, tiff_bytes: &'tiff_bytes TB) -> &'tiff_bytes [u8]
	{
		let length = self.length();
		unsafe { from_raw_parts(tiff_bytes.non_null_to_index_unchecked::<u8>(self.inclusive_from, length).as_ptr(), length as usize) }
	}
	
	#[inline(always)]
	fn length(&self) -> u64
	{
		self.exclusive_to - self.inclusive_from
	}
	
	#[inline(always)]
	const fn is_scenario_5(free_space_range: Self, used_space_range: Self) -> bool
	{
		free_space_range.exclusive_to <= used_space_range.inclusive_from
	}
	
	#[inline(always)]
	const fn is_scenario_6(free_space_range: Self, used_space_range: Self) -> bool
	{
		free_space_range.inclusive_from >= used_space_range.exclusive_to
	}
	
	#[inline(always)]
	fn compare_inclusive_from(free_space_range: Self, used_space_range: Self) -> Ordering
	{
		free_space_range.inclusive_from.cmp(&used_space_range.inclusive_from)
	}
	
	#[inline(always)]
	fn compare_exclusive_to(free_space_range: Self, used_space_range: Self) -> Ordering
	{
		free_space_range.exclusive_to.cmp(&used_space_range.exclusive_to)
	}
}
