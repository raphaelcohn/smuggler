// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


pub(crate) type VersionBigTiff = u64;

impl Version6OrBigTiffUnit for VersionBigTiff
{
	type NumberOfDirectoryEntries = u64;
	
	const U16: u16 = 43;
	
	const Version: Version = Version::BigTiff;
	
	const IndexOfZerothImageFileDirectory: Index = 8;
	
	#[inline(always)]
	fn parse_header_constants<TB: TiffBytes>(tiff_bytes_with_order: &TiffBytesWithOrder<TB>) -> Result<(), BigTiffHeaderParseError>
	{
		use BigTiffHeaderParseError::*;
		
		{
			let offset_size_in_bytes = tiff_bytes_with_order.unaligned_checked(4).map_err(TooFewBytesForOffsetSize)?;
			if unlikely!(offset_size_in_bytes != 0x0008)
			{
				return Err(OffsetSizeWasNot8 { offset_size_in_bytes })
			}
		}
		
		{
			let constant = tiff_bytes_with_order.unaligned_checked(6).map_err(TooFewBytesForConstant)?;
			if unlikely!(constant != 0x0000)
			{
				return Err(ConstantWasNot0 { constant })
			}
		}
		
		Ok(())
	}
}
