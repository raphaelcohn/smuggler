// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


/// Crypto box algorithm.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum CryptoBoxAlgorithm<C, S>
{
	/// More recent, allows the additional authenticated data to be empty.
	ChaCha(C),
	
	/// Slightly older but better studied requires the additional authenticated data to not be empty.
	///
	/// Compatible with NaCl (libsodium)'s [crypto_secretbox](https://nacl.cr.yp.to/secretbox.html).
	Salsa(S),
}

impl CryptoBoxAlgorithm<(), ()>
{
	#[inline(always)]
	fn create(self, recipient_public_key: &PublicKey, sender_secret_key: &SecretKey) -> CryptoBoxAlgorithm<ChaChaBox, SalsaBox>
	{
		use CryptoBoxAlgorithm::*;
		
		match self
		{
			ChaCha(()) => ChaCha(ChaChaBox::create(recipient_public_key, sender_secret_key)),
			
			Salsa(()) => Salsa(SalsaBox::create(recipient_public_key, sender_secret_key)),
		}
	}
}

impl CryptoBoxAlgorithm<ChaChaBox, SalsaBox>
{
	#[inline(always)]
	fn encrypt(&self, nonce: &Nonce, plain_text_message: Payload) -> Result<CipherText, aead::Error>
	{
		use CryptoBoxAlgorithm::*;
		
		match self
		{
			ChaCha(crypto_box) => crypto_box.encrypt(&nonce, plain_text_message),
			
			Salsa(crypto_box) => crypto_box.encrypt(&nonce, plain_text_message),
		}
	}
	
	#[inline(always)]
	fn decrypt(&self, nonce: Nonce, cipher_text_message: Payload) -> Result<PlainText, aead::Error>
	{
		use CryptoBoxAlgorithm::*;
		
		match self
		{
			ChaCha(crypto_box) => crypto_box.decrypt(&nonce, cipher_text_message),
			
			Salsa(crypto_box) => crypto_box.decrypt(&nonce, cipher_text_message),
		}
	}
}
