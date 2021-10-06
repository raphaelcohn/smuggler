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
		use Version::*;
		
		let byte_order = header.byte_order;
		let tag_parser = PublicTagParser::default();
		match header.version
		{
			_6 => Self::parse::<PublicTagParser, TB, u32>(tag_parser, allocator, tiff_bytes, byte_order, zeroth_image_file_directory_pointer),
			
			BigTiff => Self::parse::<PublicTagParser, TB, u64>(tag_parser, allocator, tiff_bytes, byte_order, zeroth_image_file_directory_pointer),
		}
	}
	
	#[inline(always)]
	pub(crate) fn parse<TP: TagParser<Tag=T>, TB: TiffBytes, Unit: Version6OrBigTiffUnit>(tag_parser: TP, allocator: A, tiff_bytes: &'a mut TB, byte_order: ByteOrder, zeroth_image_file_directory_pointer: ImageFileDirectoryPointer) -> Result<Self, ImageFileDirectoriesParseError>
	{
		use ImageFileDirectoriesParseError::*;
		
		let mut image_file_directories = Vec::new_in(allocator);
		let mut image_file_directory_pointer = zeroth_image_file_directory_pointer;
		let mut index = 0;
		loop
		{
			let (image_file_directory, next_image_file_directory_pointer) = ImageFileDirectory::parse::<TP, TB, Unit>(tag_parser, allocator, tiff_bytes, byte_order, image_file_directory_pointer).map_err(|cause| ImageFileDirectoryParse { cause, image_file_directory_pointer, index })?;
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
}
