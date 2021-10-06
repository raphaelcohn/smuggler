// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


#[derive(Default, Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ImageFileDirectories<'a, A: Allocator, T: Tag>(Vec<ImageFileDirectory<'a, A, T>, A>);

impl<'a, A: Allocator, T: Tag> Deref for ImageFileDirectories<'a, A, T>
{
	type Target = [ImageFileDirectory<'a, A>];
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0
	}
}

impl<'a, A: Allocator, T: Tag> ImageFileDirectories<'a, A, T>
{
	#[inline(always)]
	pub(crate) fn parse_public_top_level<TB: TiffBytes>(allocator: A, tiff_bytes: &'a mut TB, header: Header, zeroth_image_file_directory_pointer: ImageFileDirectoryPointer) -> Result<Self, ImageFileDirectoriesParseError>
	{
		let recursion = Recursion::default();
		let recursion_guard = recursion.top_level().map_err(|cause| TagParseError::SpecificTagParse { cause, tag_identifier, tag_type, count, offset_or_value_union_index })?;
		
		use Version::*;
		
		let byte_order = header.byte_order;
		let tag_parser = PublicTagParser;
		match header.version
		{
			_6 => Self::parse::<PublicTagParser, TB, u32>(tag_parser, recursion_guard, allocator, tiff_bytes, byte_order, zeroth_image_file_directory_pointer),
			
			BigTiff => Self::parse::<PublicTagParser, TB, u64>(tag_parser, recursion_guard, allocator, tiff_bytes, byte_order, zeroth_image_file_directory_pointer),
		}
	}
	
	#[inline(always)]
	pub(crate) fn parse<TP: TagParser<Tag=T>, TB: TiffBytes, Unit: Version6OrBigTiffUnit>(tag_parser: TP, mut recursion_guard: RecursionGuard, allocator: A, tiff_bytes: &'a mut TB, byte_order: ByteOrder, zeroth_image_file_directory_pointer: ImageFileDirectoryPointer) -> Result<Self, ImageFileDirectoriesParseError>
	{
		use ImageFileDirectoriesParseError::*;
		
		let mut image_file_directories = Vec::new_in(allocator);
		let mut image_file_directory_pointer = zeroth_image_file_directory_pointer;
		let mut index = 0;
		loop
		{
			recursion_guard.guard_image_file_directory_pointer(image_file_directory_pointer);
			let (image_file_directory, next_image_file_directory_pointer) = ImageFileDirectory::parse::<TP, TB, Unit>(tag_parser, &recursion_guard, allocator, tiff_bytes, byte_order, image_file_directory_pointer).map_err(|cause| ImageFileDirectoryParse { cause, image_file_directory_pointer, index })?;
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
	fn parse_child_image_file_directories<Unit: Version6OrBigTiffUnit, TB: TiffBytes, CBU: CanBeUnaligned + Into<u64>>(tiff_bytes: &mut TB, count: u64, byte_order: ByteOrder, slice: NonNull<[u8]>, recursion_guard: &RecursionGuard, allocator: A) -> Result<Vec<ImageFileDirectories<'a, A, UnrecognizedTag<'a, A>>, A>, SpecificTagParseError>
	{
		let offsets = CBU::slice_unaligned_and_byte_swap_as_appropriate(count, byte_order, slice);
		let mut vec_image_file_directories = Vec::new_with_capacity(offsets.len(), allocator).map_err(SpecificTagParseError::CouldNotAllocateMemoryForImageFileDirectories)?;
		for offset in offsets
		{
			let offset = offset.read_assuming_is_native_endian().into();
			Self::parse_child_image_file_directory::<Unit, TB>(tiff_bytes, byte_order, recursion_guard, allocator, &mut vec_image_file_directories, offset)?;
		}
		Ok(vec_image_file_directories)
	}
	
	#[inline(always)]
	fn parse_child_image_file_directory<Unit: Version6OrBigTiffUnit, TB: TiffBytes>(tiff_bytes: &mut TB, byte_order: ByteOrder, recursion_guard: &RecursionGuard, allocator: A, vec_image_file_directories: &mut Vec<ImageFileDirectories<'a, A, UnrecognizedTag<'a, A>>>, offset: u64) -> Result<(), SpecificTagParseError>
	{
		use SpecificTagParseError::*;
		
		let zeroth_image_file_directory_pointer = ImageFileDirectoryPointer::new_from_offset(Offset(offset)).map_err(ImageFileDirectoryPointerParse)?.ok_or(ImageFileDirectoryPointerIsNull)?;
		
		let recursion_guard = recursion_guard.recurse()?;
		let image_file_directories = match ImageFileDirectories::parse::<_, _, Unit>(UnrecognizedTagParser, recursion_guard, allocator, tiff_bytes, byte_order, zeroth_image_file_directory_pointer)
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
