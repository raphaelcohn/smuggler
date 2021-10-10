// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


/// Records free space in the TIFF file.
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(transparent)]
pub struct FreeSpace<A: Allocator>(FreeSpaceVector<A>);

impl<A: Allocator> FreeSpace<A>
{
	/// This method is non-trivial and not efficient.
	#[inline(always)]
	pub fn length(&self) -> FileLength
	{
		let mut length = 0;
		for free_space_range in self.0.deref()
		{
			length += free_space_range.length();
		}
		return length
	}
	
	/// Iterate over free space.
	#[inline(always)]
	pub fn iterate<'a, 'tiff_bytes: 'a, TB: TiffBytes>(&'a self, tiff_bytes: &'tiff_bytes TB) -> impl 'a + Iterator<Item=&'tiff_bytes [u8]>
	{
		self.free_space_range_iterator().map(move |free_space_range| free_space_range.into_byte_slice(tiff_bytes))
	}
	
	#[inline(always)]
	fn free_space_range_iterator(&self) -> impl Iterator<Item=&SpaceRange>
	{
		self.0.deref().iter()
	}
	
	#[inline(always)]
	pub(crate) fn new<TB: TiffBytes>(allocator: A, tiff_bytes_with_order: &TiffBytesWithOrder<TB>) -> Result<Self, FreeSpaceOutOfMemoryError>
	{
		FreeSpaceVector::new(allocator, tiff_bytes_with_order.file_length()).map(Self)
	}
	
	#[inline(always)]
	pub(crate) fn record_used_space_slice(&mut self, index: Index, size_in_bytes: u64, error: impl FnOnce(TryReserveError) -> FreeSpaceOutOfMemoryError) -> Result<(), FreeSpaceOutOfMemoryError>
	{
		self.0.record_used_space_slice(SpaceRange::from_slice(index, size_in_bytes)).map_err(error)
	}
}
