// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


use std::alloc::Allocator;
use std::collections::TryReserveError;
use crypto_box::{ChaChaBox, Tag};
use crypto_box::PublicKey;
use crypto_box::SalsaBox;
use crypto_box::SecretKey;
use crypto_box::aead;
use crypto_box::aead::AeadInPlace;
use crypto_box::aead::Aead;
use crypto_box::aead::consts::U24;
use crypto_box::aead::generic_array::GenericArray;
use std::mem::size_of;
use std::mem::transmute;
use std::ptr;
use zeroize::Zeroize;
use rtiff::collections::VecExt;


/// Receive.
pub mod receive;


/// Send.
pub mod send;


include!("CryptoBox.rs");
include!("CryptoBoxAlgorithm.rs");
include!("MessagePayload.rs");
include!("NoAdditionalAuthenticatedData.rs");
include!("Nonce.rs");
include!("Poly1305AuthenticationTag.rs");
