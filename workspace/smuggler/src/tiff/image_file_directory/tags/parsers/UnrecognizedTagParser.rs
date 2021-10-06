// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(in crate::tiff::image_file_directory::tags) struct UnrecognizedTagParser;

impl TagParser for UnrecognizedTagParser
{
	type Tag<'a, A: Allocator> = UnrecognizedTag<'a, A>;
	
	#[inline(always)]
	fn parse<'a, A, Unit: Version6OrBigTiffUnit, TB: TiffBytes>(&self, recursion_guard: &RecursionGuard, allocator: A, tiff_bytes: &'a mut TB, tag_identifier: TagIdentifier, tag_type: TagType, count: u64, byte_order: ByteOrder, slice: NonNull<[u8]>) -> Result<Self::Tag<'a, A>, SpecificTagParseError>
	{
		UnrecognizedTag::parse(recursion_guard, allocator, tiff_bytes, tag_identifier, tag_type, count, byte_order, slice)
	}
}
