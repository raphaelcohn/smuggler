// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


/// A tag type.
pub trait TagTypeTrait: Default + Debug + Copy + Eq + PartialOrd
{
	/// Aligned form.
	type Aligned: Default + Debug + Copy + Eq + PartialOrd;

	/// Dereference to aligned form.
	fn to_aligned(&self) -> Self::Aligned;
}

impl TagTypeTrait for u8
{
	type Aligned = Self;
	
	#[inline(always)]
	fn to_aligned(&self) -> Self::Aligned
	{
		*self
	}
}

impl TagTypeTrait for i8
{
	type Aligned = Self;
	
	#[inline(always)]
	fn to_aligned(&self) -> Self::Aligned
	{
		*self
	}
}
