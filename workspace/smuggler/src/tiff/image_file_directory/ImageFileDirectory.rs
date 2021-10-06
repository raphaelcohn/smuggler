// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


#[derive(Default, Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ImageFileDirectory<'a, A: Allocator, T: Tag>(Tags<'a, A, T>);

impl<'a, A: Allocator + Copy, T: Tag> ImageFileDirectory<'a, A>
{
	#[inline(always)]
	fn parse<TP: TagParser<Tag=T>, TB: TiffBytes, Unit: Version6OrBigTiffUnit>(tag_parser: TP, recursion_guard: &RecursionGuard, allocator: A, tiff_bytes: &mut TB, byte_order: ByteOrder, image_file_directory_pointer: ImageFileDirectoryPointer) -> Result<(Self, Option<ImageFileDirectoryPointer>), ImageFileDirectoryParseError>
	{
		use ImageFileDirectoryParseError::*;
		
		let index = image_file_directory_pointer.index();
		
		let number_of_directory_entries = Self::number_of_directory_entries::<TB, Unit>(tiff_bytes, byte_order, index)?;
		recursion_guard.record_used_space_slice(index, Unit::NumberOfDirectoryEntriesSize);
		let number_of_directory_entry_bytes = Self::number_of_directory_entry_bytes::<Unit>(number_of_directory_entries)?;
		let directory_entries_index = Self::directory_entries_index::<Unit>(index, number_of_directory_entries)?;
		
		// NOTE: It is important that this is parsed before `Self::parse_directory_entries()`.
		// It ensures there is enough data and allows a large number of OverflowError checks to be optimized away.
		let next_image_file_directory_pointer = Self::next_image_file_directory_pointer(tiff_bytes, byte_order, directory_entries_index, number_of_directory_entry_bytes, recursion_guard)?;
		
		let this = Self::parse_directory_entries::<TP, TB, Unit>(tag_parser, recursion_guard, allocator, tiff_bytes, byte_order, directory_entries_index, number_of_directory_entries, number_of_directory_entry_bytes)?;
		
		Ok((this, next_image_file_directory_pointer))
	}
	
	#[inline(always)]
	fn number_of_directory_entries<TB: TiffBytes, Unit: Version6OrBigTiffUnit>(tiff_bytes: &TB, byte_order: ByteOrder, index: Index) -> Result<NonZeroU64, ImageFileDirectoryParseError>
	{
		use ImageFileDirectoryParseError::*;
		
		let raw_number_of_directory_entries = match Unit::number_of_directory_entries(tiff_bytes, index, byte_order)
		{
			Ok(raw_number_of_directory_entries) => raw_number_of_directory_entries.into(),
			
			Err(cause) => return Err(NotEnoughBytesForNumberOfDirectoryElements(cause))
		};
		
		if unlikely!(number_of_directory_entries == 0)
		{
			return Err(ThereAreNoDirectoryEntries)
		}
		
		Ok(new_non_zero_u64(raw_number_of_directory_entries))
	}
	
	#[inline(always)]
	fn number_of_directory_entry_bytes<Unit: Version6OrBigTiffUnit>(number_of_directory_entries: NonZeroU64) -> Result<NonZeroU64, ImageFileDirectoryParseError>
	{
		match number_of_directory_entries.get().checked_mul(Self::SizeOfEntry::<Unit>())
		{
			Some(raw_number_of_directory_entry_bytes) => Ok(new_non_zero_u64(raw_number_of_directory_entry_bytes)),
			
			None => Err(ImageFileDirectoryParseError::NumberOfDirectoryEntryBytesOverflows { number_of_directory_entries }),
		}
	}
	
	#[inline(always)]
	fn directory_entries_index<Unit: Version6OrBigTiffUnit>(index: Index, number_of_directory_entries: NonZeroU64) -> Result<u64, ImageFileDirectoryParseError>
	{
		let size = size_of_u64::<Unit::NumberOfDirectoryEntries>();
		index.checked_add(size).ok_or(ImageFileDirectoryParseError::DirectoryEntriesArrayStartIndexWouldOverflow { number_of_directory_entries, })
	}
	
