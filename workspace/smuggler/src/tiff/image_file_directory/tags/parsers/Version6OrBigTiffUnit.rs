// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


pub(in crate::tiff::image_file_directory) trait Version6OrBigTiffUnit: Into<u64>
{
	type NumberOfDirectoryEntries: Into<u64>;
	
	type OffsetLikeValue: Into<u64>;
	
	fn number_of_directory_entries<B: Bytes>(tiff_bytes: &B, index: u64, byte_order: ByteOrder) -> Result<Self::NumberOfDirectoryEntries, OverflowError>;
	
	fn offset_like_value<B: Bytes>(tiff_bytes: &B, index: u64, byte_order: ByteOrder) -> Result<Self::OffsetLikeValue, OverflowError>;
	
	fn offset_like_value_unchecked<B: Bytes>(tiff_bytes: &B, index: u64, byte_order: ByteOrder) -> Self::OffsetLikeValue;
	
	fn image_file_directory_pointer<B: Bytes>(tiff_bytes: &B, index: u64, byte_order: ByteOrder) -> Result<Option<ImageFileDirectoryPointer>, ImageFileDirectoryPointerParseError>;
	
	fn long8_offset_or_value<B: Bytes>(tiff_bytes: &B, offset_or_value_union_index: u64, byte_order: ByteOrder) -> Result<u64, TagParseError>;
}

impl Version6OrBigTiffUnit for u32
{
	type NumberOfDirectoryEntries = u16;
	
	type OffsetLikeValue = u32;
	
	#[inline(always)]
	fn number_of_directory_entries<B: Bytes>(tiff_bytes: &B, index: u64, byte_order: ByteOrder) -> Result<Self::NumberOfDirectoryEntries, OverflowError>
	{
		tiff_bytes.unaligned_u16_value(index, byte_order)
	}
	
	#[inline(always)]
	fn offset_like_value<B: Bytes>(tiff_bytes: &B, index: u64, byte_order: ByteOrder) -> Result<Self::OffsetLikeValue, OverflowError>
	{
		tiff_bytes.unaligned_u32_value(index, byte_order)
	}
	
	#[inline(always)]
	fn offset_like_value_unchecked<B: Bytes>(tiff_bytes: &B, index: u64, byte_order: ByteOrder) -> Self::OffsetLikeValue
	{
		tiff_bytes.unaligned_u32_value_unchecked(index, byte_order)
	}
	
	#[inline(always)]
	fn image_file_directory_pointer<B: Bytes>(tiff_bytes: &B, index: u64, byte_order: ByteOrder) -> Result<Option<ImageFileDirectoryPointer>, ImageFileDirectoryPointerParseError>
	{
		tiff_bytes.image_file_directory_pointer_version_6(index, byte_order)
	}
	
	#[inline(always)]
	fn long8_offset_or_value<B: Bytes>(tiff_bytes: &B, offset_or_value_union_index: u64, byte_order: ByteOrder) -> Result<u64, TagParseError>
	{
		use TagParseError::*;
		
		let offset = tiff_bytes.offset_version_6(offset_or_value_union_index, byte_order).map_err(OffsetParse)?;
		let index = offset.u64();
		tiff_bytes.unaligned_u64_value(index, byte_order).map_err(Long8Overflow)
	}
}

impl Version6OrBigTiffUnit for u64
{
	type NumberOfDirectoryEntries = u64;
	
	type OffsetLikeValue = u64;
	
	#[inline(always)]
	fn number_of_directory_entries<B: Bytes>(tiff_bytes: &B, index: u64, byte_order: ByteOrder) -> Result<Self::NumberOfDirectoryEntries, OverflowError>
	{
		tiff_bytes.unaligned_u64_value(index, byte_order)
	}
	
	#[inline(always)]
	fn offset_like_value<B: Bytes>(tiff_bytes: &B, index: u64, byte_order: ByteOrder) -> Result<Self::OffsetLikeValue, OverflowError>
	{
		tiff_bytes.unaligned_u64_value(index, byte_order)
	}
	
	#[inline(always)]
	fn offset_like_value_unchecked<B: Bytes>(tiff_bytes: &B, index: u64, byte_order: ByteOrder) -> Self::OffsetLikeValue
	{
		tiff_bytes.unaligned_u64_value_unchecked(index, byte_order)
	}
	
	#[inline(always)]
	fn image_file_directory_pointer<B: Bytes>(tiff_bytes: &B, index: u64, byte_order: ByteOrder) -> Result<Option<ImageFileDirectoryPointer>, ImageFileDirectoryPointerParseError>
	{
		tiff_bytes.image_file_directory_pointer_version_big_tiff(index, byte_order)
	}
	
	#[inline(always)]
	fn long8_offset_or_value<B: Bytes>(tiff_bytes: &B, offset_or_value_union_index: u64, byte_order: ByteOrder) -> Result<u64, TagParseError>
	{
		Ok(Self::offset_like_value_unchecked(tiff_bytes, offset_or_value_union_index, byte_order))
	}
}
