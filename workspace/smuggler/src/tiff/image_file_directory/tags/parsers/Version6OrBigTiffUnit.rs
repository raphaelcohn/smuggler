// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


pub(in crate::tiff::image_file_directory) trait Version6OrBigTiffUnit: Into<u64>
{
	type NumberOfDirectoryEntries: Into<u64>;
	
	const NumberOfDirectoryEntriesSize: u64 = size_of_u64::<Self::NumberOfDirectoryEntries>();
	
	type OffsetLikeValue: Into<u64>;
	
	const ImageFileDirectoryPointerSize: u64 = size_of_u64::<Self::OffsetLikeValue>();
	
	const OffsetOrValueUnionSize: u64 = size_of_u64::<Self::OffsetLikeValue>();
	
	fn number_of_directory_entries<TB: TiffBytes>(tiff_bytes: &TB, index: Index, byte_order: ByteOrder) -> Result<Self::NumberOfDirectoryEntries, OverflowError>;
	
	fn offset_like_value_unchecked<TB: TiffBytes>(tiff_bytes: &TB, index: Index, byte_order: ByteOrder) -> Self::OffsetLikeValue;
	
	fn image_file_directory_pointer<TB: TiffBytes>(tiff_bytes: &TB, index: Index, byte_order: ByteOrder) -> Result<Option<ImageFileDirectoryPointer>, ImageFileDirectoryPointerParseError>;
}

impl Version6OrBigTiffUnit for u32
{
	type NumberOfDirectoryEntries = u16;
	
	type OffsetLikeValue = u32;
	
	#[inline(always)]
	fn number_of_directory_entries<TB: TiffBytes>(tiff_bytes: &TB, index: Index, byte_order: ByteOrder) -> Result<Self::NumberOfDirectoryEntries, OverflowError>
	{
		tiff_bytes.unaligned_checked(index, byte_order)
	}
	
	#[inline(always)]
	fn offset_like_value_unchecked<TB: TiffBytes>(tiff_bytes: &TB, index: Index, byte_order: ByteOrder) -> Self::OffsetLikeValue
	{
		tiff_bytes.unaligned_unchecked(index, byte_order)
	}
	
	#[inline(always)]
	fn image_file_directory_pointer<TB: TiffBytes>(tiff_bytes: &TB, index: Index, byte_order: ByteOrder) -> Result<Option<ImageFileDirectoryPointer>, ImageFileDirectoryPointerParseError>
	{
		tiff_bytes.image_file_directory_pointer_version_6(index, byte_order)
	}
}

impl Version6OrBigTiffUnit for u64
{
	type NumberOfDirectoryEntries = u64;
	
	type OffsetLikeValue = u64;
	
	#[inline(always)]
	fn number_of_directory_entries<TB: TiffBytes>(tiff_bytes: &TB, index: Index, byte_order: ByteOrder) -> Result<Self::NumberOfDirectoryEntries, OverflowError>
	{
		tiff_bytes.unaligned_checked(index, byte_order)
	}
	
	#[inline(always)]
	fn offset_like_value_unchecked<TB: TiffBytes>(tiff_bytes: &TB, index: Index, byte_order: ByteOrder) -> Self::OffsetLikeValue
	{
		tiff_bytes.unaligned_unchecked(index, byte_order)
	}
	
	#[inline(always)]
	fn image_file_directory_pointer<TB: TiffBytes>(tiff_bytes: &TB, index: Index, byte_order: ByteOrder) -> Result<Option<ImageFileDirectoryPointer>, ImageFileDirectoryPointerParseError>
	{
		tiff_bytes.image_file_directory_pointer_version_big_tiff(index, byte_order)
	}
}
