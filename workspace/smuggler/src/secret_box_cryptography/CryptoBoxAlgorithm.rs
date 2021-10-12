// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


/// Crypto box algorithm.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum CryptoBoxAlgorithm<C, S>
{
	/// More recent, allows the additional authenticated data to be empty.
	///
	/// Compatible with NaCl (libsodium)'s `crypto_box_curve25519xchacha20poly1305`.
	///
	/// Implemented using:-
	///
	/// * X25519 elliptic curve with Diffie-Hellman for public key cryptography.
	/// * XChaCha20 stream cipher for encryption.
	/// 	* This uses a 256-bit (32 byte) key.
	/// 	* This uses a 192-bit (24 byte) nonce.
	/// * Poly1305 for the message authentication code (MAC).
	/// 	* This uses a 128-bit (16 byte) authenticated tag.
	X25519_XChaCha20_Poly1305(C),
	
	/// Slightly older but better studied requires the additional authenticated data to not be empty.
	///
	/// Compatible with NaCl (libsodium)'s [crypto_secretbox](https://nacl.cr.yp.to/secretbox.html), specifically , `crypto_box_curve25519xsalsa20poly1305`.
	///
	/// Implemented using:-
	///
	/// * X25519 elliptic curve with Diffie-Hellman for public key cryptography.
	/// * XSalsa20 stream cipher for encryption.
	/// 	* This uses a 256-bit (32 byte) key.
	/// 	* This uses 192-bit (24 byte) nonce.
	/// * Poly1305 for the message authentication code (MAC).
	/// 	* This uses a 128-bit (16 byte) authenticated tag.
	X25519_XSalsa20_Poly1305(S),
}

impl CryptoBoxAlgorithm<(), ()>
{
	#[inline(always)]
	fn create(self, recipient_public_key: &PublicKey, sender_secret_key: &SecretKey) -> CryptoBoxAlgorithm<ChaChaBox, SalsaBox>
	{
		use CryptoBoxAlgorithm::*;
		
		match self
		{
			X25519_XChaCha20_Poly1305(()) => X25519_XChaCha20_Poly1305(ChaChaBox::create(recipient_public_key, sender_secret_key)),
			
			X25519_XSalsa20_Poly1305(()) => X25519_XSalsa20_Poly1305(SalsaBox::create(recipient_public_key, sender_secret_key)),
		}
	}
}

impl CryptoBoxAlgorithm<ChaChaBox, SalsaBox>
{
	#[inline(always)]
	fn encrypt<'message>(&self, one_time_use_nonce: Nonce, authenticated_but_not_encrypted_associated_data: &[u8], plain_text_message: &'message mut [u8]) -> Result<(Nonce, Poly1305AuthenticationTag), aead::Error>
	{
		use CryptoBoxAlgorithm::*;
		
		let nonce = Self::nonce_as_generic_array(&one_time_use_nonce);
		let tag = match self
		{
			X25519_XChaCha20_Poly1305(crypto_box) => crypto_box.encrypt_in_place_detached(nonce, authenticated_but_not_encrypted_associated_data, plain_text_message)?,
			
			X25519_XSalsa20_Poly1305(crypto_box) => crypto_box.encrypt_in_place_detached(nonce, authenticated_but_not_encrypted_associated_data, plain_text_message)?,
		};
		
		Ok((one_time_use_nonce, Self::generic_array_into_authentication_tag(tag)))
	}
	
	#[inline(always)]
	fn decrypt<'message>(&self, one_time_use_nonce: &Nonce, authenticated_but_not_encrypted_associated_data: &[u8], cipher_text_message: &'message mut [u8], authentication_tag: &Poly1305AuthenticationTag) -> Result<(), aead::Error>
	{
		use CryptoBoxAlgorithm::*;
		
		let nonce = Self::nonce_as_generic_array(one_time_use_nonce);
		let tag = Self::authentication_tag_as_generic_array(authentication_tag);
		match self
		{
			X25519_XChaCha20_Poly1305(crypto_box) => crypto_box.decrypt_in_place_detached(nonce, authenticated_but_not_encrypted_associated_data, cipher_text_message, tag)?,
			
			X25519_XSalsa20_Poly1305(crypto_box) => crypto_box.decrypt_in_place_detached(nonce, authenticated_but_not_encrypted_associated_data, cipher_text_message, tag)?,
		}
		
		Ok(())
	}
	
	#[inline(always)]
	fn nonce_as_generic_array(nonce: &Nonce) -> &GenericArray<u8, U24>
	{
		unsafe { &* (nonce as *const _ as *const GenericArray<u8, U24>) }
	}
	
	#[inline(always)]
	fn authentication_tag_as_generic_array(authentication_tag: &Poly1305AuthenticationTag) -> &Tag
	{
		unsafe { &* (authentication_tag as *const _ as *const Tag) }
	}
	
	#[inline(always)]
	fn generic_array_into_authentication_tag(tag: Tag) -> Poly1305AuthenticationTag
	{
		unsafe { transmute(tag) }
	}
}
