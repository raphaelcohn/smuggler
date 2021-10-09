// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


/// Subfile type enumeration.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum SubfileTypeEnum
{
	#[allow(missing_docs)]
	FullResolutionImageData = 1,
	
	#[allow(missing_docs)]
	ReducedResolutionImageData = 2,
	
	#[allow(missing_docs)]
	SinglePageOfAMultiPageImage = 3,
}

impl UnsignedEnum for SubfileTypeEnum
{
	#[inline(always)]
	fn try_from_u64(value: u64) -> Result<Self, u64>
	{
		use SubfileTypeEnum::*;
		
		match value
		{
			0 => Err(0),
			
			1 => Ok(FullResolutionImageData),
			
			2 => Ok(ReducedResolutionImageData),
			
			3 => Ok(SinglePageOfAMultiPageImage),
			
			_ => Err(value),
		}
	}
}
