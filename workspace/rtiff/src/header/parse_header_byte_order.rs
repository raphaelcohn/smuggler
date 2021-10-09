// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


#[inline(always)]
pub(super) fn parse_header_byte_order(tiff_bytes: &impl TiffBytes) -> Result<ByteOrder, HeaderParseError>
{
	use ByteOrderParseError::*;
	use ByteOrder::*;
	
	/// `II`.
	const IntelByteOrder: u16 = 0x4949;
	
	/// `MM`.
	const MotorolaByteOrder: u16 = 0x4D4D;
	
	match tiff_bytes.unaligned_u16_checked_native_endian_byte_order(0).map_err(TooFewBytesForByteOrder)?
	{
		IntelByteOrder => Ok(LittleEndian),
		
		MotorolaByteOrder => Ok(BigEndian),
		
		native_endian_byte_order_bytes @ _ => Err(HeaderParseError::ByteOrderParse(InvalidByteOrder { native_endian_byte_order_bytes })),
	}
}
