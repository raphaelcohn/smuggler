// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


#[derive(Debug)]
pub(in crate::image_file_directory) struct RawTagValue<'tiff_bytes>
{
	pub(in crate::image_file_directory) count: u64,
	
	pub(in crate::image_file_directory) slice: NonNull<[u8]>,

	marker: PhantomData<&'tiff_bytes ()>,
}

impl<'tiff_bytes> RawTagValue<'tiff_bytes>
{
	#[inline(always)]
	pub(in crate::image_file_directory) fn byte_slice<B: Byte>(self) -> &'tiff_bytes [B]
	{
		B::byte_slice(self.slice)
	}
	
	#[inline(always)]
	pub(in crate::image_file_directory) fn unaligned_slice<'allocator, TB: TiffBytes, A: Allocator + Clone, Version: Version6OrBigTiffVersion, CBU: CanBeUnaligned>(self, common: &mut TagParserCommon<'tiff_bytes, 'allocator, TB, A, Version>) -> &'tiff_bytes [Unaligned<CBU>]
	{
		CBU::slice_unaligned_and_byte_swap_as_appropriate(self.count, common.byte_order(), self.slice)
	}
	
	#[inline(always)]
	pub(in crate::image_file_directory) fn ascii_strings<'allocator, TB: TiffBytes, A: Allocator + Clone, Version: Version6OrBigTiffVersion>(self, common: &mut TagParserCommon<'tiff_bytes, 'allocator, TB, A, Version>) -> Result<AsciiStrings<'tiff_bytes, A>, AsciiStringsParseError>
	{
		AsciiStrings::parse(common.allocator(), self.byte_slice())
	}
	
	#[inline(always)]
	pub(in crate::image_file_directory) fn bitfield_integer<'allocator, TB: TiffBytes, A: Allocator + Clone, Version: Version6OrBigTiffVersion, UINT: UnsignedIntegerNormalizedType, BF: BitField>(self, common: &mut TagParserCommon<'tiff_bytes, 'allocator, TB, A, Version>, tag_type: TagType) -> Result<BitFieldInteger<UINT, BF>, IntegerValueParseError>
	{
		self.unsigned_integer_value(common, tag_type).map(BitFieldInteger::from)
	}
	
	#[inline(always)]
	pub(in crate::image_file_directory) fn enum_unsigned_integer<'allocator, TB: TiffBytes, A: Allocator + Clone, Version: Version6OrBigTiffVersion, UINT: UnsignedIntegerNormalizedType, UE: UnsignedEnum>(self, common: &mut TagParserCommon<'tiff_bytes, 'allocator, TB, A, Version>, tag_type: TagType) -> Result<EnumUnsignedInteger<UINT, UE>, IntegerValueParseError>
	{
		self.unsigned_integer_value(common, tag_type).map(EnumUnsignedInteger::from)
	}
	
	#[inline(always)]
	pub(in crate::image_file_directory) fn enum_signed_integer<'allocator, TB: TiffBytes, A: Allocator + Clone, Version: Version6OrBigTiffVersion, SINT: SignedIntegerNormalizedType, SE: SignedEnum>(self, common: &mut TagParserCommon<'tiff_bytes, 'allocator, TB, A, Version>, tag_type: TagType) -> Result<EnumSignedInteger<SINT, SE>, IntegerValueParseError>
	{
		self.signed_integer_value(common, tag_type).map(EnumSignedInteger::from)
	}
	
	#[inline(always)]
	pub(in crate::image_file_directory) fn unsigned_integer<'allocator, TB: TiffBytes, A: Allocator + Clone, Version: Version6OrBigTiffVersion, UINT: UnsignedIntegerNormalizedType>(self, common: &mut TagParserCommon<'tiff_bytes, 'allocator, TB, A, Version>, tag_type: TagType) -> Result<UnsignedInteger<UINT>, IntegerValueParseError>
	{
		self.unsigned_integer_value(common, tag_type).map(UnsignedInteger::from)
	}
	
	#[inline(always)]
	pub(in crate::image_file_directory) fn signed_integer<'allocator, TB: TiffBytes, A: Allocator + Clone, Version: Version6OrBigTiffVersion, SINT: SignedIntegerNormalizedType>(self, common: &mut TagParserCommon<'tiff_bytes, 'allocator, TB, A, Version>, tag_type: TagType) -> Result<SignedInteger<SINT>, IntegerValueParseError>
	{
		self.signed_integer_value(common, tag_type).map(SignedInteger::from)
	}
	
