// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(EnumCount, EnumIter, EnumString, IntoStaticStr, ToString)]
#[repr(u8)]
pub enum NewSubfileTypeBitFieldBit
{
	#[allow(missing_docs)]
	ReducedResolutionVersionOfAnotherImage = 0,
	
	#[allow(missing_docs)]
	SinglePageOfMultiPageImage = 1,
	
	#[allow(missing_docs)]
	TransparencyMaskOfAnotherImage = 2,
	
	#[allow(missing_docs)]
	DepthMapForAnotherImageDigitalNegativeBit = 3,
	
	/// This bit has conflicting meanings.
	MixedRasterContentOrEnhancedImageDigitalNegative = 4,
}

impl Into<u8> for NewSubfileTypeBitFieldBit
{
	#[inline(always)]
	fn into(self) -> u8
	{
		self as u8
	}
}
