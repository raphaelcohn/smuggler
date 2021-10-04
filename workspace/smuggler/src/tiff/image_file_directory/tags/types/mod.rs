// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


use likely::unlikely;
use super::TagIdentifier;
use super::parsers::TagParseError;
use super::parsers::SpecificTagParseError;
use super::parsers::Version6OrBigTiffUnit;
use crate::collections::{ByteOrder, VecExt};
use crate::collections::Bytes;
use crate::tiff::image_file_directory::pointer::ImageFileDirectoryPointer;
use crate::tiff::offset::Offset;
use memchr::memchr;
use std::alloc::Allocator;
use std::cmp::Ordering;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt;
use std::hash::Hash;
use std::hash::Hasher;
use std::marker::PhantomData;
use std::mem::transmute;
use std::num::NonZeroU8;
use strum::EnumCount;
use strum::IntoEnumIterator;
use std::str::FromStr;
use std::convert::TryInto;
use std::slice::from_raw_parts_mut;
use swiss_army_knife::byte_swap;
use swiss_army_knife::byte_swap::Unaligned16;
use swiss_army_knife::byte_swap::Unaligned32;
use swiss_army_knife::byte_swap::Unaligned64;


include!("AsciiStrings.rs");
include!("BitField.rs");
include!("BitFieldInteger.rs");
include!("BytesSlice.rs");
include!("CanBeUnaligned.rs");
include!("EnumSignedInteger.rs");
include!("EnumUnsignedInteger.rs");
include!("IntegerNormalizedType.rs");
include!("RationalFraction.rs");
include!("RationalFractionAtor.rs");
include!("SignedEnum.rs");
include!("SignedInteger.rs");
include!("SignedIntegerNormalizedType.rs");
include!("SignedIntegerValue.rs");
include!("TagType.rs");
include!("Unaligned.rs");
include!("Undefined.rs");
include!("UnsignedEnum.rs");
include!("UnsignedInteger.rs");
include!("UnsignedIntegerNormalizedType.rs");
include!("UnsignedIntegerValue.rs");
include!("UnrecognizedTagValue.rs");
