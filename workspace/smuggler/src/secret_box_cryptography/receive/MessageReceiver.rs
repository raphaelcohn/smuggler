// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


/// Message sender.
#[derive(Clone)]
pub struct MessageReceiver
{
	crypto_box_algorithm: CryptoBoxAlgorithm<ChaChaBox, SalsaBox>,
}

impl MessageReceiver
{
	/// Create a new instance.
	#[inline(always)]
	pub fn new(crypto_box_algorithm: CryptoBoxAlgorithm<(), ()>, recipient_secret_key: SecretKey, sender_public_key: PublicKey) -> Self
	{
		Self
		{
			crypto_box_algorithm: crypto_box_algorithm.create(&sender_public_key, &recipient_secret_key),
		}
	}
	
	/// Decrypt a message.
	#[inline(always)]
	pub fn decrypt_message(&mut self, authenticated_but_not_encrypted_associated_data: &[u8], cipher_text_message: &mut MessagePayload) -> Result<(), MessageDecryptionFailedError>
	{
		self.crypto_box_algorithm.decrypt(&cipher_text_message.nonce, authenticated_but_not_encrypted_associated_data, &mut cipher_text_message.message, &cipher_text_message.authentication_tag).map_err(MessageDecryptionFailedError)?;
		cipher_text_message.zeroize_nonce_and_authentication_tag();
		Ok(())
	}
}
