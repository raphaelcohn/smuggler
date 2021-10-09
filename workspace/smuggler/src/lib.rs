// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/smuggler/master/COPYRIGHT.


#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![deny(absolute_paths_not_starting_with_crate)]
#![deny(invalid_html_tags)]
#![deny(macro_use_extern_crate)]
#![deny(missing_crate_level_docs)]
#![deny(missing_docs)]
#![deny(pointer_structural_match)]
#![deny(unaligned_references)]
#![deny(unconditional_recursion)]
#![deny(unreachable_patterns)]
#![deny(unused_import_braces)]
#![deny(unused_must_use)]
#![deny(unused_qualifications)]
#![deny(unused_results)]
#![warn(unreachable_pub)]
#![warn(unused_lifetimes)]
#![warn(unused_crate_dependencies)]


#![feature(allocator_api)]


#![doc = include_str!("../README.md")]


use crypto_box::ChaChaBox;
use crypto_box::PublicKey;
use crypto_box::SalsaBox;
use crypto_box::SecretKey;
use crypto_box::generate_nonce;
use crypto_box::aead;
use crypto_box::aead::Aead;
use crypto_box::aead::Payload;
use crypto_box::aead::consts::U24;
use crypto_box::aead::generic_array::GenericArray;
use crypto_box::rand_core::CryptoRng;
use crypto_box::rand_core::RngCore;
use crypto_box::rand_core::SeedableRng;
use std::alloc::Global;
use std::borrow::Cow;
use std::error;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;


include!("CipherText.rs");
include!("CipherTextMessageReceived.rs");
include!("CryptoBox.rs");
include!("CryptoBoxAlgorithm.rs");
include!("MessageDecryptionFailedError.rs");
include!("MessageEncryptionFailedError.rs");
include!("MessageReceiver.rs");
include!("MessageSender.rs");
include!("NoAdditionalAuthenticatedData.rs");
include!("Nonce.rs");
include!("PlainText.rs");
include!("PlainTextMessageToSend.rs");