	#[inline(always)]
	pub(in crate::image_file_directory) fn unsigned_integers<'allocator, TB: TiffBytes, A: Allocator + Clone, Version: Version6OrBigTiffVersion, UINT: UnsignedIntegerNormalizedType>(self, common: &mut TagParserCommon<'tiff_bytes, 'allocator, TB, A, Version>, tag_type: TagType) -> Result<UnsignedIntegers<'tiff_bytes, UINT>, IntegerValuesParseError>
	{
		self.unsigned_integer_values(common, tag_type).map(UnsignedIntegers::from)
	}
	
	#[inline(always)]
	pub(in crate::image_file_directory) fn signed_integers<'allocator, TB: TiffBytes, A: Allocator + Clone, Version: Version6OrBigTiffVersion, SINT: SignedIntegerNormalizedType>(self, common: &mut TagParserCommon<'tiff_bytes, 'allocator, TB, A, Version>, tag_type: TagType) -> Result<SignedIntegers<'tiff_bytes, SINT>, IntegerValuesParseError>
	{
		self.signed_integer_values(common, tag_type).map(SignedIntegers::from)
	}
	
	#[inline(always)]
	fn unsigned_integer_value<'allocator, TB: TiffBytes, A: Allocator + Clone, Version: Version6OrBigTiffVersion>(self, common: &mut TagParserCommon<'tiff_bytes, 'allocator, TB, A, Version>, tag_type: TagType) -> Result<UnsignedIntegerValue, IntegerValueParseError>
	{
		UnsignedIntegerValue::parse(common.byte_order(), tag_type, self)
	}
	
	#[inline(always)]
	fn signed_integer_value<'allocator, TB: TiffBytes, A: Allocator + Clone, Version: Version6OrBigTiffVersion>(self, common: &mut TagParserCommon<'tiff_bytes, 'allocator, TB, A, Version>, tag_type: TagType) -> Result<SignedIntegerValue, IntegerValueParseError>
	{
		SignedIntegerValue::parse(common.byte_order(), tag_type, self)
	}
	
	#[inline(always)]
	fn unsigned_integer_values<'allocator, TB: TiffBytes, A: Allocator + Clone, Version: Version6OrBigTiffVersion>(self, common: &mut TagParserCommon<'tiff_bytes, 'allocator, TB, A, Version>, tag_type: TagType) -> Result<UnsignedIntegerValues<'tiff_bytes>, IntegerValuesParseError>
	{
		UnsignedIntegerValues::parse(common, tag_type, self)
	}
	
	#[inline(always)]
	fn signed_integer_values<'allocator, TB: TiffBytes, A: Allocator + Clone, Version: Version6OrBigTiffVersion>(self, common: &mut TagParserCommon<'tiff_bytes, 'allocator, TB, A, Version>, tag_type: TagType) -> Result<SignedIntegerValues<'tiff_bytes>, IntegerValuesParseError>
	{
		SignedIntegerValues::parse(common, tag_type, self)
	}
	
	#[inline(always)]
	fn parse<'allocator, TB: TiffBytes, A: Allocator + Clone, Version: Version6OrBigTiffVersion>(common: &mut TagParserCommon<'tiff_bytes, 'allocator, TB, A, Version>, tag_type_size_in_bytes: u64, count: u64, offset_or_value_union_index: u64) -> Result<Self, SpecificTagParseError>
	{
		use SpecificTagParseError::*;
		
		let slice =
		{
			let slice_size_in_bytes = tag_type_size_in_bytes.checked_mul(count).ok_or(CountIsTooLargeForTargetArchitecture)?;
			let (index, non_null) = if slice_size_in_bytes <= Version::Size
			{
				(offset_or_value_union_index, common.non_null_to_index_unchecked_mut::<u8>(offset_or_value_union_index, slice_size_in_bytes))
			}
			else
			{
				let raw_offset = common.unaligned_unchecked::<Version>(offset_or_value_union_index).into();
				let index = Offset::parse_offset_value(common.tiff_bytes, raw_offset).map_err(SliceOffsetParse)?.index();
				(index, common.non_null_to_index_checked_mut::<u8>(index, slice_size_in_bytes).map_err(OffsetIsTooLargeForTargetArchitecture)?)
			};
			
			common.record_used_space_slice(index, slice_size_in_bytes, FreeSpaceOutOfMemoryError::RecordingSlice)?;
			
			NonNull::slice_from_raw_parts(non_null, slice_size_in_bytes as usize)
		};
		
		Ok
		(
			Self
			{
				count,
				slice,
				marker: Default::default(),
			}
		)
	}
}
