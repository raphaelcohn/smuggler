// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


use crate::bytes::Byte;
use crate::bytes::ByteOrder;
use crate::bytes::CanBeUnaligned;
use crate::bytes::Index;
use crate::bytes::OverflowError;
use crate::bytes::TiffBytes;
use crate::bytes::TiffBytesWithOrder;
use crate::bytes::versions::Version6OrBigTiffVersion;
use crate::offset::Offset;
use crate::offset::OffsetParseError;
use likely::unlikely;
use std::alloc::Allocator;
use std::collections::{HashSet, TryReserveError};
use std::error;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt;
use std::marker::PhantomData;
use std::num::NonZeroU8;
use std::ops::Deref;
use std::ops::DerefMut;
use std::ptr::NonNull;
use super::Tag;
use super::TagType;
use super::public::PublicTagFinishParseError;
use super::public::PublicTagParseError;
use super::super::ChildImageFileDirectoriesParseError;
use super::super::ImageFileDirectoriesParseError;
use super::super::pointer::ImageFileDirectoryPointer;
use super::tag_identifiers::TagIdentifier;
use super::values::AsciiStrings;
use super::values::AsciiStringsParseError;
use super::values::BitField;
use super::values::BitFieldInteger;
use super::values::EnumSignedInteger;
use super::values::EnumUnsignedInteger;
use super::values::IntegerValueParseError;
use super::values::IntegerValuesParseError;
use super::values::SignedEnum;
use super::values::SignedInteger;
use super::values::SignedIntegerNormalizedType;
use super::values::SignedIntegerValue;
use super::values::SignedIntegerValues;
use super::values::SignedIntegers;
use super::values::Unaligned;
use super::values::UnsignedEnum;
use super::values::UnsignedInteger;
use super::values::UnsignedIntegerNormalizedType;
use super::values::UnsignedIntegerValue;
use super::values::UnsignedIntegerValues;
use super::values::UnsignedIntegers;
use super::unrecognized::UnrecognizedTagFinishParseError;
use super::unrecognized::UnrecognizedTagParseError;
use swiss_army_knife::non_zero::new_non_zero_u8;
use crate::free_space::{FreeSpace, FreeSpaceOutOfMemoryError};


include!("FinishParseError.rs");
include!("RawTagValue.rs");
include!("SpecificTagParseError.rs");
include!("TagParserCommon.rs");
include!("TagParseError.rs");
include!("TagParser.rs");
include!("TagEventHandler.rs");
