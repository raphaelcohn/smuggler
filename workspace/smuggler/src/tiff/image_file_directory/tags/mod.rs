// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


use self::types::AsciiCharacter;
use self::types::TagType;
use self::types::TagTypeTrait;
use self::types::UnalignedF32;
use self::types::UnalignedF64;
use self::types::UnalignedI16;
use self::types::UnalignedI32;
use self::types::UnalignedI64;
use self::types::UnalignedSignedRational;
use self::types::UnalignedU16;
use self::types::UnalignedU32;
use self::types::UnalignedU64;
use self::types::UnsignedIntegerTagType;
use self::types::UnalignedUnsignedRational;
use self::types::SignedRational;
use self::types::UnsignedRational;
use super::DirectoryEntry;
use super::pointer::ImageFileDirectoryPointer;
use likely::unlikely;
use std::alloc::Allocator;
use std::collections::BTreeMap;
use std::collections::TryReserveError;
use std::mem::size_of;
use std::mem::transmute;
use swiss_army_knife::get_unchecked::GetUnchecked;
use crate::collections::TiffBytes;
use crate::collections::Index;
use crate::collections::VecExt;
use crate::collections::ByteOrder;
use std::num::NonZeroU64;
use std::convert::TryInto;
use std::ops::Deref;
use std::cmp::Ordering;
use std::fmt::Debug;
use std::hash::Hash;
use self::types::Unaligned;
use self::types::RationalFraction;
use crate::tiff::image_file_directory::tags::types::UnrecognizedTagValue;
use crate::tiff::image_file_directory::tags::parsers::{TagEventHandler, RawTagValue, TagParserCommon};
use crate::tiff::image_file_directory::tags::parsers::RecursionGuard;
use crate::tiff::image_file_directory::tags::parsers::Version6OrBigTiffUnit;
use crate::tiff::image_file_directory::tags::parsers::SpecificTagParseError;
use std::ptr::NonNull;


/// Parsers.
pub mod parsers;


/// Parsers.
pub mod public;


pub(in crate::tiff::image_file_directory::tags) mod tag_identifiers;


/// Tag types.
pub mod types;


include!("EnumRepresentationU16.rs");
include!("RawTagKey.rs");
include!("Tag.rs");
include!("TagIdentifier.rs");
include!("TagKey.rs");
include!("Tags.rs");
include!("UnrecognizedRepresentationValue.rs");
include!("UnrecognizedTag.rs");
