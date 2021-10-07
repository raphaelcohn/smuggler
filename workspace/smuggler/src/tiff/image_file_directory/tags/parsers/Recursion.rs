// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


#[derive(Default, Debug)]
pub(in crate::tiff::tags) struct Recursion
{
	descent_depth: Cell<u8>,

	seen_image_file_directory_pointers: RefCell<HashSet<ImageFileDirectoryPointer>>,

	free_space: RefCell<FreeSpace>,
}

impl Recursion
{
	/// This allows for 4 levels of Image File Directory (IFD).
	const MaximumDescents: NonZeroU8 = new_non_zero_u8(3);
	
	#[inline(always)]
	pub(in crate::tiff::tags) fn top_level(&self) -> RecursionGuard
	{
		debug_assert_eq!(self.descent_depth.get(), 0, "Already descending");
		self.descent_depth.set(1);
		RecursionGuard(self)
	}
	
	#[inline(always)]
	fn record_used_space_slice(&self, index: Index, size_in_bytes: u64)
	{
		self.free_space.borrow_mut().record_used_space_slice(index, size_in_bytes)
	}
	
	#[inline(always)]
	fn guard_image_file_directory_pointer(&self, image_file_directory_pointer: ImageFileDirectoryPointer) -> Result<(), ImageFileDirectoriesParseError>
	{
		use ImageFileDirectoriesParseError::*;
		
		let borrow = self.seen_image_file_directory_pointers.borrow_mut();
		if borrow.contains(&image_file_directory_pointer)
		{
			return Err(CyclicImageFileDirectoryPointer(image_file_directory_pointer))
		}
		borrow.try_reserve(1).map_err(CouldNotAllocateMemoryForImageFileDirectoryPointer)?;
		let _ = borrow.insert(image_file_directory_pointer);
		Ok(())
	}
}
