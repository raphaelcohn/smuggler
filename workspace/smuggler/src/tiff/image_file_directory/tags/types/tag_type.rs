// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


macro_rules! tag_type
{
	($unaligned_type: ty, $aligned_type: ty) =>
	{
		/// Unaligned type.
		#[repr(packed)]
		pub struct UnalignedF64(f64);
		
		impl Default for $unaligned_type
		{
			#[inline(always)]
			fn default() -> Self
			{
				Self(Default::default())
			}
		}
		
		impl Debug for $unaligned_type
		{
			#[inline(always)]
			fn fmt(&self, f: &mut Formatter) -> fmt::Result
			{
				self.to_aligned().fmt(f)
			}
		}
		
		impl Clone for $unaligned_type
		{
			#[inline(always)]
			fn clone(&self) -> Self
			{
				Self(self.to_aligned())
			}
		}
		
		impl Copy for $unaligned_type
		{
		}
		
		impl PartialEq for $unaligned_type
		{
			#[inline(always)]
			fn eq(&self, other: &Self) -> bool
			{
				self.to_aligned() == other.to_aligned()
			}
		}
		
		impl Eq for $unaligned_type
		{
		}
		
		impl PartialOrd for $unaligned_type
		{
			#[inline(always)]
			fn partial_cmp(&self, other: &Self) -> Option<Ordering>
			{
				self.to_aligned().partial_cmp(&other.to_aligned())
			}
		}
		
		impl TagTypeTrait for $unaligned_type
		{
			type Aligned = $aligned_type;
			
			#[inline(always)]
			fn to_aligned(&self) -> Self::Aligned
			{
				unsafe { (&self.0 as *const Self::Aligned).read_unaligned() }
			}
		}
	}
}
