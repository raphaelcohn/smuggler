// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


use crate::bytes::Byte;
use crate::bytes::ByteOrder;
use crate::bytes::CanBeUnaligned;
use crate::bytes::Index;
use crate::bytes::OverflowError;
use crate::bytes::TiffBytes;
use crate::bytes::TiffBytesWithOrder;
use crate::bytes::size_of_u64;
use crate::header::{BigTiffHeaderParseError, Version};
use crate::offset::Offset;
use crate::offset::OffsetParseError;
use likely::unlikely;
use std::alloc::Allocator;
use std::collections::HashSet;
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
use super::tag_identifiers::TagIdentifier;
use super::public::PublicTagFinishParseError;
use super::super::ImageFileDirectoriesParseError;
use super::super::pointer::ImageFileDirectoryPointer;
use super::types::AsciiStrings;
use super::types::BitField;
use super::types::BitFieldInteger;
use super::types::EnumSignedInteger;
use super::types::EnumUnsignedInteger;
use super::types::SignedEnum;
use super::types::SignedInteger;
use super::types::SignedIntegerNormalizedType;
use super::types::SignedIntegerValue;
use super::types::SignedIntegerValues;
use super::types::SignedIntegers;
use super::types::TagType;
use super::types::Unaligned;
use super::types::UnsignedEnum;
use super::types::UnsignedInteger;
use super::types::UnsignedIntegerNormalizedType;
use super::types::UnsignedIntegerValue;
use super::types::UnsignedIntegerValues;
use super::types::UnsignedIntegers;
use swiss_army_knife::non_zero::new_non_zero_u8;
use crate::image_file_directory::ChildImageFileDirectoriesParseError;
use crate::image_file_directory::tags::public::PublicTagParseError;
use crate::image_file_directory::tags::types::{AsciiStringsParseError, IntegerValueParseError, IntegerValuesParseError};
use crate::image_file_directory::tags::unrecognized::{UnrecognizedTagFinishParseError, UnrecognizedTagParseError};


include!("FinishParseError.rs");
include!("FreeSpace.rs");
include!("RawTagValue.rs");
include!("SpecificTagParseError.rs");
include!("TagParserCommon.rs");
include!("TagParseError.rs");
include!("TagParser.rs");
include!("TagEventHandler.rs");
include!("Version6.rs");
include!("Version6OrBigTiffUnit.rs");
include!("VersionBigTiff.rs");
