// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


#[doc(hidden)]
pub trait Version6OrBigTiffVersion: CanBeUnaligned + Into<u64>
{
	#[doc(hidden)]
	type NumberOfDirectoryEntries: DirectoryEntries;
	
	#[doc(hidden)]
	const U16: u16;
	
	#[doc(hidden)]
	const Version: Version;
	
	#[doc(hidden)]
	const IndexOfZerothImageFileDirectory: Index;
	
	#[doc(hidden)]
	const Size: u64 = size_of_u64::<Self>();
	
	#[doc(hidden)]
	const DirectoryEntryCountSize: u64 = Self::Size;
	
	#[doc(hidden)]
	const PointerSize: u64 = size_of_u64::<Self>();
	
	#[doc(hidden)]
	const HeaderSizeInBytes: u64 = Self::IndexOfZerothImageFileDirectory + Self::PointerSize;
	
	#[doc(hidden)]
	fn parse_header_constants<TB: TiffBytes>(tiff_bytes_with_order: &TiffBytesWithOrder<TB>) -> Result<(), BigTiffHeaderParseError>;
	
	#[doc(hidden)]
	#[inline(always)]
	fn number_of_directory_entries<TB: TiffBytes>(tiff_bytes_with_order: &TiffBytesWithOrder<TB>, index: Index) -> Result<Self::NumberOfDirectoryEntries, OverflowError>
	{
		tiff_bytes_with_order.unaligned_checked(index)
	}
}
