// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) struct Offset(u64);

impl Offset
{
	#[inline(always)]
	pub(crate) fn version_6<B: Bytes>(bytes: &B, index: u64, byte_order: ByteOrder) -> Result<Offset, OffsetParseError>
	{
		Self::<B, u32>(bytes, index, byte_order, B::unaligned_u32)
	}
	
	#[inline(always)]
	pub(crate) fn version_big_tiff<B: Bytes>(bytes: &B, index: u64, byte_order: ByteOrder) -> Result<Offset, OffsetParseError>
	{
		Self::<B, u64>(bytes, index, byte_order, B::unaligned_u64)
	}
	
	#[inline(always)]
	fn get_and_parse<B: Bytes, Unit: Into<u64>>(bytes: &B, index: u64, byte_order: ByteOrder, get_raw_offset_callback: impl FnOnce(&B, u64, ByteOrder) -> Result<Unit, OverflowError>) -> Result<Self, OffsetParseError>
	{
		let raw_offset = match get_raw_offset_callback(bytes, index)
		{
			Ok(raw_offset) => raw_offset,
			
			Err(cause) => return Err(OffsetParseError::Overflow(cause))
		};
		Self::parse(bytes, raw_offset)
	}
	
	#[inline(always)]
	pub(crate) fn parse_offset_value(bytes: &impl Bytes, raw_offset: u64) -> Result<Self, OffsetParseError>
	{
		let file_length = bytes.file_length();
		if unlikely!(raw_offset > file_length)
		{
			return Err(OffsetParseError::TooLarge { offset, file_length })
		}
		
		Ok(Self(raw_offset))
	}
	
	#[inline(always)]
	fn parse<Unit: Into<u64>>(bytes: &impl Bytes, raw_offset: Unit) -> Result<Self, OffsetParseError>
	{
		let raw_offset = raw_offset.into();
		Self::parse_offset_value(bytes, raw_offset)
	}
	
	#[inline(always)]
	pub(crate) const fn u64(self) -> u64
	{
		self.0
	}
}
