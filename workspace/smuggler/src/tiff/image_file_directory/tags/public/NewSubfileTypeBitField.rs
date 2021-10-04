// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct NewSubfileTypeBitField(u64);

impl Into<u64> for NewSubfileTypeBitField
{
	#[inline(always)]
	fn into(self) -> u64
	{
		self.0
	}
}

impl BitField for NewSubfileTypeBitField
{
	type Bit = NewSubfileTypeBitFieldBit;
	
	#[inline(always)]
	fn try_from_u64(bits: u64) -> Result<Self, (Self, u64)>
	{
		Ok(Self(bits))
	}
}
