// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


/// A value that can be unaligned in memory, but will be native endian.
#[repr(packed)]
pub struct Unaligned<CBU: CanBeUnaligned>(CBU);

impl<CBU: CanBeUnaligned> Default for Unaligned<CBU>
{
	#[inline(always)]
	fn default() -> Self
	{
		Self(CBU::default())
	}
}

impl<CBU: CanBeUnaligned> Debug for Unaligned<CBU>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		self.read().fmt(f)
	}
}

impl<CBU: CanBeUnaligned> Clone for Unaligned<CBU>
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		Self(self.read())
	}
}

impl<CBU: CanBeUnaligned> PartialEq for Unaligned<CBU>
{
	#[inline(always)]
	fn eq(&self, rhs: &Self) -> bool
	{
		self.read().eq(&rhs.read())
	}
}

impl<CBU: CanBeUnaligned + Eq> Eq for Unaligned<CBU>
{
}

impl<CBU: CanBeUnaligned> PartialOrd for Unaligned<CBU>
{
	#[inline(always)]
	fn partial_cmp(&self, rhs: &Self) -> Option<Ordering>
	{
		self.read().partial_cmp(&rhs.read())
	}
}

impl<CBU: CanBeUnaligned + Eq + Ord> Ord for Unaligned<CBU>
{
	#[inline(always)]
	fn cmp(&self, rhs: &Self) -> Ordering
	{
		self.read().cmp(&rhs.read())
	}
}

impl<CBU: CanBeUnaligned + Hash> Hash for Unaligned<CBU>
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, state: &mut H)
	{
		self.read().hash(state)
	}
}

impl<CBU: CanBeUnaligned> Unaligned<CBU>
{
	#[inline(always)]
	fn read(&self) -> CBU
	{
		let pointer = addr_of!(self.0);
		unsafe { pointer.read_unaligned() }
	}
}
