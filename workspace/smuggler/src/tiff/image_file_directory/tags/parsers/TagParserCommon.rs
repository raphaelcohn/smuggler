// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


#[derive(Debug, Eq, PartialEq)]
pub(crate) struct TagParserCommon<'tiff_bytes, 'recursion: 'recursion_guard, 'recursion_guard, TB: TiffBytes, A: Allocator + Copy>
{
	pub(crate) tiff_bytes_with_order: TiffBytesWithOrder<'tiff_bytes, TB>,
	
	pub(crate) recursion_guard: &'recursion_guard RecursionGuard<'recursion>,
	
	pub(crate) allocator: A,
}

impl<'tiff_bytes, 'recursion: 'recursion_guard, 'recursion_guard, TB: TiffBytes, A: Allocator + Copy> Deref for TagParserCommon<'tiff_bytes, 'recursion, 'recursion_guard, TB, A>
{
	type Target = TiffBytesWithOrder<'tiff_bytes, TB>;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.tiff_bytes_with_order
	}
}

impl<'tiff_bytes, 'recursion: 'recursion_guard, 'recursion_guard, TB: TiffBytes, A: Allocator + Copy> TagParserCommon<'tiff_bytes, 'recursion, 'recursion_guard, TB, A>
{
	#[inline(always)]
	pub(crate) const fn new(tiff_bytes_with_order: TiffBytesWithOrder<'tiff_bytes, TB>, recursion_guard: &'recursion_guard RecursionGuard<'recursion>, allocator: A) -> Self
	{
		Self
		{
			tiff_bytes_with_order,
			
			recursion_guard,
		
			allocator,
		}
	}
	
	#[inline(always)]
	pub(crate) fn recurse(&self) -> Result<Self, SpecificTagParseError>
	{
		let recursion_guard = self.recursion_guard.recurse()?;
		Ok(Self::new(self.tiff_bytes_with_order, &recursion_guard, self.allocator))
	}
	
	#[inline(always)]
	pub(crate) const fn tiff_bytes(&self) -> &'tiff_bytes TB
	{
		self.tiff_bytes_with_order.tiff_bytes
	}
	
	#[inline(always)]
	pub(crate) const fn byte_order(&self) -> ByteOrder
	{
		self.tiff_bytes_with_order.byte_order
	}
}
