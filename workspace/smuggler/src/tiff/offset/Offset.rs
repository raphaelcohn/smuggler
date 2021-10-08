// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) struct Offset(u64);

impl Offset
{
	#[inline(always)]
	pub(crate) fn version_6<TB: TiffBytes>(bytes: &TB, index: Index, byte_order: ByteOrder) -> Result<Offset, OffsetParseError>
	{
		Self::get_and_parse::<TB, u32>(bytes, index, byte_order)
	}
	
	#[inline(always)]
	pub(crate) fn version_big_tiff<TB: TiffBytes>(bytes: &TB, index: Index, byte_order: ByteOrder) -> Result<Offset, OffsetParseError>
	{
		Self::get_and_parse::<TB, u64>(bytes, index, byte_order)
	}
	
	#[inline(always)]
	fn get_and_parse<TB: TiffBytes, CBU: CanBeUnaligned + Into<u64>>(bytes: &TB, index: Index, byte_order: ByteOrder) -> Result<Self, OffsetParseError>
	{
		let raw_offset = match TB::unaligned_checked::<CBU>(bytes, index, byte_order)
		{
			Ok(raw_offset) => raw_offset,
			
			Err(cause) => return Err(OffsetParseError::Overflow(cause))
		};
		Self::parse(bytes, raw_offset)
	}
	
	#[inline(always)]
	fn parse<Unit: Into<u64>>(bytes: &impl TiffBytes, raw_offset: Unit) -> Result<Self, OffsetParseError>
	{
		let raw_offset = raw_offset.into();
		Self::parse_offset_value(bytes, raw_offset)
	}
	
	#[inline(always)]
	pub(crate) fn parse_offset_value(bytes: &impl TiffBytes, raw_offset: Index) -> Result<Self, OffsetParseError>
	{
		let file_length = bytes.file_length();
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
