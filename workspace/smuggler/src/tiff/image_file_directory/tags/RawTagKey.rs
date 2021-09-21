// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct RawTagKey(u16, TagIdentifier);

impl PartialOrd for RawTagKey
{
	#[inline(always)]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering>
	{
		Some(self.cmp(other))
	}
}

impl Ord for RawTagKey
{
	#[inline(always)]
	fn cmp(&self, other: &Self) -> Ordering
	{
		self.tag_identifier().cmp(&other.tag_identifier())
	}
}

impl RawTagKey
{
	#[inline(always)]
	fn tag_identifier(self) -> TagIdentifier
	{
		if unlikely!(self.0 == UnrecognizedRepresentationValue)
		{
			self.1
		}
		else
		{
			self.0
		}
	}
}
