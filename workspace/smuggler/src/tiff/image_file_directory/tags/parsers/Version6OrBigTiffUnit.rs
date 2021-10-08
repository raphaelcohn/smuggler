// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


pub(crate) trait Version6OrBigTiffUnit: CanBeUnaligned + Into<u64>
{
	type NumberOfDirectoryEntries: CanBeUnaligned + Into<u64>;
	
	#[inline(always)]
	fn number_of_directory_entries<TB: TiffBytes>(tiff_bytes_with_order: &TiffBytesWithOrder<TB>, index: Index) -> Result<Self::NumberOfDirectoryEntries, OverflowError>
	{
		tiff_bytes_with_order.unaligned_checked(index)
	}
}

impl Version6OrBigTiffUnit for u32
{
	type NumberOfDirectoryEntries = u16;
}

impl Version6OrBigTiffUnit for u64
{
	type NumberOfDirectoryEntries = u64;
}
