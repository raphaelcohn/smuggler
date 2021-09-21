// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


use std::error;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use crate::collections::{Bytes, ByteOrder, OverflowError};
use std::alloc::Allocator;
use super::PublicTag;
use super::TagIdentifier;
use crate::tiff::image_file_directory::tags::types::TagType;
use crate::tiff::image_file_directory::tags::types::UnsignedIntegerTagType;
use crate::tiff::image_file_directory::pointer::{ImageFileDirectoryPointer, ImageFileDirectoryPointerParseError};
use crate::tiff::offset::OffsetParseError;
use crate::tiff::header::Header;


include!("Version6OrBigTiffUnit.rs");
include!("PublicTagParser.rs");
include!("TagParser.rs");
include!("TagParseError.rs");
