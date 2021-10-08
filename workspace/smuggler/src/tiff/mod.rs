// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


use crate::collections::Index;
use self::header::Header;
use self::header::HeaderParseError;
use self::image_file_directory::ImageFileDirectoriesParseError;
use std::error;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use crate::tiff::header::Version;


/// Header.
pub mod header;


/// Image File Directory (IFD).
pub mod image_file_directory;


/// Offset.
pub mod offset;


include!("FreeSpace.rs");
include!("TiffParseError.rs");
