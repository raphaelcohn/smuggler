// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


/// A rational fraction.
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct RationalFraction<RFA: RationalFractionAtor>
{
	numerator: RFA,
	
	denominator: RFA,
}

impl<RFA: RationalFractionAtor> CanBeUnaligned for RationalFraction<RFA>
{
	#[inline(always)]
	fn read_unaligned_byte_swapped(this: NonNull<Self>) -> Self
	{
		let numerator_non_null: NonNull<RFA> = this.cast();
		let denominator_non_null = new_non_null(unsafe { numerator_non_null.as_ptr().add(1) });
		
		Self::new
		(
			RFA::read_unaligned_byte_swapped(numerator_non_null),
			RFA::read_unaligned_byte_swapped(denominator_non_null),
		)
	}
}

impl<RFA: RationalFractionAtor> RationalFraction<RFA>
{
	/// Numerator.
	#[inline(always)]
	pub const fn numerator(self) -> RFA
	{
		self.numerator
	}
	
	/// Denominator.
	#[inline(always)]
	pub const fn denominator(self) -> RFA
	{
		self.denominator
	}
	
	#[inline(always)]
	pub(crate) fn new(numerator: RFA, denominator: RFA) -> Self
	{
		Self
		{
			numerator,
		
			denominator,
		}
	}
}
