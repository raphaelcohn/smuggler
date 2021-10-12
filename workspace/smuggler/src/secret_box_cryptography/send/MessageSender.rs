// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


/// Message sender.
#[derive(Clone)]
pub struct MessageSender<CryptographicallySecureRandomNumberGenerator: RngCore + CryptoRng>
{
	crypto_box_algorithm: CryptoBoxAlgorithm<ChaChaBox, SalsaBox>,

	cryptographically_secure_random_number_generator: CryptographicallySecureRandomNumberGenerator,
}

impl<CryptographicallySecureRandomNumberGenerator: RngCore + CryptoRng + SeedableRng> MessageSender<CryptographicallySecureRandomNumberGenerator>
{
	/// Create a new secure instance.
	#[inline(always)]
	pub fn new_secure_from_operating_system_entropy(crypto_box_algorithm: CryptoBoxAlgorithm<(), ()>, recipient_public_key: PublicKey, sender_secret_key: SecretKey) -> Self
	{
		Self::new(crypto_box_algorithm, recipient_public_key, sender_secret_key, CryptographicallySecureRandomNumberGenerator::from_entropy())
	}
}

impl<CryptographicallySecureRandomNumberGenerator: RngCore + CryptoRng> MessageSender<CryptographicallySecureRandomNumberGenerator>
{
	#[inline(always)]
	fn new(crypto_box_algorithm: CryptoBoxAlgorithm<(), ()>, recipient_public_key: PublicKey, sender_secret_key: SecretKey, cryptographically_secure_random_number_generator: CryptographicallySecureRandomNumberGenerator) -> Self
	{
		Self
		{
			crypto_box_algorithm: crypto_box_algorithm.create(&recipient_public_key, &sender_secret_key),
			
			cryptographically_secure_random_number_generator,
		}
	}
	
	/// Encrypt a message.
	#[inline(always)]
	pub fn encrypt_message<'message>(&mut self, authenticated_but_not_encrypted_associated_data: &[u8], plain_text_message: &'message mut MessagePayload) -> Result<(), MessageEncryptionFailedError>
	{
		let one_time_use_nonce = self.generate_nonce()?;
		
		match self.crypto_box_algorithm.encrypt(one_time_use_nonce, authenticated_but_not_encrypted_associated_data, &mut plain_text_message.message)
		{
			Ok((nonce, authentication_tag)) =>
			{
				plain_text_message.nonce = nonce;
				plain_text_message.authentication_tag = authentication_tag;
				Ok(())
			},
			
			Err(cause) => Err(MessageEncryptionFailedError::BoxEncryption(cause)),
		}
	}
	
	#[inline(always)]
	fn generate_nonce(&mut self) -> Result<Nonce, MessageEncryptionFailedError>
	{
		let mut nonce: Nonce = unsafe_uninitialized();
		self.cryptographically_secure_random_number_generator.try_fill_bytes(&mut nonce).map_err(MessageEncryptionFailedError::NonceGeneration)?;
		Ok(nonce)
	}
}