	#[inline(always)]
	fn next_image_file_directory_pointer<TB: TiffBytes, Unit: Version6OrBigTiffUnit>(tiff_bytes: &TB, byte_order: ByteOrder, directory_entries_index: Index, number_of_directory_entry_bytes: NonZeroU64, recursion_guard: &RecursionGuard) -> Result<Option<ImageFileDirectoryPointer>, ImageFileDirectoryParseError>
	{
		let image_file_directory_pointer_index = Self::image_file_directory_pointer_index(directory_entries_index, number_of_directory_entry_bytes)?;
		let image_file_directory_pointer = Unit::image_file_directory_pointer(tiff_bytes, image_file_directory_pointer_index, byte_order).map_err(ImageFileDirectoryParseError::NextImageFileDirectoryPointerParse)?;
		
		recursion_guard.record_used_space_slice(image_file_directory_pointer_index, Unit::ImageFileDirectoryPointerSize);
		
		Ok(image_file_directory_pointer)
	}
	
	#[inline(always)]
	fn image_file_directory_pointer_index(directory_entries_index: Index, number_of_directory_entry_bytes: NonZeroU64) -> Result<Index, ImageFileDirectoryParseError>
	{
		directory_entries_index.checked_add(number_of_directory_entry_bytes.get()).ok_or(ImageFileDirectoryParseError::NextImageFileDirectoryPointerStartIndexWouldOverflow { number_of_directory_entry_bytes })
	}
	
