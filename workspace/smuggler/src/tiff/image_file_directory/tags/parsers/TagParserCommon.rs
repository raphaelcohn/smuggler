// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(in crate::tiff::image_file_directory) struct TagParserCommon<'tiff_bytes, 'recursion: 'recursion_guard, 'recursion_guard, TB: TiffBytes, A: Allocator + Copy>
{
	pub(in crate::tiff::image_file_directory) tiff_bytes_with_order: TiffBytesWithOrder<'tiff_bytes, TB>,
	
	pub(in crate::tiff::image_file_directory) recursion_guard: &'recursion_guard RecursionGuard<'recursion>,
	
	pub(in crate::tiff::image_file_directory) allocator: A,
}

impl<'tiff_bytes, 'recursion: 'recursion_guard, 'recursion_guard, TB: TiffBytes, A: Allocator + Copy> TagParserCommon<'tiff_bytes, 'recursion, 'recursion_guard, A>
{
	#[inline(always)]
	pub(in crate::tiff::image_file_directory) const fn new(tiff_bytes_with_order: TiffBytesWithOrder<'tiff_bytes, TB>, recursion_guard: &'recursion_guard RecursionGuard<'recursion>, allocator: A) -> Self
	{
		Self
		{
			tiff_bytes_with_order,
			
			recursion_guard,
		
			allocator,
		}
	}
}
