// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


pub(in crate::tiff::image_file_directory) trait TagParser
{
	type Tag<'a, A: Allocator>: Tag<'a, A>;
	
	fn parse<'a, Unit: Version6OrBigTiffUnit, A: Allocator, B: Bytes>(&self, allocator: A, tiff_bytes: &'a mut B, tag_identifier: TagIdentifier, tag_type: u16, count: u64, byte_order: ByteOrder, offset_or_value_union_index: u64) -> Result<Self::Tag<'a, A>, TagParseError>;
}
