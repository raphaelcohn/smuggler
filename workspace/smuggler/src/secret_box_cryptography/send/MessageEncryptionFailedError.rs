// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


/// Message encrypted failed.
#[derive(Debug)]
pub enum MessageEncryptionFailedError
{
	#[allow(missing_docs)]
	NonceGeneration(rand_core::Error),
	
	#[allow(missing_docs)]
	BoxEncryption(aead::Error)
}

impl Display for MessageEncryptionFailedError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for MessageEncryptionFailedError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use MessageEncryptionFailedError::*;
		
		match self
		{
			NonceGeneration(cause) => Some(cause),
			
			// Does not implement the trait `error::Error`.
			BoxEncryption(_) => None,
		}
	}
}