	#[inline(always)]
	fn parse_directory_entries<TP: TagParser<Tag=T>, TB: TiffBytes, Unit: Version6OrBigTiffUnit>(tag_parser: TP, recursion_guard: &RecursionGuard, allocator: A, tiff_bytes: &'a mut TB, byte_order: ByteOrder, directory_entries_index: Index, number_of_directory_entries: NonZeroU64, number_of_directory_entry_bytes: NonZeroU64) -> Result<Self, ImageFileDirectoryParseError>
	{
		let mut tags = Tags::new(number_of_directory_entries, allocator).map_err(ImageFileDirectoryParseError::CouldNotAllocateMemoryForDirectoryEntries)?;
		let mut directory_entry_index = directory_entries_index;
		let mut previous_tag_identifier = None;
		let last_directory_entry_index = number_of_directory_entry_bytes.get() - Self::SizeOfEntry::<Unit>();
		loop
		{
			let tag = Self::parse_directory_entry_unchecked::<TP, TB, Unit>(tag_parser, recursion_guard, allocator, tiff_bytes, byte_order, directory_entry_index, &mut previous_tag_identifier)?;
			tags.push_unchecked_for_duplicates_or_sort(tag);
			
			if unlikely!(directory_entry_index == last_directory_entry_index)
			{
				break
			}
			directory_entry_index += Self::SizeOfEntry::<Unit>()
		}
		Ok(Self(tags))
	}
	
	#[inline(always)]
	fn parse_directory_entry_unchecked<TP: TagParser<Tag=T>, TB: TiffBytes, Unit: Version6OrBigTiffUnit>(tag_parser: TP, recursion_guard: &RecursionGuard, allocator: A, tiff_bytes: &'a mut TB, byte_order: ByteOrder, directory_entry_index: Index, previous_tag_identifier: &mut Option<u16>) -> Result<DirectoryEntry<'a, A>, TagParseError>
	{
		let tag_identifier = Self::tag_identifier(tiff_bytes, byte_order, directory_entry_index, previous_tag_identifier)?;
		let (tag_type, tag_type_size_in_bytes) = TagType::parse(Self::value_unchecked_u16(tiff_bytes, byte_order, directory_entry_index, Self::SizeOfTag))?;
		let count = Self::value_unchecked(tiff_bytes, byte_order, directory_entry_index, Self::SizeOfFixedEntry, Unit::offset_like_value_unchecked);
		let offset_or_value_union_index = directory_entry_index + Self::SizeOfEntryUptoCount::<Unit>();
		
		Self::parse_directory_entry_contents::<_, _, Unit>(tag_parser, recursion_guard, allocator, tiff_bytes, byte_order, tag_identifier, tag_type, tag_type_size_in_bytes, count, offset_or_value_union_index).map_err(|cause| TagParseError::SpecificTagParse { cause, tag_identifier, tag_type, count, offset_or_value_union_index })
	}
	
	#[inline(always)]
	fn parse_directory_entry_contents<TP: TagParser<Tag=T>, TB: TiffBytes, Unit: Version6OrBigTiffUnit>(tag_parser: TP, recursion_guard: &RecursionGuard, allocator: A, tiff_bytes: &'a mut TB, byte_order: ByteOrder, tag_identifier: TagIdentifier, tag_type: TagType, tag_type_size_in_bytes: u64, count: u64, offset_or_value_union_index: Index) -> Result<DirectoryEntry<'a, A>, SpecificTagParseError>
	{
		use SpecificTagParseError::*;
		
		let slice =
		{
			let slice_size_in_bytes = tag_type_size_in_bytes.checked_mul(count).ok_or(CountIsTooLargeForTargetArchitecture)?;
			let (index, non_null) = if slice_size_in_bytes <= Unit::OffsetOrValueUnionSize
			{
				(offset_or_value_union_index, tiff_bytes.non_null_to_index_unchecked_mut::<u8>(offset_or_value_union_index, slice_size_in_bytes))
			}
			else
			{
				let index = Offset::parse_offset_value(tiff_bytes, offset_or_value_union_index).map_err(SliceOffsetParse)?.index();
				(index, tiff_bytes.non_null_to_index_checked_mut::<u8>(index, slice_size_in_bytes).map_err(OffsetIsTooLargeForTargetArchitecture)?)
			};
			recursion_guard.record_used_space_slice(index, slice_size_in_bytes);
			NonNull::slice_from_raw_parts(non_null, slice_size_in_bytes as usize)
		};
		
		tag_parser.parse::<A, Unit, TB>(recursion_guard, allocator, tiff_bytes, tag_identifier, tag_type, count, byte_order, slice)
	}
	
	#[inline(always)]
	fn tag_identifier(tiff_bytes: &impl TiffBytes, byte_order: ByteOrder, directory_entry_index: Index, previous_tag_identifier: &mut Option<TagIdentifier>) -> Result<TagIdentifier, TagParseError>
	{
		let tag_identifier = Self::value_unchecked_u16(tiff_bytes, byte_order, directory_entry_index, 0);
		let previous_tag_identifier = previous_tag_identifier.replace(tag_identifier);
		if let Some(previous_tag_identifier) = previous_tag_identifier
		{
			if unlikely!(previous_tag_identifier >= tag_identifier)
			{
				return Err(TagParseError::OutOfSequenceTagIdentifier { tag_identifier, previous_tag_identifier })
			}
		}
		Ok(tag_identifier)
	}
	
	#[inline(always)]
	fn value_unchecked_u16<TB: TiffBytes>(tiff_bytes: &impl TB, byte_order: ByteOrder, directory_entry_index: Index, offset: u64) -> u16
	{
		Self::value_unchecked::<u16, _>(tiff_bytes, byte_order, directory_entry_index, offset, TB::unaligned_unchecked::<u16>)
	}
	
	#[inline(always)]
	fn value_unchecked<Value, TB: TiffBytes>(tiff_bytes: &impl TB, byte_order: ByteOrder, directory_entry_index: Index, offset: u64, callback: impl FnOnce(&TB, u64, ByteOrder) -> Value) -> Value
	{
		callback(tiff_bytes, directory_entry_index + offset, byte_order)
	}
	
	#[inline(always)]
	const fn SizeOfEntry<Unit>() -> u64
	{
		Self::SizeOfEntryUptoCount::<Unit>() + Self::SizeOfOffsetOrValueUnion::<Unit>()
	}
	
	#[inline(always)]
	const fn SizeOfEntryUptoCount<Unit>() -> u64
	{
		Self::SizeOfFixedEntry + Self::SizeOfCount::<Unit>()
	}
	
	const SizeOfFixedEntry: u64 = Self::SizeOfTag + Self::SizeOfType;
	
	#[inline(always)]
	const fn SizeOfCount<Unit>() -> u64
	{
		Self::SizeOfUnit::<Unit>()
	}
	
	#[inline(always)]
	const fn SizeOfOffsetOrValueUnion<Unit>() -> u64
	{
		Self::SizeOfUnit::<Unit>()
	}
	
	#[inline(always)]
	const fn SizeOfUnit<Unit>() -> u64
	{
		size_of_u64::<Unit>()
	}
	
	const SizeOfTag: u64 = size_of_u64::<u16>();
	
	const SizeOfType: u64 = size_of_u64::<u16>();
}
