// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


/// Message payload.
#[repr(C, packed)]
pub struct MessagePayload
{
	#[allow(missing_docs)]
	pub nonce: Nonce,
	
	#[allow(missing_docs)]
	pub authentication_tag: Poly1305AuthenticationTag,
	
	#[allow(missing_docs)]
	pub message: [u8],
}

impl<'a> From<&'a [u8]> for &'a MessagePayload
{
	#[inline(always)]
	fn from(bytes: &'a [u8]) -> Self
	{
		let data_address = bytes.as_ptr() as *const ();
		let metadata = MessagePayload::metadata(bytes);
		let fat_pointer = ptr::from_raw_parts::<MessagePayload>(data_address, metadata);
		unsafe { & * fat_pointer }
	}
}

impl<'a> From<&'a mut [u8]> for &'a mut MessagePayload
{
	#[inline(always)]
	fn from(bytes: &'a mut [u8]) -> Self
	{
		let data_address = bytes.as_mut_ptr() as *mut ();
		let metadata = MessagePayload::metadata(bytes);
		let fat_pointer = ptr::from_raw_parts_mut::<MessagePayload>(data_address, metadata);
		unsafe { &mut * fat_pointer }
	}
}

impl Zeroize for MessagePayload
{
	#[inline(always)]
	fn zeroize(&mut self)
	{
		self.zeroize_nonce_and_authentication_tag();
		self.message.zeroize()
	}
}

impl MessagePayload
{
	/// Creates a new, uninitialized instance in which all fields contain the random contents of memory (garbage).
	#[inline(always)]
	pub fn try_new_in<A: Allocator>(allocator: A, message_length: usize) -> Result<Box<Self, A>, TryReserveError>
	{
		let size = Self::FixedSize + message_length;
		// NOTE: This can be used instead of layout because (a) we're packed, (b) we're a C representation (so layout is known) and (c) all fields are effectively Copy and can be initialized with unknown values.
		let bytes: Box<[u8], A> = Vec::new_buffer(size, allocator)?.into_boxed_slice();
		let (bytes, allocator) = Box::into_raw_with_allocator(bytes);
		
		let data_address = bytes.as_mut_ptr() as *mut ();
		let metadata = message_length;
		let fat_pointer = ptr::from_raw_parts_mut::<Self>(data_address, metadata);
		Ok(unsafe { Box::from_raw_in(fat_pointer, allocator) })
	}
	
	#[inline(always)]
	fn zeroize_nonce_and_authentication_tag(&mut self)
	{
		self.nonce.zeroize();
		self.authentication_tag.zeroize();
	}
	
	const FixedSize: usize = size_of::<Nonce>() + size_of::<Poly1305AuthenticationTag>();
	
	#[inline(always)]
	const fn metadata(bytes: &[u8]) -> usize
	{
		bytes.len() - Self::FixedSize
	}
}
