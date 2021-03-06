// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


use crate::bytes::ByteOrder;
use crate::bytes::CanBeUnaligned;
use crate::bytes::NonNullExt;
use crate::bytes::TiffBytes;
use crate::collections::VecExt;
use likely::unlikely;
use memchr::memchr;
use std::alloc::Allocator;
use std::cmp::Ordering;
use std::collections::TryReserveError;
use std::error;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt;
use std::hash::Hash;
use std::hash::Hasher;
use std::marker::PhantomData;
use std::mem::transmute;
use std::num::NonZeroU8;
use std::num::NonZeroUsize;
use std::ptr::NonNull;
use std::ptr::addr_of;
use std::str::FromStr;
use strum::EnumCount;
use strum::IntoEnumIterator;
use super::TagType;
use super::parsers::RawTagValue;
use super::parsers::TagParserCommon;
use crate::bytes::versions::Version6OrBigTiffVersion;
use swiss_army_knife::get_unchecked::GetUnchecked;
use swiss_army_knife::non_zero::new_non_null;
use swiss_army_knife::non_zero::new_non_zero_usize;


include!("AsciiStrings.rs");
include!("AsciiStringsParseError.rs");
include!("BitField.rs");
include!("BitFieldInteger.rs");
include!("EnumSignedInteger.rs");
include!("EnumUnsignedInteger.rs");
include!("IntegerNormalizedType.rs");
include!("IntegerValueParseError.rs");
include!("IntegerValues.rs");
include!("IntegerValuesParseError.rs");
include!("RationalFraction.rs");
include!("RationalFractionAtor.rs");
include!("SignedEnum.rs");
include!("SignedInteger.rs");
include!("SignedIntegers.rs");
include!("SignedIntegerNormalizedType.rs");
include!("SignedIntegerValue.rs");
include!("SignedIntegerValues.rs");
include!("Unaligned.rs");
include!("UnsignedEnum.rs");
include!("UnsignedInteger.rs");
include!("UnsignedIntegers.rs");
include!("UnsignedIntegerNormalizedType.rs");
include!("UnsignedIntegerValue.rs");
include!("UnsignedIntegerValues.rs");
