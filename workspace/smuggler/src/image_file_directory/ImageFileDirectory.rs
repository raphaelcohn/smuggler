// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


/// Image File Directory (IFD).
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ImageFileDirectory<A: Allocator, T: Tag>(Tags<A, T>);

impl<A: Allocator + Clone, T: Tag> ImageFileDirectory<A, T>
{
	#[inline(always)]
	fn parse<'tiff_bytes, 'allocator, TP: TagParser<'tiff_bytes, 'allocator, A, Tags<A, T>, T>, TB: TiffBytes, Unit: 'tiff_bytes + Version6OrBigTiffUnit>(common: &mut TagParserCommon<'tiff_bytes, 'allocator, TB, A, Unit>, image_file_directory_pointer: ImageFileDirectoryPointer) -> Result<(Self, Option<ImageFileDirectoryPointer>), ImageFileDirectoryParseError>
	{
		let index = image_file_directory_pointer.index();
		
		let number_of_directory_entries = Self::number_of_directory_entries::<_, Unit>(&common, index)?;
		common.record_used_space_slice(index, size_of_u64::<Unit::NumberOfDirectoryEntries>());
		let number_of_directory_entry_bytes = Self::number_of_directory_entry_bytes::<Unit>(number_of_directory_entries)?;
		let directory_entries_index = Self::directory_entries_index::<Unit>(index, number_of_directory_entries)?;
		
		// NOTE: It is important that this is parsed before `Self::parse_directory_entries()`.
		// It ensures there is enough data and allows a large number of OverflowError checks to be optimized away.
		let next_image_file_directory_pointer = Self::next_image_file_directory_pointer::<_, Unit>(common, directory_entries_index, number_of_directory_entry_bytes, )?;
		
		let this = Self::parse_directory_entries::<TP, _, Unit>( common, directory_entries_index, number_of_directory_entries, number_of_directory_entry_bytes)?;
		
		Ok((this, next_image_file_directory_pointer))
	}
	
