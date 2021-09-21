// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


/// Tags; always sorted in ascending key (tag identifier) order except for unrecognized tags, which are sorted last.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Tags<'a, A: Allocator, T: Tag<'a, A>>
{
	sorted_tags: Vec<T, A>,
}

impl <'a, A: Allocator, T: Tag<'a, A>> Deref for Tags<'a, A, T>
{
	type Target = [T];
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.sorted_tags
	}
}

impl <'a, A: Allocator, T: Tag<'a, A>> Tags<'a, A, T>
{
	/// Find a tag.
	#[inline(always)]
	pub fn find(&self, key: T::Key) -> Option<&T>
	{
		match self.sorted_tags.binary_search_by_key(&key, |tag| tag.key())
		{
			Ok(index) => Some(self.sorted_tags.get_unchecked_safe(index)),
			
			Err(_insertion_index) => None,
		}
	}
	
	#[inline(always)]
	pub(super) fn new(number_of_directory_entries: NonZeroU64, allocator: A) -> Result<Self, TryReserveError>
	{
		let number_of_directory_entries = number_of_directory_entries.get();
		let length = if cfg!(target_pointer_width = "64")
		{
			number_of_directory_entries as usize
		}
		else
		{
			number_of_directory_entries.try_into().map_err(|_| TryReserveError::CapacityOverflow)?
		};
		
		Ok
		(
			Self
			{
				sorted_tags: Vec::new_with_capacity(length, allocator)?
			}
		)
	}
	
	#[inline(always)]
	pub(super) fn push_unchecked_for_duplicates_or_sort(&mut self, tag: T)
	{
		self.sorted_tags.push_unchecked(tag);
	}
}
