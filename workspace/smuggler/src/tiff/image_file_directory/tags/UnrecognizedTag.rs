// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


/// An unrecognized tag.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct UnrecognizedTag<'a, A: Allocator>(pub TagIdentifier, pub UnrecognizedTagValue<'a, A>);

impl<'a, A: Allocator> Tag for UnrecognizedTag<'a, A>
{
	/// Key type.
	type Key = TagIdentifier;
	
	/// Obtain key.
	fn key(&self) -> Self::Key
	{
		self.0
	}
}
