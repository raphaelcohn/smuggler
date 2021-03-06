// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


pub(super) trait PointerToIndexLength: Default + Debug + Copy + Clone + Eq + Ord + Hash
{
	type CheckOutcome;
	
	#[inline(always)]
	fn check<X: Sized, TB: TiffBytes + ?Sized>(tiff_bytes: &TB, index: Index, count: u64) -> Self::CheckOutcome
	{
		Self::check_inner(index, size_of_u64::<X>() * count, tiff_bytes.file_length())
	}
	
	fn check_inner(index: Index, size_in_bytes: u64, file_length: FileLength) -> Self::CheckOutcome;
}
