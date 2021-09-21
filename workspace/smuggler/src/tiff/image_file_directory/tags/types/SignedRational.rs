// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


/// A signed rational.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct SignedRational
{
	numerator: i32,
	
	denominator: i32,
}

impl Rational<i32> for SignedRational
{
	#[inline(always)]
	fn numerator(self) -> i32
	{
		self.numerator
	}
	
	#[inline(always)]
	fn denominator(self) -> i32
	{
		self.denominator
	}
}
