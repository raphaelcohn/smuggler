// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


use crate::collections::TiffBytes;
use crate::collections::VecExt;
use likely::unlikely;
use self::parsers::RawTagValue;
use self::parsers::SpecificTagParseError;
use self::parsers::TagEventHandler;
use self::parsers::TagParserCommon;
use self::parsers::Version6OrBigTiffUnit;
use self::types::TagType;
use self::types::UnrecognizedTagValue;
use std::alloc::Allocator;
use std::cmp::Ordering;
use std::collections::TryReserveError;
use std::convert::TryInto;
use std::fmt::Debug;
use std::hash::Hash;
use std::num::NonZeroU64;
use std::ops::Deref;
use swiss_army_knife::get_unchecked::GetUnchecked;


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
