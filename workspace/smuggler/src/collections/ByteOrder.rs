// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(u8)]
pub(crate) enum ByteOrder
{
	/// Known as 'Intel' in EXIF.
	LittleEndian,
	
	/// Known as 'Motorola' in EXIF.
	BigEndian,
}

impl Default for ByteOrder
{
	#[inline(always)]
	fn default() -> Self
	{
		ByteOrder::LittleEndian
	}
}

macro_rules! read_value
{
	($self: ident, $pointer_to_index: ident, $read: ident) =>
	{
		{
			use ByteOrder::*;
			
			let value = unsafe { $pointer_to_index.$read() };
			if cfg!(target_endian = "little")
			{
				match $self
				{
					LittleEndian => value,
					
					BigEndian => value.swap_bytes(),
				}
			}
			else if cfg!(target_endian = "big")
			{
				match $self
				{
					LittleEndian => value.swap_bytes(),
					
					BigEndian => value,
				}
			}
			else
			{
				panic!("Unknown target_endian")
			}
		}
	}
}

impl ByteOrder
{
	#[inline(always)]
	fn read_unaligned_u16(self, pointer_to_index: *const u16) -> u16
	{
		read_value!(self, pointer_to_index, read_unaligned)
	}
	
	#[inline(always)]
	fn read_unaligned_u32(self, pointer_to_index: *const u32) -> u32
	{
		read_value!(self, pointer_to_index, read_unaligned)
	}
	
	#[inline(always)]
	fn read_unaligned_u64(self, pointer_to_index: *const u64) -> u64
	{
		read_value!(self, pointer_to_index, read_unaligned)
	}
}
