// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


/// A public tag.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(u16)]
pub enum PublicTag<'a, A: Allocator>
{
	#[allow(missing_docs)]
	XResolution(UnalignedUnsignedRational) = 0,
	
	#[allow(missing_docs)]
	YResolution(UnalignedUnsignedRational) = 1,
	
	#[allow(missing_docs)]
	Unrecognized(TagIdentifier, UnrecognizedTagValue<'a>) = UnrecognizedRepresentationValue,
}

impl<'a, A: Allocator> EnumRepresentationU16 for PublicTag<'a, A>
{
}

impl<'a, A: Allocator> Tag for PublicTag<'a, A>
{
	type Key = PublicTagKey;
	
	#[inline(always)]
	fn key(&self) -> Self::Key
	{
		unsafe { transmute(self.raw_tag_key()) }
	}
}
