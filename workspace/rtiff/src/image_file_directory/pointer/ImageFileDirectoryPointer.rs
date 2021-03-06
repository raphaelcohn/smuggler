// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


/// A pointer.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ImageFileDirectoryPointer(NonZeroU64);

impl ImageFileDirectoryPointer
{
	#[inline(always)]
	pub(crate) fn new_unchecked(offset: Result<Offset, OffsetParseError>) -> Result<Option<Self>, ImageFileDirectoryPointerParseError>
	{
		let offset = match offset
		{
			Ok(offset) => offset,
			
			Err(cause) => return Err(ImageFileDirectoryPointerParseError::OffsetParse(cause)),
		};
		
		Self::new_from_offset(offset)
	}
	
	#[inline(always)]
	fn new_from_offset(offset: Offset) -> Result<Option<Self>, ImageFileDirectoryPointerParseError>
	{
		Self::new_from_raw_offset(offset.index())
	}
	
	#[inline(always)]
	pub(crate) fn new_from_raw_offset(raw_offset: u64) -> Result<Option<Self>, ImageFileDirectoryPointerParseError>
	{
		use ImageFileDirectoryPointerParseError::*;
	
		if unlikely!(raw_offset % 2 == 1)
		{
			return Err(NotAlignedToA16BitWordBoundary { raw_offset })
		}
		
		let outcome = if unlikely!(raw_offset == 0)
		{
			None
		}
		else
		{
			
			Some(Self(new_non_zero_u64(raw_offset)))
		};
		Ok(outcome)
	}
	
	#[inline(always)]
	pub(crate) const fn index(self) -> Index
	{
		self.0.get()
	}
}
