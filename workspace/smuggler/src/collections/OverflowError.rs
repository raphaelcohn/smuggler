// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


/// A parse error.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) enum OverflowError
{
	#[allow(missing_docs)]
	SizeOverflowsIndex
	{
		index: Index,
		
		size_in_bytes: u64,
	},
	
	#[allow(missing_docs)]
	PointerOverflowsFileLength
	{
		index: Index,
		
		size_in_bytes: u64,
		
		file_length: FileLength,
	},
	
	/// This only occurs on 32-bit and 16-bit architectures.
	#[cfg(not(target_pointer_width = "64"))]
	IndexExceedsUsize
	{
		index: Index,
	},
	
	/// This only occurs on 32-bit and 16-bit architectures.
	#[cfg(not(target_pointer_width = "64"))]
	SizeExceedsUsize
	{
		size_in_bytes: u64,
	},
	
	/// This only occurs on 32-bit and 16-bit architectures.
	#[cfg(not(target_pointer_width = "64"))]
	EndPointerExceedsUsizePlusOne
	{
		index: Index,
		
		size_in_bytes: u64,
	},
}

impl Display for OverflowError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for OverflowError
{
}
