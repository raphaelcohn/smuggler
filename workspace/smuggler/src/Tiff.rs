// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


/// A TIFF.
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Tiff<'tiff_bytes, A: Allocator>
{
	#[allow(missing_docs)]
	pub byte_order: ByteOrder,
	
	#[allow(missing_docs)]
	pub version: Version,
	
	#[allow(missing_docs)]
	pub image_file_directories: ImageFileDirectories<A, PublicTag<'tiff_bytes, A>>,
	
	#[allow(missing_docs)]
	pub free_space: FreeSpace<A>,
}

impl<'tiff_bytes, A: Allocator + Clone> Tiff<'tiff_bytes, A>
{
	/// Parse the bytes of a TIFF file.
	pub fn parse<'allocator, TB: TiffBytes>(tiff_bytes: &'tiff_bytes mut TB, allocator: &'allocator A) -> Result<Self, TiffParseError>
	{
		let byte_order = parse_header_byte_order(tiff_bytes)?;
		let tiff_bytes_with_order = TiffBytesWithOrder::new(tiff_bytes, byte_order);
		Self::parse_header_version_and_remainder_of_file(tiff_bytes_with_order, allocator)
	}
	
	#[inline(always)]
	fn parse_header_version_and_remainder_of_file<'allocator, TB: TiffBytes>(tiff_bytes_with_order: TiffBytesWithOrder<'tiff_bytes, TB>, allocator: &'allocator A) -> Result<Self, TiffParseError>
	{
		use HeaderParseError::VersionParse;
		use VersionParseError::*;
		
		#[inline(always)]
		fn new_tag_parser_common<'tiff_bytes, 'allocator, TB: TiffBytes, A: Allocator + Clone, Version: Version6OrBigTiffVersion>(tiff_bytes_with_order: TiffBytesWithOrder<'tiff_bytes, TB>, allocator: &'allocator A) -> Result<TagParserCommon<'tiff_bytes, 'allocator, TB, A, Version>, TiffParseError>
		{
			TagParserCommon::new(tiff_bytes_with_order, allocator).map_err(TiffParseError::OutOfMemoryCreatingTagParserCommon)
		}
		
		match tiff_bytes_with_order.unaligned_checked(2).map_err(|cause| VersionParse(TooFewBytesForVersion(cause)))?
		{
			Version6::U16 => Self::parse_remainder_of_file::<_, Version6>(new_tag_parser_common(tiff_bytes_with_order, allocator)?),
			
			VersionBigTiff::U16 => Self::parse_remainder_of_file::<_, VersionBigTiff>(new_tag_parser_common(tiff_bytes_with_order, allocator)?),
			
			version @ _ => Err(VersionParse(UnknownVersion { version }))?
		}
	}
	
	#[inline(always)]
	fn parse_remainder_of_file<'allocator, TB: TiffBytes, Version: 'tiff_bytes + Version6OrBigTiffVersion>(mut common: TagParserCommon<'tiff_bytes, 'allocator, TB, A, Version>) -> Result<Self, TiffParseError>
	{
		let zeroth_image_file_directory_pointer = Self::parse_remainder_of_header(&mut common)?;
		
		let image_file_directories = ImageFileDirectories::parse_public_top_level(zeroth_image_file_directory_pointer, &mut common)?;
		
		let (byte_order, free_space) = common.finish();
		Ok
		(
			Self
			{
				byte_order,
				
				version: Version::Version,
				
				image_file_directories,
			
				free_space,
			}
		)
	}
	
	#[inline(always)]
	fn parse_remainder_of_header<'allocator, TB: TiffBytes, Version: 'tiff_bytes + Version6OrBigTiffVersion>(common: &mut TagParserCommon<'tiff_bytes, 'allocator, TB, A, Version>) -> Result<ImageFileDirectoryPointer, TiffParseError>
	{
		common.record_used_space_slice(0, Version::HeaderSizeInBytes);
		let zeroth_image_file_directory_pointer = parse_header_zeroth_image_file_directory_pointer::<TB, Version>(&common)?;
		Ok(zeroth_image_file_directory_pointer)
	}
}

