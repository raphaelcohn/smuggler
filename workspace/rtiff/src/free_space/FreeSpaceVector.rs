// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


#[derive(Debug, Clone, PartialOrd, PartialEq, Ord, Eq, Hash)]
#[repr(transparent)]
struct FreeSpaceVector<A: Allocator>
{
	free_space_ranges: Vec<SpaceRange, A>,
}

impl<A: Allocator> Deref for FreeSpaceVector<A>
{
	type Target = [SpaceRange];
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.free_space_ranges
	}
}

impl<A: Allocator> FreeSpaceVector<A>
{
	#[inline(always)]
	fn new(allocator: A, file_length: FileLength) -> Result<Self, TryReserveError>
	{
		Ok
		(
			Self
			{
				free_space_ranges: Vec::new_with_capacity(file_length, allocator)?
			}
		)
	}
	
	/// Uses an optimized binary search, rather than the one provided by Rust in `std::slice::binary_search()`.
	///
	/// There are many scenarios.
	///
	/// In these scenarios, `[` means an inclusive index and `)` means an exclusive index, `FSR` means Free Space Range and `USR` means Used Space Range.
	/// ```sh
	/// 1. Overlap within FSR.
	///
	/// 1.a. Exact.
	/// 	[  FSR  )
	/// 	[  USR  )
	///
	/// 1.b. Less than end.
	///		[  FSR  )
	///		[ USR )
	///
	/// 1.c. Greater than start.
	///		[  FSR  )
	///		  [ USR )
	///
	/// 1.d. Included.
	///		[  FSR  )
	///		 [ USR )
	///
	/// 2. Partial greater overlap.
	///
	/// 2.a. Not equal inclusive start.
	///
	/// 2.a.1.
	/// 	[  FSR  )
	///			[  USR  )
	///
	/// 2.a.2. Partial greater overlap with FSR but also partial greater overlap with Other FSR.
	/// Detection of overlap is with FSR.
	/// 	[  FSR  ) [ Other FSR )
	///			[  USR  )
	///
	/// 2.b. Equal inclusive start.
	///
	/// 2.b.1.
	/// 	[  FSR  )
	///		[      USR  )
	///
	/// 2.b.2. Partial greater overlap with FSR but also partial greater overlap with Other FSR.
	/// Detection of overlap is with FSR.
	/// 	[  FSR  ) [ Other FSR )
	///		[      USR  )
	///
	/// 3. Partial lesser overlap with FSR but also partial less overlap with Other FSR.
	///
	/// 3.a. Not equal exclusive end.
	///
	/// 3.a.1.
	/// 		[  FSR  )
	///		[  USR  )
	///
	/// 3.a.2. Detection of overlap is with FSR.
	/// 	[  Other FSR  ) [  FSR  )
	///					[  USR  )
	///
	/// 3.b. Equal exclusive end.
	///
	/// 3.b.1.
	/// 		[  FSR  )
	///		[  USR      )
	///
	/// 3.b.2. Detection of overlap is with FSR.
	/// 	[  Other FSR  ) [  FSR  )
	///					[  USR      )
	///
	/// 4. Messy; iterate in both directions.
	/// 	 [ FSR )
	/// 	[  USR  )
	///
	/// 	[  Other FSR  ) [ FSR ) [  Other FSR  )
	/// 	            [    USR      )
	///
	/// 5. No overlap; exclusive end of FSR is equal to or less than inclusive start of USR.
	/// 	[  FSR  )
	/// 			[  USR  )
	/// 	[  FSR  )
	/// 			 [  USR  )
	/// 	Start searching to the right of FSR.
	///
	/// 6. No overlap; exclusive end of USR is equal to or less than inclusive start of FSR.
	/// 			[  FSR  )
	/// 	[  USR  )
	/// 			 [  FSR  )
	/// 	[  USR  )
	/// 	Start searching to the left of FSR.
	/// ```
	#[inline(always)]
	fn record_used_space_slice(&mut self, used_space_range: SpaceRange)
	{
		let mut size = self.length();
        let mut left = 0;
        let mut right = size;
        while left < right
		{
            let mid = left + size / 2;
			
			let free_space_range = *self.get_free_space_range(mid);
			
			if SpaceRange::is_scenario_5(free_space_range, used_space_range)
			{
				Self::scenario_5(mid, &mut left, &mut right, &mut size)
			}
			else if SpaceRange::is_scenario_6(free_space_range, used_space_range)
			{
				Self::scenario_6(mid, &mut left, &mut right, &mut size)
			}
			else
			{
				return self.scenarios_1_to_4(mid, free_space_range, used_space_range)
			}
        }
    }
	
