// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


pub(in crate::tiff::image_file_directory) trait Version6OrBigTiffUnit: Into<u64>
{
	type NumberOfDirectoryEntries: Into<u64>;
	
	type OffsetLikeValue: Into<u64>;
	
	const Count8: u64;
	
	const Count16: u64;
	
	const Count32: u64;
	
	const Count64: u64;
	
	fn number_of_directory_entries<TB: TiffBytes>(tiff_bytes: &TB, index: Index, byte_order: ByteOrder) -> Result<Self::NumberOfDirectoryEntries, OverflowError>;
	
	fn offset_like_value<TB: TiffBytes>(tiff_bytes: &TB, index: Index, byte_order: ByteOrder) -> Result<Self::OffsetLikeValue, OverflowError>;
	
	fn offset_like_value_unchecked<TB: TiffBytes>(tiff_bytes: &TB, index: Index, byte_order: ByteOrder) -> Self::OffsetLikeValue;
	
	fn image_file_directory_pointer<TB: TiffBytes>(tiff_bytes: &TB, index: Index, byte_order: ByteOrder) -> Result<Option<ImageFileDirectoryPointer>, ImageFileDirectoryPointerParseError>;
	
	fn long8_offset_or_value<TB: TiffBytes>(tiff_bytes: &TB, offset_or_value_union_index: Index, byte_order: ByteOrder) -> Result<u64, SpecificTagParseError>;
	
	fn slong8_offset_or_value<TB: TiffBytes>(tiff_bytes: &TB, offset_or_value_union_index: Index, byte_order: ByteOrder) -> Result<i64, SpecificTagParseError>;
}

impl Version6OrBigTiffUnit for u32
{
	type NumberOfDirectoryEntries = u16;
	
	type OffsetLikeValue = u32;
	
	const Count8: u64 = 4;
	
	const Count16: u64 = 2;
	
	const Count32: u64 = 1;
	
	const Count64: u64 = 0;
	
	#[inline(always)]
	fn number_of_directory_entries<TB: TiffBytes>(tiff_bytes: &TB, index: Index, byte_order: ByteOrder) -> Result<Self::NumberOfDirectoryEntries, OverflowError>
	{
		tiff_bytes.unaligned_checked(index, byte_order)
	}
	
	#[inline(always)]
	fn offset_like_value<TB: TiffBytes>(tiff_bytes: &TB, index: Index, byte_order: ByteOrder) -> Result<Self::OffsetLikeValue, OverflowError>
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
	
	#[inline(always)]
	fn long8_offset_or_value<TB: TiffBytes>(tiff_bytes: &TB, offset_or_value_union_index: Index, byte_order: ByteOrder) -> Result<u64, SpecificTagParseError>
	{
		use SpecificTagParseError::*;
		
		let offset = tiff_bytes.offset_version_6(offset_or_value_union_index, byte_order).map_err(Long8OffsetParse)?;
		let index = offset.index();
		tiff_bytes.unaligned_checked(index, byte_order).map_err(Long8Overflow)
	}
	
	#[inline(always)]
	fn slong8_offset_or_value<TB: TiffBytes>(tiff_bytes: &TB, offset_or_value_union_index: Index, byte_order: ByteOrder) -> Result<i64, SpecificTagParseError>
	{
		use SpecificTagParseError::*;
		
		let offset = tiff_bytes.offset_version_6(offset_or_value_union_index, byte_order).map_err(Slong8OffsetParse)?;
		let index = offset.index();
		tiff_bytes.unaligned_checked(index, byte_order).map_err(Slong8Overflow)
	}
}

impl Version6OrBigTiffUnit for u64
{
	type NumberOfDirectoryEntries = u64;
	
	type OffsetLikeValue = u64;
	
	const Count8: u64 = 8;
	
	const Count16: u64 = 4;
	
	const Count32: u64 = 2;
	
	const Count64: u64 = 1;
	
	#[inline(always)]
	fn number_of_directory_entries<TB: TiffBytes>(tiff_bytes: &TB, index: Index, byte_order: ByteOrder) -> Result<Self::NumberOfDirectoryEntries, OverflowError>
	{
		tiff_bytes.unaligned_checked(index, byte_order)
	}
	
	#[inline(always)]
	fn offset_like_value<TB: TiffBytes>(tiff_bytes: &TB, index: Index, byte_order: ByteOrder) -> Result<Self::OffsetLikeValue, OverflowError>
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
	
	#[inline(always)]
	fn long8_offset_or_value<TB: TiffBytes>(tiff_bytes: &TB, offset_or_value_union_index: Index, byte_order: ByteOrder) -> Result<u64, TagParseError>
	{
		Ok(tiff_bytes.unaligned_unchecked(offset_or_value_union_index, byte_order))
	}
	
	#[inline(always)]
	fn slong8_offset_or_value<TB: TiffBytes>(tiff_bytes: &TB, offset_or_value_union_index: Index, byte_order: ByteOrder) -> Result<i64, TagParseError>
	{
		Ok(tiff_bytes.unaligned_unchecked(offset_or_value_union_index, byte_order))
	}
}
