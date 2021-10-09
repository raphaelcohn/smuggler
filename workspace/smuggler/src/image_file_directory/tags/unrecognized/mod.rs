// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


use crate::bytes::TiffBytes;
use crate::image_file_directory::ChildImageFileDirectoriesParseError;
use crate::image_file_directory::ImageFileDirectories;
use std::alloc::Allocator;
use std::error;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt;
use super::Tag;
use super::tag_identifiers::TagIdentifier;
use super::parsers::RawTagValue;
use super::parsers::TagEventHandler;
use super::parsers::TagParser;
use super::parsers::TagParserCommon;
use crate::bytes::versions::Version6OrBigTiffUnit;
use super::values::AsciiStrings;
use super::values::AsciiStringsParseError;
use super::values::RationalFraction;
use super::TagType;
use super::values::Unaligned;


include!("UnrecognizedTagFinishParseError.rs");
include!("UnrecognizedRepresentationValue.rs");
include!("UnrecognizedTag.rs");
include!("UnrecognizedTagParseError.rs");
include!("UnrecognizedTagParser.rs");
include!("UnrecognizedTagValue.rs");
