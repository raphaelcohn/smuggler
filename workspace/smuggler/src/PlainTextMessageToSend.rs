// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


/// A plain text message.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct PlainTextMessageToSend<'plain_text, 'additional_authenticated_data>
{
	/// Plain text to encrypt.
	pub plain_text: Cow<'plain_text, [u8]>,
	
	/// Also called the header; intended to be used for metadata that does not need to be confidential but does need authenticity and integrity protection.
	pub additional_authenticated_data: Cow<'additional_authenticated_data, [u8]>,
}

impl<'plain_text, 'additional_authenticated_data> PlainTextMessageToSend<'plain_text, 'additional_authenticated_data>
{
	#[allow(missing_docs)]
	#[inline(always)]
	pub const fn borrowed_with_additional_authenticated_data(plain_text: &'plain_text [u8], additional_authenticated_data: &'additional_authenticated_data [u8]) -> Self
	{
		use Cow::Borrowed;
		
		Self
		{
			plain_text: Borrowed(plain_text),
			
			additional_authenticated_data: Borrowed(additional_authenticated_data),
		}
	}
}

impl<'plain_text> PlainTextMessageToSend<'plain_text, 'static>
{
	#[allow(missing_docs)]
	#[inline(always)]
	pub const fn borrowed(plain_text: &'plain_text [u8]) -> Self
	{
		Self::borrowed_with_additional_authenticated_data(plain_text, NoAdditionalAuthenticatedData)
	}
}

impl PlainTextMessageToSend<'static, 'static>
{
	#[allow(missing_docs)]
	#[inline(always)]
	pub const fn owned(plain_text: Vec<u8>) -> Self
	{
		use Cow::*;
		
		Self
		{
			plain_text: Owned(plain_text),
		
			additional_authenticated_data: Borrowed(NoAdditionalAuthenticatedData),
		}
	}
	
	#[allow(missing_docs)]
	#[inline(always)]
	pub const fn owned_with_additional_authenticate_data(plain_text: Vec<u8, Global>, additional_authenticated_data: Vec<u8, Global>) -> Self
	{
		use Cow::Owned;
		
		Self
		{
			plain_text: Owned(plain_text),
			
			additional_authenticated_data: Owned(additional_authenticated_data),
		}
	}
}