	/// FSR was less than USR with no overlap so search to the right.
	#[inline(always)]
	fn scenario_5(mid: usize, left: &mut usize, right: &mut usize, size: &mut usize)
	{
		*left = mid + 1;
		*size = *right - *left;
	}
	
	/// FSR was greater than USR with no overlap so search to the left.
	#[inline(always)]
	fn scenario_6(mid: usize, left: &mut usize, right: &mut usize, size: &mut usize)
	{
		*right = mid;
		*size = *right - *left;
	}
	
	#[inline(always)]
	fn scenarios_1_to_4(&mut self, mid: usize, free_space_range: SpaceRange, used_space_range: SpaceRange)
	{
		use Ordering::*;
		match (SpaceRange::compare_inclusive_from(free_space_range, used_space_range), SpaceRange::compare_exclusive_to(free_space_range, used_space_range))
		{
			(Less, Less) => self.scenario_2a(mid, used_space_range),
			(Less, Equal) => self.scenario_1c(mid, used_space_range),
			(Less, Greater) => self.scenario_1d(mid, used_space_range),
			
			(Equal, Less) => self.scenario_2b(mid, used_space_range),
			(Equal, Equal) => self.scenario_1a(mid),
			(Equal, Greater) => self.scenario_1b(mid, used_space_range),
			
			(Greater, Less) => self.scenario_4(mid, used_space_range),
			(Greater, Equal) => self.scenario_3b(mid, used_space_range),
			(Greater, Greater) => self.scenario_3a(mid, used_space_range),
		}
	}
	
	#[inline(always)]
	fn scenario_1a(&mut self, mid: usize)
	{
		self.remove(mid)
	}
	
	#[inline(always)]
	fn scenario_1b(&mut self, mid: usize, used_space_range: SpaceRange)
	{
		let free_space_range = self.get_free_space_range_mut(mid);
		free_space_range.inclusive_from = used_space_range.exclusive_to;
	}
	
	#[inline(always)]
	fn scenario_1c(&mut self, mid: usize, used_space_range: SpaceRange)
	{
		let free_space_range = self.get_free_space_range_mut(mid);
		free_space_range.exclusive_to = used_space_range.inclusive_from;
	}
	
	#[inline(always)]
	fn scenario_1d(&mut self, mid: usize, used_space_range: SpaceRange)
	{
		let old_exclusive_to =
		{
			let free_space_range = self.get_free_space_range_mut(mid);
			let old_exclusive_to = free_space_range.exclusive_to;
			free_space_range.exclusive_to = used_space_range.inclusive_from;
			old_exclusive_to
		};
		
		self.free_space_ranges.insert(mid, SpaceRange { inclusive_from: used_space_range.exclusive_to, exclusive_to: old_exclusive_to });
	}
	
	#[inline(always)]
	fn scenario_2a(&mut self, mid: usize, used_space_range: SpaceRange)
	{
		{
			let free_space_range = self.get_free_space_range_mut(mid);
			free_space_range.exclusive_to = used_space_range.inclusive_from;
		}
		
		self.scenario_2_common::<1>(mid, used_space_range)
	}
	
	#[inline(always)]
	fn scenario_2b(&mut self, mid: usize, used_space_range: SpaceRange)
	{
		self.scenario_2_common::<0>(mid, used_space_range)
	}
	
	#[inline(always)]
	fn scenario_2_common<const RemoveFromStartAdjustment: usize>(&mut self, mid: usize, used_space_range: SpaceRange)
	{
		let length = self.length();
		
		let remove_from_free_space_range_index = mid + RemoveFromStartAdjustment;
		for free_space_range_index in (mid + 1) .. length
		{
			let free_space_range = self.get_free_space_range_mut(free_space_range_index);
			
			use Ordering::*;
			match free_space_range.exclusive_to.cmp(&used_space_range.exclusive_to)
			{
				// ```sh
				//	  [  FSR  )
				//	[    USR    )
				// ```
				Less => continue,
				
				// ```sh
				//		[  FSR  )
				//	[ USR       )
				// ```
				Equal => return self.remove_exclusive(remove_from_free_space_range_index, free_space_range_index + 1),
				
				// ```sh
				//		[  FSR  )
				//	[  USR  )
				//		    [  FSR  )
				//	[  USR  )
				//		     [  FSR  )
				//	[  USR  )
				// ```
				Greater => return
				{
					// ```sh
					//		[  FSR  )
					//	[  USR  )
					// ```
					if free_space_range.inclusive_from < used_space_range.exclusive_to
					{
						free_space_range.inclusive_from = used_space_range.exclusive_to;
					}
					
					self.remove_exclusive(remove_from_free_space_range_index, free_space_range_index)
				},
			}
		}
		
		self.remove_exclusive(remove_from_free_space_range_index, length);
	}
	
