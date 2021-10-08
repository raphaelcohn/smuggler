// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


use crate::collections::Index;
use crate::collections::OverflowError;
use crate::collections::TiffBytes;
use crate::collections::TiffBytesWithOrder;
use crate::collections::VecExt;
use crate::collections::size_of_u64;
use crate::tiff::header::Version;
use likely::unlikely;
use self::pointer::ImageFileDirectoryPointer;
use self::pointer::ImageFileDirectoryPointerParseError;
use self::tags::Tag;
use self::tags::TagIdentifier;
use self::tags::Tags;
use self::tags::parsers::RawTagValue;
use self::tags::parsers::Recursion;
use self::tags::parsers::SpecificTagParseError;
use self::tags::parsers::TagParseError;
use self::tags::parsers::TagParser;
use self::tags::parsers::TagParserCommon;
use self::tags::parsers::Version6OrBigTiffUnit;
use self::tags::public::PublicTagParser;
use self::tags::types::TagType;
use std::alloc::{Allocator, Global};
use std::collections::TryReserveError;
use std::error;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt;
use std::num::NonZeroU64;
use std::ops::Deref;
use swiss_army_knife::non_zero::new_non_zero_u64;
use crate::tiff::image_file_directory::tags::parsers::FinishTagParseError;
use crate::tiff::image_file_directory::tags::public::PublicTag;


/// Pointer.
pub mod pointer;


/// Tags.
pub mod tags;


include!("ImageFileDirectories.rs");
include!("ImageFileDirectoriesParseError.rs");
include!("ImageFileDirectory.rs");
include!("ImageFileDirectoryParseError.rs");
