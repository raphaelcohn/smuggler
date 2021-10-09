// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


pub(crate) type Version6 = u32;

impl Version6OrBigTiffVersion for Version6
{
	type NumberOfDirectoryEntries = u16;
	
	const U16: u16 = 42;
	
	const Version: Version = Version::_6;
	
	const IndexOfZerothImageFileDirectory: Index = 4;
	
	#[inline(always)]
	fn parse_header_constants<TB: TiffBytes>(_tiff_bytes_with_order: &TiffBytesWithOrder<TB>) -> Result<(), BigTiffHeaderParseError>
	{
		Ok(())
	}
}
