// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


use std::hash::Hash;
use std::hash::Hasher;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::cmp::Ordering;
use crate::tiff::image_file_directory::tags::parsers::{TagParseError, Version6OrBigTiffUnit};
use std::mem::transmute;
use crate::tiff::image_file_directory::tags::TagIdentifier;
use crate::collections::{Bytes, ByteOrder};


include!("tag_type.rs");
include!("tag_type_ord_and_hash.rs");


include!("AsciiCharacter.rs");
include!("Rational.rs");
include!("SignedRational.rs");
include!("TagType.rs");
include!("TagTypeTrait.rs");
include!("UnalignedF32.rs");
include!("UnalignedF64.rs");
include!("UnalignedI16.rs");
include!("UnalignedI32.rs");
include!("UnalignedI64.rs");
include!("UnalignedU16.rs");
include!("UnalignedU32.rs");
include!("UnalignedU64.rs");
include!("UnalignedUnsignedRational.rs");
include!("UnalignedSignedRational.rs");
include!("UnsignedIntegerTagType.rs");
include!("UnsignedRational.rs");
