// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


/// Public tag key (or name).
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u16)]
pub enum PublicTagKey
{
	#[allow(missing_docs)]
	XResolution = 0,
	
	#[allow(missing_docs)]
	YResolution = 1,
	
	#[allow(missing_docs)]
	Unrecognized(TagIdentifier) = UnrecognizedRepresentationValue,
}

impl PartialOrd for PublicTagKey
{
	#[inline(always)]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering>
	{
		Some(self.cmp(other))
	}
}

impl Ord for PublicTagKey
{
	#[inline(always)]
	fn cmp(&self, other: &Self) -> Ordering
	{
		self.tag_identifier().cmp(&other.tag_identifier())
	}
}

impl TagKey for PublicTagKey
{
	#[inline(always)]
	fn tag_identifier(self) -> TagIdentifier
	{
		let raw_tag_key: RawTagKey = unsafe { transmute(self) };
		raw_tag_key.tag_identifier()
	}
}

impl PublicTagKey
{
}
