// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


#[derive(Debug, Eq, PartialEq)]
pub(crate) struct TagParserCommon<'tiff_bytes, 'allocator, TB: TiffBytes, A: Allocator + Clone, Version: Version6OrBigTiffVersion>
{
	tiff_bytes_with_order: TiffBytesWithOrder<'tiff_bytes, TB>,
	
	allocator: &'allocator A,
	
	descent_depth: u8,
	
	seen_image_file_directory_pointers: HashSet<ImageFileDirectoryPointer>,
	
	free_space: FreeSpace<A>,

	marker: PhantomData<Version>,
}

impl<'tiff_bytes, 'allocator, TB: TiffBytes, A: Allocator + Clone, Version: Version6OrBigTiffVersion> Deref for TagParserCommon<'tiff_bytes, 'allocator, TB, A, Version>
{
	type Target = TiffBytesWithOrder<'tiff_bytes, TB>;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.tiff_bytes_with_order
	}
}

impl<'tiff_bytes, 'allocator, TB: TiffBytes, A: Allocator + Clone, Version: Version6OrBigTiffVersion> DerefMut for TagParserCommon<'tiff_bytes, 'allocator, TB, A, Version>
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target
	{
		&mut self.tiff_bytes_with_order
	}
}

impl<'tiff_bytes, 'allocator, TB: TiffBytes, A: Allocator + Clone, Version: Version6OrBigTiffVersion> TagParserCommon<'tiff_bytes, 'allocator, TB, A, Version>
{
	/// This allows for 4 levels of Image File Directory (IFD).
	const MaximumDescents: NonZeroU8 = new_non_zero_u8(3);
	
	#[inline(always)]
	pub(crate) fn new(tiff_bytes_with_order: TiffBytesWithOrder<'tiff_bytes, TB>, allocator: &'allocator A) -> Result<Self, FreeSpaceOutOfMemoryError>
	{
		Ok
		(
			Self
			{
				free_space: FreeSpace::new(allocator.clone(), &tiff_bytes_with_order)?,
				
				tiff_bytes_with_order,
			
				allocator,
			
				descent_depth: 0,
			
				seen_image_file_directory_pointers: HashSet::new(),
			
				marker: PhantomData,
			}
		)
	}
	
	#[inline(always)]
	pub(in crate::image_file_directory) fn guard_image_file_directory_pointer(&mut self, image_file_directory_pointer: ImageFileDirectoryPointer) -> Result<(), ImageFileDirectoriesParseError>
	{
		use ImageFileDirectoriesParseError::*;
		
		let seen_image_file_directory_pointers = &mut self.seen_image_file_directory_pointers;
		if seen_image_file_directory_pointers.contains(&image_file_directory_pointer)
		{
			return Err(CyclicImageFileDirectoryPointer(image_file_directory_pointer))
		}
		seen_image_file_directory_pointers.try_reserve(1).map_err(CouldNotAllocateMemoryForImageFileDirectoryPointer)?;
		let _ = seen_image_file_directory_pointers.insert(image_file_directory_pointer);
		Ok(())
	}
	
	#[inline(always)]
	pub(crate) fn record_used_space_slice(&mut self, index: Index, size_in_bytes: u64, error: impl FnOnce(TryReserveError) -> FreeSpaceOutOfMemoryError) -> Result<(), FreeSpaceOutOfMemoryError>
	{
		self.free_space.record_used_space_slice(index, size_in_bytes, error)
	}
	
	#[inline(always)]
	pub(in crate::image_file_directory) fn allocator(&self) -> A
	{
		self.allocator.clone()
	}
	
	#[inline(always)]
	pub(crate) fn byte_order(&self) -> ByteOrder
	{
		self.byte_order
	}
	
	#[inline(always)]
	pub(in crate::image_file_directory) fn recurse<R>(&mut self, callback: impl FnOnce(&mut Self) -> R) -> Result<R, ChildImageFileDirectoriesParseError>
	{
		self.descend()?;
		let result = callback(self);
		self.ascend();
		Ok(result)
	}
	
	#[inline(always)]
	fn descend(&mut self) -> Result<(), ChildImageFileDirectoriesParseError>
	{
		if unlikely!(self.descent_depth == Self::MaximumDescents.get())
		{
			return Err(ChildImageFileDirectoriesParseError::MaximumDescentReached)
		}
		self.descent_depth += 1;
		Ok(())
	}
	
	#[inline(always)]
	fn ascend(&mut self)
	{
		debug_assert_ne!(self.descent_depth, 0);
		self.descent_depth -= 1;
	}
	
	#[inline(always)]
	pub(crate) fn finish(self) -> (ByteOrder, FreeSpace<A>)
	{
		(self.byte_order, self.free_space)
	}
}
