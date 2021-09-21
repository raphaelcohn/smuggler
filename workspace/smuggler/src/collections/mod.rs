// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


use self::pointer_to_index_lengths::CheckedPointerToIndexLength;
use self::pointer_to_index_lengths::PointerToIndexLength;
use self::pointer_to_index_lengths::UncheckedPointerToIndexLength;
use crate::tiff::header::Version;
use crate::tiff::header::Header;
use crate::tiff::image_file_directory::pointer::ImageFileDirectoryPointer;
use crate::tiff::image_file_directory::pointer::ImageFileDirectoryPointerParseError;
use crate::tiff::offset::Offset;
use crate::tiff::offset::OffsetParseError;
use likely::unlikely;
use std::alloc::Allocator;
use std::collections::TryReserveError;
use std::convert::TryInto;
use std::error;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt;
use std::mem::size_of;
use std::ptr;
use std::slice::from_raw_parts;
use swiss_army_knife::get_unchecked::AsUsizeIndex;


mod pointer_to_index_lengths;


include!("Bytes.rs");
include!("ByteOrder.rs");
include!("FileLength.rs");
include!("OverflowError.rs");
include!("size_of_u64.rs");
include!("u64_index_to_usize_index.rs");
include!("VecExt.rs");
