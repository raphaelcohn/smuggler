// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


use crate::collections::ByteOrder;
use crate::collections::TiffBytesWithOrder;
use crate::collections::OverflowError;
use crate::collections::TiffBytes;
use crate::image_file_directory::pointer::ImageFileDirectoryPointer;
use crate::image_file_directory::pointer::ImageFileDirectoryPointerParseError;
use std::error;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use crate::image_file_directory::tags::parsers::Version6OrBigTiffUnit;


include!("BigTiffHeaderParseError.rs");
include!("ByteOrderParseError.rs");
include!("HeaderParseError.rs");
include!("parse_header_byte_order.rs");
include!("parse_header_zeroth_image_file_directory_pointer.rs");
include!("Version.rs");
include!("VersionParseError.rs");
include!("ZerothImageFileDirectoryPointerParseError.rs");