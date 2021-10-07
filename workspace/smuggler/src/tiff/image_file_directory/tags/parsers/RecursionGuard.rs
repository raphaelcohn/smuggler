// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


#[derive(Debug)]
pub(in crate::tiff::tags) struct RecursionGuard<'recursion>(&'recursion Recursion);

impl<'recursion> Drop for RecursionGuard<'recursion>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		let depth = self.0.descent_depth.get();
		debug_assert_ne!(depth, 0);
		self.0.descent_depth.set(depth - 1)
	}
}

impl<'recursion> RecursionGuard<'recursion>
{
	#[inline(always)]
	pub(in crate::tiff::tags) fn recurse(&self) -> Result<Self, SpecificTagParseError>
	{
		let depth = self.0.descent_depth.get();
		
		if unlikely!(depth == Recursion::MaximumDescents.get())
		{
			Err(SpecificTagParseError::MaximumDescentReached)
		}
		else
		{
			self.0.descent_depth.set(depth + 1);
			Ok(Self(self.0))
		}
	}
	
	#[inline(always)]
	pub(in crate::tiff::tags) fn guard_image_file_directory_pointer(&self, image_file_directory_pointer: ImageFileDirectoryPointer) -> Result<(), ImageFileDirectoriesParseError>
	{
		self.0.guard_image_file_directory_pointer(image_file_directory_pointer)
	}
	
	#[inline(always)]
	pub(in crate::tiff::tags) fn record_used_space_slice(&self, index: Index, size_in_bytes: u64)
	{
		self.0.record_used_space_slice(index, size_in_bytes)
	}
}
