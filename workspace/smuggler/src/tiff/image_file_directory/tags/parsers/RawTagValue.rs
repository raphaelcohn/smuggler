// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


#[derive(Debug)]
pub(in crate::tiff::image_file_directory) struct RawTagValue<'tiff_bytes>
{
	pub(in crate::tiff::image_file_directory) count: u64,
	
	pub(in crate::tiff::image_file_directory) slice: NonNull<[u8]>,

	marker: PhantomData<&'tiff_bytes ()>,
}

impl<'tiff_bytes> RawTagValue<'tiff_bytes>
{
	#[inline(always)]
	fn parse<Unit: Version6OrBigTiffUnit>(recursion_guard: &RecursionGuard, tiff_bytes: &'tiff_bytes impl TiffBytes, tag_type_size_in_bytes: u64, count: u64, offset_or_value_union_index: u64) -> Result<Self, SpecificTagParseError>
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
				let raw_offset = Unit::offset_like_value_unchecked(tiff_bytes, offset_or_value_union_index, byte_order);
				let index = Offset::parse_offset_value(tiff_bytes, raw_offset).map_err(SliceOffsetParse)?.index();
				(index, tiff_bytes.non_null_to_index_checked_mut::<u8>(index, slice_size_in_bytes).map_err(OffsetIsTooLargeForTargetArchitecture)?)
			};
			
			recursion_guard.record_used_space_slice(index, slice_size_in_bytes);
			
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
