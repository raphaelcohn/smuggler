// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


/// Image file directories (IFDs).
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

impl<A: Allocator + Clone, T: Tag> ImageFileDirectories<A, T>
{
	#[inline(always)]
	pub(in crate::image_file_directory) fn parse<'tiff_bytes, 'allocator, TP: TagParser<'tiff_bytes, 'allocator, A, Tags<A, T>, T>, TB: TiffBytes, Version: 'tiff_bytes + Version6OrBigTiffVersion>(common: &mut TagParserCommon<'tiff_bytes, 'allocator, TB, A, Version>, zeroth_image_file_directory_pointer: ImageFileDirectoryPointer) -> Result<Self, ImageFileDirectoriesParseError>
	{
		use ImageFileDirectoriesParseError::*;
		
		let mut image_file_directories = Vec::new_in(common.allocator());
		let mut image_file_directory_pointer = zeroth_image_file_directory_pointer;
		let mut index = 0;
		loop
		{
			common.guard_image_file_directory_pointer(image_file_directory_pointer)?;
			
			let (image_file_directory, next_image_file_directory_pointer) = ImageFileDirectory::parse::<TP, _, Version>(common, image_file_directory_pointer).map_err(|cause| ImageFileDirectoryParse { cause, image_file_directory_pointer, index })?;
			
			image_file_directories.try_push(image_file_directory).map_err(CouldNotAllocateMemoryForImageFileDirectoryStorage)?;
			
			match next_image_file_directory_pointer
			{
				None => break,
				
				Some(next_image_file_directory_pointer) =>
				{
					image_file_directory_pointer = next_image_file_directory_pointer;
				}
			}
			
			index += 1;
		}
		Ok(Self(image_file_directories))
	}
	
	#[inline(always)]
	fn parse_child_image_file_directories<'tiff_bytes, 'allocator, TP: TagParser<'tiff_bytes, 'allocator, A, Tags<A, T>, T>, TB: TiffBytes, Version: 'tiff_bytes + Version6OrBigTiffVersion>(common: &mut TagParserCommon<'tiff_bytes, 'allocator, TB, A, Version>, raw_tag_value: RawTagValue<'tiff_bytes>) -> Result<Vec<ImageFileDirectories<A, T>, A>, ChildImageFileDirectoriesParseError>
	{
		let offsets = raw_tag_value.unaligned_slice::<_, _, Version, Version>(common);
		let mut vec_image_file_directories = Vec::new_with_capacity(offsets.len(), common.allocator()).map_err(ChildImageFileDirectoriesParseError::CouldNotAllocateMemoryForImageFileDirectories)?;
		for offset in offsets
		{
			let offset = offset.read_assuming_is_native_endian().into();
			Self::parse_child_image_file_directory::<TP, _, Version>(common, &mut vec_image_file_directories, offset)?;
		}
		Ok(vec_image_file_directories)
	}
	
	#[inline(always)]
	fn parse_child_image_file_directory<'tiff_bytes, 'allocator, TP: TagParser<'tiff_bytes, 'allocator, A, Tags<A, T>, T>, TB: TiffBytes, Version: 'tiff_bytes + Version6OrBigTiffVersion>(common: &mut TagParserCommon<'tiff_bytes, 'allocator, TB, A, Version>, vec_image_file_directories: &mut Vec<ImageFileDirectories<A, T>, A>, raw_offset: u64) -> Result<(), ChildImageFileDirectoriesParseError>
	{
		let zeroth_image_file_directory_pointer = Self::zeroth_image_file_directory_pointer(raw_offset)?;
		
		let result = common.recurse(|common| Self::parse::<TP, TB, Version>(common, zeroth_image_file_directory_pointer))?;
		
		match result
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
	fn zeroth_image_file_directory_pointer(raw_offset: u64) -> Result<ImageFileDirectoryPointer, ChildImageFileDirectoriesParseError>
	{
		use ChildImageFileDirectoriesParseError::*;
		
		ImageFileDirectoryPointer::new_from_raw_offset(raw_offset).map_err(ImageFileDirectoryPointerParse)?.ok_or(ImageFileDirectoryPointerIsNull)
	}
	
	#[inline(always)]
	fn box_error(cause: ImageFileDirectoriesParseError) -> Result<(), ChildImageFileDirectoriesParseError>
	{
		use ChildImageFileDirectoriesParseError::*;
		
		let cause = Box::try_new_in(cause, Global).map_err(CouldNotAllocateMemoryForImageFileDirectoriesParseError)?;
		Err(ImageFileDirectoriesParse(cause))
	}
}

impl<'tiff_bytes, A: Allocator + Clone> ImageFileDirectories<A, PublicTag<'tiff_bytes, A>>
{
	#[inline(always)]
	pub(crate) fn parse_public_top_level<'allocator, TB: TiffBytes, Version: 'tiff_bytes + Version6OrBigTiffVersion>(zeroth_image_file_directory_pointer: ImageFileDirectoryPointer, common: &mut TagParserCommon<'tiff_bytes, 'allocator, TB, A, Version>) -> Result<Self, ImageFileDirectoriesParseError>
	{
		Self::parse::<PublicTagParser<'tiff_bytes, A>, _, Version>(common, zeroth_image_file_directory_pointer)
	}
}
