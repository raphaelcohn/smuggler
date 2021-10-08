// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ImageFileDirectories<A: Allocator, T: Tag>(Vec<ImageFileDirectory<A, T>, A>);

impl<A: Allocator, T: Tag> Deref for ImageFileDirectories<A, T>
{
	type Target = [ImageFileDirectory<A, T>];
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0
	}
}

impl<A: Allocator + Copy, T: Tag> ImageFileDirectories<A, T>
{
	#[inline(always)]
	pub(crate) fn parse<'tiff_bytes, 'recursion, 'recursion_guard, TP: TagParser<'tiff_bytes, 'recursion, 'recursion_guard, A, Tags<A, T>, T>, TB: TiffBytes, Unit: Version6OrBigTiffUnit>(common: &TagParserCommon<'tiff_bytes, 'recursion, 'recursion_guard, TB, A>, zeroth_image_file_directory_pointer: ImageFileDirectoryPointer) -> Result<Self, ImageFileDirectoriesParseError>
	{
		use ImageFileDirectoriesParseError::*;
		
		let mut image_file_directories = Vec::new_in(common.allocator);
		let mut image_file_directory_pointer = zeroth_image_file_directory_pointer;
		let mut index = 0;
		loop
		{
			common.recursion_guard.guard_image_file_directory_pointer(image_file_directory_pointer);
			
			let tag_parser = TP::default();
			let (image_file_directory, next_image_file_directory_pointer) = ImageFileDirectory::parse::<_, _, Unit>(tag_parser, common, image_file_directory_pointer).map_err(|cause| ImageFileDirectoryParse { cause, image_file_directory_pointer, index })?;
			
			image_file_directories.try_push(image_file_directory).map_err(|cause| CouldNotAllocateMemoryForImageFileDirectoryStorage)?;
			
			match next_image_file_directory_pointer
			{
				None => break,
				
				Some(next_image_file_directory_pointer) =>
				{
					image_file_directory_pointer = next_image_file_directory_pointer;
				}
			}
		}
		Ok(Self(image_file_directories))
	}
	
	#[inline(always)]
	fn parse_child_image_file_directories<'tiff_bytes, 'recursion: 'recursion_guard, 'recursion_guard, TP: TagParser<'tiff_bytes, 'recursion, 'recursion_guard, A, Tags<A, T>, T>, TB: TiffBytes, Unit: Version6OrBigTiffUnit>(common: &TagParserCommon<'tiff_bytes, 'recursion, 'recursion_guard, TB, A>, raw_tag_value: RawTagValue<'tiff_bytes>) -> Result<Vec<ImageFileDirectories<A, T>, A>, SpecificTagParseError>
	{
		let offsets = Unit::slice_unaligned_and_byte_swap_as_appropriate(raw_tag_value.count, common.tiff_bytes_with_order.byte_order, raw_tag_value.slice);
		let mut vec_image_file_directories = Vec::new_with_capacity(offsets.len(), common.allocator).map_err(SpecificTagParseError::CouldNotAllocateMemoryForImageFileDirectories)?;
		for offset in offsets
		{
			let offset = offset.read_assuming_is_native_endian().into();
			Self::parse_child_image_file_directory::<TP, _, Unit>(common, &mut vec_image_file_directories, offset)?;
		}
		Ok(vec_image_file_directories)
	}
	
	#[inline(always)]
	fn parse_child_image_file_directory<'tiff_bytes, 'recursion: 'recursion_guard, 'recursion_guard, TP: TagParser<'tiff_bytes, 'recursion, 'recursion_guard, A, Tags<A, T>, T>, TB: TiffBytes, Unit: Version6OrBigTiffUnit>(common: &TagParserCommon<'tiff_bytes, 'recursion, 'recursion_guard, TB, A>, vec_image_file_directories: &mut Vec<ImageFileDirectories<A, T>, A>, raw_offset: u64) -> Result<(), SpecificTagParseError>
	{
		let zeroth_image_file_directory_pointer = Self::zeroth_image_file_directory_pointer(raw_offset)?;
		
		let common = common.recurse()?;
		
		match Self::parse::<TP, TB, Unit>(&common, zeroth_image_file_directory_pointer)
		{
			Err(cause) => Self::box_error(cause),
			
			Ok(image_file_directories) =>
			{
				vec_image_file_directories.push_unchecked(image_file_directories);
				Ok(())
			}
		}
	}
	
	#[inline(always)]
	fn zeroth_image_file_directory_pointer(raw_offset: u64) -> Result<ImageFileDirectoryPointer, SpecificTagParseError>
	{
		use SpecificTagParseError::*;
		
		ImageFileDirectoryPointer::new_from_raw_offset(raw_offset).map_err(ImageFileDirectoryPointerParse)?.ok_or(ImageFileDirectoryPointerIsNull)
	}
	
	#[inline(always)]
	fn box_error(cause: ImageFileDirectoriesParseError) -> Result<(), SpecificTagParseError>
	{
		use SpecificTagParseError::*;
		
		let cause = Box::try_new_in(cause, Global).map_err(CouldNotAllocateMemoryForImageFileDirectoriesParseError)?;
		Err(ImageFileDirectoriesParse(cause))
	}
}

impl<'tiff_bytes, A: Allocator + Copy> ImageFileDirectories<A, PublicTag<'tiff_bytes, A>>
{
	#[inline(always)]
	pub(crate) fn parse_public_top_level<TB: TiffBytes>(version: Version, zeroth_image_file_directory_pointer: ImageFileDirectoryPointer, tiff_bytes_with_order: TiffBytesWithOrder<'tiff_bytes, TB>, allocator: A) -> Result<Self, ImageFileDirectoriesParseError>
	{
		let recursion = Recursion::new();
		
		let common =
		{
			let recursion_guard = recursion.top_level();
			TagParserCommon::new(tiff_bytes_with_order, &recursion_guard, allocator)
		};
		
		use Version::*;
		
		match version
		{
			_6 => Self::parse::<PublicTagParser<'tiff_bytes, A>, _, u32>(&common, zeroth_image_file_directory_pointer),
			
			BigTiff => Self::parse::<PublicTagParser<'tiff_bytes, A>, _, u64>(&common, zeroth_image_file_directory_pointer),
		}
	}
}
