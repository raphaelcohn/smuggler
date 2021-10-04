// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


use crate::collections::ByteOrder;
use crate::collections::Bytes;
use crate::collections::OverflowError;
use crate::tiff::header::Header;
use crate::tiff::offset::OffsetParseError;
use super::super::pointer::ImageFileDirectoryPointer;
use super::super::pointer::ImageFileDirectoryPointerParseError;
use super::PublicTag;
use super::TagIdentifier;
use super::UnrecognizedTagValue;
use std::alloc::Allocator;
use std::error;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt;
use crate::tiff::image_file_directory::tags::types::TagType;
use std::collections::TryReserveError;


include!("SpecificTagParseError.rs");
include!("TagParser.rs");
include!("TagParseError.rs");
include!("Version6OrBigTiffUnit.rs");
