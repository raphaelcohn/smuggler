// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


#[derive(Default, Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
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
	pub(crate) fn parse_public_top_level<TB>(version: Version, zeroth_image_file_directory_pointer: ImageFileDirectoryPointer, tiff_bytes_with_order: TiffBytesWithOrder<TB>, allocator: A) -> Result<Self, ImageFileDirectoriesParseError>
	{
		let recursion = Recursion::default();
		
		let common =
		{
			let recursion_guard = recursion.top_level();
			TagParserCommon::new(tiff_bytes_with_order, &recursion_guard, allocator)
		};
		
		use Version::*;
		
		match version
		{
			_6 => Self::parse::<PublicTagParser, _, u32>(&common, zeroth_image_file_directory_pointer),
			
			BigTiff => Self::parse::<PublicTagParser, _, u64>(&common, zeroth_image_file_directory_pointer),
		}
	}
	
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
	fn parse_child_image_file_directories<'tiff_bytes, 'recursion: 'recursion_guard, 'recursion_guard, TB: TiffBytes, Unit: Version6OrBigTiffUnit, CBU: CanBeUnaligned + Into<u64>>(common: &TagParserCommon<'tiff_bytes, 'recursion, 'recursion_guard, TB, A>, raw_tag_value: RawTagValue<'tiff_bytes>) -> Result<Vec<ImageFileDirectories<A, UnrecognizedTag<'tiff_bytes, A>>, A>, SpecificTagParseError>
	{
		let offsets = CBU::slice_unaligned_and_byte_swap_as_appropriate(raw_tag_value.count, common.tiff_bytes_with_order.byte_order, raw_tag_value.slice);
		let mut vec_image_file_directories = Vec::new_with_capacity(offsets.len(), common.allocator).map_err(SpecificTagParseError::CouldNotAllocateMemoryForImageFileDirectories)?;
		for offset in offsets
		{
			let offset = offset.read_assuming_is_native_endian().into();
			Self::parse_child_image_file_directory::<_, Unit>(common, &mut vec_image_file_directories, offset)?;
		}
		Ok(vec_image_file_directories)
	}
	
	#[inline(always)]
	fn parse_child_image_file_directory<'tiff_bytes, 'recursion: 'recursion_guard, 'recursion_guard, TB: TiffBytes, Unit: Version6OrBigTiffUnit>(common: &TagParserCommon<'tiff_bytes, 'recursion, 'recursion_guard, TB, A>, vec_image_file_directories: &mut Vec<ImageFileDirectories<A, UnrecognizedTag<'tiff_bytes, A>>, A>, offset: u64) -> Result<(), SpecificTagParseError>
	{
		use SpecificTagParseError::*;
		
		let zeroth_image_file_directory_pointer = ImageFileDirectoryPointer::new_from_offset(Offset(offset)).map_err(ImageFileDirectoryPointerParse)?.ok_or(ImageFileDirectoryPointerIsNull)?;
		
		let common =
		{
			let recursion_guard = common.recursion_guard.recurse()?;
			TagParserCommon::new(common.tiff_bytes_with_order, &recursion_guard, common.allocator)
		};
		
		let image_file_directories = match Self::parse::<UnrecognizedTagParser, TB, Unit>(&common, zeroth_image_file_directory_pointer)
		{
			Ok(image_file_directories) => image_file_directories,
			
			Err(cause) =>
			{
				let cause = Box::try_new_in(cause, allocator).map_err(CouldNotAllocateMemoryForImageFileDirectoriesParseError)?;
				Err(ImageFileDirectoriesParse(cause))
			}
		};
		vec_image_file_directories.push_unchecked(image_file_directories);
		Ok(())
	}
}
