// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


/// A set of one or more ASCII strings.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct AsciiStrings<'a, A: Allocator>
{
	strings: Vec<&'a [NonZeroU8], A>,

	omits_final_nul_byte: bool,
}

impl<'a, A: Allocator> AsciiStrings<'a, A>
{
	#[inline(always)]
	const fn new(strings: Vec<&'a [NonZeroU8], A>, omits_final_nul_byte: bool) -> Self
	{
		Self
		{
			strings,
		
			omits_final_nul_byte,
		}
	}
}