	#[inline(always)]
	fn scenario_3a(&mut self, mid: usize, used_space_range: SpaceRange)
	{
		{
			let free_space_range = self.get_free_space_range_mut(mid);
			free_space_range.inclusive_from = used_space_range.exclusive_to;
		}
		
		if unlikely!(mid == 0)
		{
			return
		}
		
		self.scenario_3_common::<1>(new_non_zero_usize(mid), used_space_range)
	}
	
	#[inline(always)]
	fn scenario_3b(&mut self, mid: usize, used_space_range: SpaceRange)
	{
		if unlikely!(mid == 0)
		{
			self.remove(mid);
			return
		}
		
		self.scenario_3_common::<0>(new_non_zero_usize(mid), used_space_range)
	}
	
	#[inline(always)]
	fn scenario_3_common<const RemoveFromEndAdjustment: usize>(&mut self, mid: NonZeroUsize, used_space_range: SpaceRange)
	{
		let mid = mid.get();
		let exclusive_end = mid - RemoveFromEndAdjustment;
		let mut free_space_range_index = mid - 1;
		loop
		{
			let free_space_range = self.get_free_space_range_mut(free_space_range_index);
			
			use Ordering::*;
			match free_space_range.inclusive_from.cmp(&used_space_range.inclusive_from)
			{
				// ```sh
				//	[  FSR  )
				//		[    USR    )
				//	[  FSR  )
				//		    [    USR    )
				//	[  FSR  )
				//		     [    USR    )
				// ```
				Less => return
				{
					if free_space_range.exclusive_to > used_space_range.inclusive_from
					{
						free_space_range.exclusive_to = used_space_range.inclusive_from;
					}
					
					self.remove_exclusive(free_space_range_index + 1, exclusive_end)
				},
				
				// ```sh
				//	[  FSR  )
				//	[ USR       )
				// ```
				Equal => return self.remove_exclusive(free_space_range_index, exclusive_end),
				
				// ```sh
				//		[ FSR )
				//	[    USR    )
				// ```
				Greater => if unlikely!(free_space_range_index == 0)
				{
					break
				},
			}
			
			free_space_range_index -= 1;
		}
		
		self.remove_exclusive(0, exclusive_end);
	}
	
	#[inline(always)]
	fn scenario_4(&mut self, mid: usize, used_space_range: SpaceRange)
	{
		self.scenario_2b(mid, used_space_range);
		
		if unlikely!(mid == 0)
		{
			return
		}
		
		self.scenario_3_common::<0>(new_non_zero_usize(mid), used_space_range)
	}
	
	/// Done this way rather than using `Vec::drain()` as it is more efficient.
	///
	/// ```sh
	///
	/// Vector layout
	///
	/// From
	///                                       /--- Tail Length --\
	/// [     Unchanged     |     Removed     |       Tail       ]
	/// |                   |                 |                  |
	/// 0            inclusive_start     exclusive_end      old_length
	/// \------------------- old_length ------------------------/
	///
	/// To
	/// [     Unchanged     |       Tail       ]
	/// |                   |                  |
	/// ```
	#[inline(always)]
	fn remove_exclusive(&mut self, inclusive_start: usize, exclusive_end: usize)
	{
		debug_assert!(inclusive_start <= exclusive_end);
		
		let old_length = self.length();
		let tail_length = old_length - exclusive_end;
		let new_length = inclusive_start + tail_length;
		let tail_inclusive_start = exclusive_end;
		let pointer_ref = self.free_space_ranges.as_ptr();
		let pointer_mut = self.free_space_ranges.as_mut_ptr();
		unsafe
		{
			let pointer_to_start = pointer_mut.add(inclusive_start);
			let pointer_to_tail = pointer_ref.add(tail_inclusive_start);
			pointer_to_start.copy_from(pointer_to_tail, tail_length);
			self.free_space_ranges.set_len(new_length);
		}
	}
	
	
	#[inline(always)]
	fn remove(&mut self, mid: usize)
	{
		let _ = self.free_space_ranges.remove(mid);
	}
	
	#[inline(always)]
	fn length(&self) -> usize
	{
		self.free_space_ranges.len()
	}
	
	#[inline(always)]
	fn get_free_space_range(&self, free_space_range_index: usize) -> &SpaceRange
	{
		self.free_space_ranges.get_unchecked_safe(free_space_range_index)
	}
	
	#[inline(always)]
	fn get_free_space_range_mut(&mut self, free_space_range_index: usize) -> &mut SpaceRange
	{
		self.free_space_ranges.get_unchecked_mut_safe(free_space_range_index)
	}
}
