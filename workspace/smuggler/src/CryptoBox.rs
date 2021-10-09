// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


trait CryptoBox: Aead + Clone
{
	fn create(public_key: &PublicKey, secret_key: &SecretKey) -> Self;
}

impl CryptoBox for ChaChaBox
{
	#[inline(always)]
	fn create(public_key: &PublicKey, secret_key: &SecretKey) -> Self
	{
		Self::new(public_key, secret_key)
	}
}

impl CryptoBox for SalsaBox
{
	#[inline(always)]
	fn create(public_key: &PublicKey, secret_key: &SecretKey) -> Self
	{
		Self::new(public_key, secret_key)
	}
}
