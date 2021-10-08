// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


/// An offset.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Offset(u64);

impl Offset
{
	#[inline(always)]
	pub(crate) fn parse_offset_value<TB: TiffBytes + ?Sized>(tiff_bytes: &TB, raw_offset: Index) -> Result<Self, OffsetParseError>
	{
		let file_length = tiff_bytes.file_length();
		if unlikely!(raw_offset > file_length)
		{
			return Err(OffsetParseError::TooLarge { offset: raw_offset, file_length })
		}
		
		Ok(Self(raw_offset))
	}
	
	#[inline(always)]
	pub(crate) const fn index(self) -> Index
	{
		self.0
	}
}
