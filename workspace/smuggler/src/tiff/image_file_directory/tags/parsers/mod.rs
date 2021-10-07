// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


use crate::collections::{ByteOrder, size_of_u64, Byte, CanBeUnaligned};
use crate::collections::Index;
use crate::collections::OverflowError;
use crate::collections::TiffBytes;
use crate::collections::TiffBytesWithOrder;
use crate::tiff::header::Header;
use crate::tiff::image_file_directory::ImageFileDirectoriesParseError;
use crate::tiff::image_file_directory::tags::types::{TagType, Unaligned, AsciiStrings, UnsignedIntegerNormalizedType, SignedIntegerNormalizedType, UnsignedIntegerValue, SignedIntegerValue, EnumSignedInteger, EnumUnsignedInteger, BitFieldInteger, BitField, UnsignedEnum, SignedEnum, UnsignedInteger, SignedInteger};
use crate::tiff::offset::OffsetParseError;
use crate::tiff::offset::Offset;
use likely::unlikely;
use std::alloc::Allocator;
use std::alloc::AllocError;
use std::collections::{TryReserveError, HashSet};
use std::error;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt;
use super::PublicTag;
use super::TagIdentifier;
use super::UnrecognizedTagValue;
use super::super::pointer::ImageFileDirectoryPointer;
use super::super::pointer::ImageFileDirectoryPointerParseError;
use std::cell::{Cell, RefCell};
use std::num::NonZeroU8;
use swiss_army_knife::non_null::new_non_zero_u8;
use crate::tiff::image_file_directory::tags::{UnrecognizedTag, Tag};
use std::ptr::NonNull;
use crate::tiff::FreeSpace;
use std::marker::PhantomData;
use std::ops::Deref;


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
