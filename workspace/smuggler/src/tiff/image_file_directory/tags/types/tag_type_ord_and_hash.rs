// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


macro_rules! tag_type_ord_and_hash
{
	($aligned_type: ty, $unaligned_type: ty) =>
	{
		tag_type!($aligned_type, $unaligned_type);
		
		impl Ord for $aligned_type
		{
			#[inline(always)]
			fn cmp(&self, other: &Self) -> Ordering
			{
				self.to_aligned().cmp(&other.to_aligned())
			}
		}
		
		impl Hash for $aligned_type
		{
			#[inline(always)]
			fn hash<H: Hasher>(&self, state: &mut H)
			{
				self.to_aligned().hash(state)
			}
		}
	}
}
