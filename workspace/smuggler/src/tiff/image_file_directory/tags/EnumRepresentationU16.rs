// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


trait EnumRepresentationU16
{
	#[inline(always)]
	fn raw_tag_key(&self) -> RawTagKey
	{
		RawTagKey
		(
			self.representation_value(),
			
			// If there is no first field of any tuple or struct enum member, this value will be zero (0).
			self.tag_identifier()
		)
	}
	
	/// This code is safe\*, as it relies on [RFC 2195](https://rust-lang.github.io/rfcs/2195-really-tagged-unions.html) stabilizing enum layout.
	///
	/// \* As long as the enum is annotated `#[repr(u16)]`.
	#[inline(always)]
	fn representation_value(&self) -> u16
	{
		let pointer_to_representation_value = self.pointer_to_representation_value();
		unsafe { pointer_to_representation_value.read() }
	}
	
	/// This code is safe\*, as it relies on [RFC 2195](https://rust-lang.github.io/rfcs/2195-really-tagged-unions.html) stabilizing enum layout.
	///
	/// \* As long as the enum is annotated `#[repr(u16)]` *AND* the first field of any tuple or struct enum member is either an TagIdentifier or nothing at all.
	#[inline(always)]
	fn tag_identifier(&self) -> TagIdentifier
	{
		let pointer_to_representation_value = self.pointer_to_representation_value();
		unsafe { (pointer_to_representation_value.add(1) as *const TagIdentifier).read() }
	}
	
	#[inline(always)]
	fn pointer_to_representation_value(&self) -> *const u16
	{
		self as *const Self as *const u16
	}
}