	#[inline(always)]
	fn number_of_directory_entries<TB: TiffBytes, Unit: Version6OrBigTiffUnit>(tiff_bytes_with_order: &TiffBytesWithOrder<TB>, index: Index) -> Result<NonZeroU64, ImageFileDirectoryParseError>
	{
		use ImageFileDirectoryParseError::*;
		
		let raw_number_of_directory_entries = match Unit::number_of_directory_entries(tiff_bytes_with_order, index)
		{
			Ok(raw_number_of_directory_entries) => raw_number_of_directory_entries.into(),
			
			Err(cause) => return Err(NotEnoughBytesForNumberOfDirectoryElements(cause))
		};
		
		if unlikely!(raw_number_of_directory_entries == 0)
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
	fn next_image_file_directory_pointer<TB: TiffBytes, Unit: Version6OrBigTiffUnit>(common: &mut TagParserCommon<TB, A, Unit>, directory_entries_index: Index, number_of_directory_entry_bytes: NonZeroU64) -> Result<Option<ImageFileDirectoryPointer>, ImageFileDirectoryParseError>
	{
		let image_file_directory_pointer_index = Self::image_file_directory_pointer_index(directory_entries_index, number_of_directory_entry_bytes)?;
		let image_file_directory_pointer = common.image_file_directory_pointer::<Unit>(image_file_directory_pointer_index).map_err(ImageFileDirectoryParseError::NextImageFileDirectoryPointerParse)?;
		
		common.record_used_space_slice(image_file_directory_pointer_index, size_of_u64::<Unit>());
		
		Ok(image_file_directory_pointer)
	}
	
	#[inline(always)]
	fn image_file_directory_pointer_index(directory_entries_index: Index, number_of_directory_entry_bytes: NonZeroU64) -> Result<Index, ImageFileDirectoryParseError>
	{
		directory_entries_index.checked_add(number_of_directory_entry_bytes.get()).ok_or(ImageFileDirectoryParseError::NextImageFileDirectoryPointerStartIndexWouldOverflow { number_of_directory_entry_bytes })
	}
	
	#[inline(always)]
	fn parse_directory_entries<'tiff_bytes, 'allocator, TP: TagParser<'tiff_bytes, 'allocator, A, Tags<A, T>, T>, TB: TiffBytes, Unit: 'tiff_bytes + Version6OrBigTiffUnit>(common: &mut TagParserCommon<'tiff_bytes, 'allocator, TB, A, Unit>, directory_entries_index: Index, number_of_directory_entries: NonZeroU64, number_of_directory_entry_bytes: NonZeroU64) -> Result<Self, ImageFileDirectoryParseError>
	{
		let mut tag_parser = TP::default();
		let mut tags = Tags::new(number_of_directory_entries, common.allocator()).map_err(ImageFileDirectoryParseError::CouldNotAllocateMemoryForDirectoryEntries)?;
		let mut directory_entry_index = directory_entries_index;
		let mut previous_tag_identifier = None;
		let last_directory_entry_index = number_of_directory_entry_bytes.get() - Self::SizeOfEntry::<Unit>();
		loop
		{
			Self::parse_directory_entry::<_, _, Unit>(&mut tag_parser, common, &mut tags, directory_entry_index, &mut previous_tag_identifier)?;
			
			if unlikely!(directory_entry_index == last_directory_entry_index)
			{
				break
			}
			directory_entry_index += Self::SizeOfEntry::<Unit>()
		}
		
		if let Err(cause) = tag_parser.finish::<_, Unit>(common, &mut tags)
		{
			return Err(ImageFileDirectoryParseError::from(cause.into()))
		}
		
		Ok(Self(tags))
	}
	
	#[inline(always)]
	fn parse_directory_entry<'tiff_bytes, 'allocator, TP: TagParser<'tiff_bytes, 'allocator, A, Tags<A, T>, T>, TB: TiffBytes, Unit: 'tiff_bytes + Version6OrBigTiffUnit>(tag_parser: &mut TP, common: &mut TagParserCommon<'tiff_bytes, 'allocator, TB, A, Unit>, tag_event_handler: &mut Tags<A, T>, directory_entry_index: Index, previous_tag_identifier: &mut Option<u16>) -> Result<(), TagParseError>
	{
		let tag_identifier = Self::tag_identifier(&common, directory_entry_index, previous_tag_identifier)?;
		let (tag_type, tag_type_size_in_bytes) = TagType::parse(Self::value_unchecked_u16(&common, directory_entry_index, Self::SizeOfTag))?;
		let count = Self::value_unchecked::<Unit, TB, _>(&common, directory_entry_index, Self::SizeOfFixedEntry, TiffBytesWithOrder::<TB>::unaligned_unchecked::<Unit>).into();
		let offset_or_value_union_index = directory_entry_index + Self::SizeOfEntryUptoCount::<Unit>();
		
		tag_parser.parse_tag::<TB, Unit>(common, tag_event_handler, tag_identifier, tag_type, tag_type_size_in_bytes, count, offset_or_value_union_index).map_err(|cause| TagParseError::SpecificTagParse { cause, tag_identifier, tag_type, count, offset_or_value_union_index })
	}
	
	#[inline(always)]
	fn tag_identifier<TB: TiffBytes>(tiff_bytes_with_order: &TiffBytesWithOrder<TB>, directory_entry_index: Index, previous_tag_identifier: &mut Option<TagIdentifier>) -> Result<TagIdentifier, TagParseError>
	{
		let tag_identifier = Self::value_unchecked_u16(tiff_bytes_with_order, directory_entry_index, 0);
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
	fn value_unchecked_u16<'tiff_bytes, TB: TiffBytes>(tiff_bytes_with_order: &TiffBytesWithOrder<'tiff_bytes, TB>, directory_entry_index: Index, relative_offset: u64) -> u16
	{
		Self::value_unchecked::<u16, TB, _>(tiff_bytes_with_order, directory_entry_index, relative_offset, TiffBytesWithOrder::<'tiff_bytes, TB>::unaligned_unchecked::<u16>)
	}
	
	#[inline(always)]
	fn value_unchecked<'tiff_bytes, Value, TB: TiffBytes, Callback: FnOnce(&TiffBytesWithOrder<'tiff_bytes, TB>, Index) -> Value>(tiff_bytes_with_order: &TiffBytesWithOrder<'tiff_bytes, TB>, directory_entry_index: Index, relative_offset: u64, callback: Callback) -> Value
	{
		callback(tiff_bytes_with_order, directory_entry_index + relative_offset)
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
