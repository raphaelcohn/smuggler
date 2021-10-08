// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


use crate::tiff::image_file_directory::pointer::ImageFileDirectoryPointer;
use crate::tiff::image_file_directory::pointer::ImageFileDirectoryPointerParseError;
use crate::tiff::image_file_directory::tags::parsers::Version6OrBigTiffUnit;
use crate::tiff::image_file_directory::tags::types::Unaligned;
use crate::tiff::offset::Offset;
use crate::tiff::offset::OffsetParseError;
use self::pointer_to_index_lengths::CheckedPointerToIndexLength;
use self::pointer_to_index_lengths::PointerToIndexLength;
use self::pointer_to_index_lengths::UncheckedPointerToIndexLength;
use std::alloc::Allocator;
use std::collections::TryReserveError;
use std::error;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt;
use std::hash::Hash;
use std::mem::size_of;
use std::mem::transmute;
use std::num::NonZeroUsize;
use std::ptr::NonNull;
use std::ptr;
use std::slice::from_raw_parts;
use std::slice::from_raw_parts_mut;
use swiss_army_knife::byte_swap::Unaligned as _;
use swiss_army_knife::byte_swap::Unaligned16;
use swiss_army_knife::byte_swap::Unaligned32;
use swiss_army_knife::byte_swap::Unaligned64;
use swiss_army_knife::byte_swap;
use swiss_army_knife::get_unchecked::AsUsizeIndex;
use swiss_army_knife::non_zero::new_non_null;
use swiss_army_knife::non_zero::new_non_zero_usize;


mod pointer_to_index_lengths;


include!("Byte.rs");
include!("ByteOrder.rs");
include!("CanBeUnaligned.rs");
include!("FileLength.rs");
include!("Index.rs");
include!("NonNullExt.rs");
include!("OverflowError.rs");
include!("size_of_u64.rs");
include!("TiffBytes.rs");
include!("TiffBytesWithOrder.rs");
include!("VecExt.rs");
