// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


/// A set of one or more ASCII strings.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct AsciiStrings<'tiff_bytes, A: Allocator>
{
	strings: Vec<&'tiff_bytes [NonZeroU8], A>,

	omits_final_nul_byte: bool,
}

impl<'tiff_bytes, A: Allocator + Copy> AsciiStrings<'tiff_bytes, A>
{
	#[inline(always)]
	pub(crate) fn parse(allocator: A, byte_slice: &'tiff_bytes [u8]) -> Result<Self, SpecificTagParseError>
	{
		#[inline(always)]
		fn u8_to_ascii(string: &[u8]) -> &[NonZeroU8]
		{
			unsafe { transmute(string) }
		}
		
		#[inline(always)]
		fn try_push_string<'tiff_bytes>(strings: &mut Vec<&'tiff_bytes [NonZeroU8]>, string: &'tiff_bytes [u8]) -> Result<(), SpecificTagParseError>
		{
			strings.try_push(u8_to_ascii(string)).map_err(SpecificTagParseError::CouldNotAllocateMemoryForAsciiStringReference)
		}
		
		let mut strings: Vec<&'tiff_bytes [NonZeroU8]> = Vec::new_in(allocator);
		let mut remaining_bytes = byte_slice;
		loop
		{
			match memchr(0x00, remaining_bytes)
			{
				None =>
				{
					try_push_string(&mut strings, remaining_bytes)?;
					
					return Ok(Self::new(strings, true))
				}
				
				Some(index) =>
				{
					try_push_string(&mut strings, remaining_bytes.get_unchecked_range_safe(.. index))?;
					
					let next = index + 1;
					if next == remaining_bytes.len()
					{
						return Ok(Self::new(strings, false))
					}
					remaining_bytes = remaining_bytes.get_unchecked_range_safe(next ..);
				}
			}
		}
	}
	
	#[inline(always)]
	const fn new(strings: Vec<&'tiff_bytes [NonZeroU8], A>, omits_final_nul_byte: bool) -> Self
	{
		Self
		{
			strings,
		
			omits_final_nul_byte,
		}
	}
}
