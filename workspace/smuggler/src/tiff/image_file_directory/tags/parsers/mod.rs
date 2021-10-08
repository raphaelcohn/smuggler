// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


use crate::collections::Byte;
use crate::collections::ByteOrder;
use crate::collections::CanBeUnaligned;
use crate::collections::Index;
use crate::collections::OverflowError;
use crate::collections::TiffBytes;
use crate::collections::TiffBytesWithOrder;
use crate::collections::size_of_u64;
use crate::tiff::FreeSpace;
use crate::tiff::offset::Offset;
use crate::tiff::offset::OffsetParseError;
use likely::unlikely;
use std::alloc::AllocError;
use std::alloc::Allocator;
use std::cell::Cell;
use std::cell::RefCell;
use std::collections::{TryReserveError, HashSet};
use std::error;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt;
use std::marker::PhantomData;
use std::num::NonZeroU8;
use std::ops::Deref;
use std::ptr::NonNull;
use super::Tag;
use super::TagIdentifier;
use super::UnrecognizedTag;
use super::public::PublicTagParseError;
use super::super::ImageFileDirectoriesParseError;
use super::super::pointer::ImageFileDirectoryPointer;
use super::super::pointer::ImageFileDirectoryPointerParseError;
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


include!("RawTagValue.rs");
include!("RecursionGuard.rs");
include!("Recursion.rs");
include!("SpecificTagParseError.rs");
include!("TagParserCommon.rs");
include!("TagParseError.rs");
include!("TagParser.rs");
include!("TagEventHandler.rs");
include!("UnrecognizedTagParser.rs");
include!("Version6OrBigTiffUnit.rs");
