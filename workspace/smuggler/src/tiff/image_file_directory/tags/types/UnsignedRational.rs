// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


/// An unsigned rational.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct UnsignedRational
{
	numerator: u32,
	
	denominator: u32,
}

impl Rational<u32> for UnsignedRational
{
	#[inline(always)]
	fn numerator(self) -> u32
	{
		self.numerator
	}
	
	#[inline(always)]
	fn denominator(self) -> u32
	{
		self.denominator
	}
}
