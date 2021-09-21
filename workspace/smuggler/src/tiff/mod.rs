// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


use crate::collections::ByteOrder;
use crate::collections::VecExt;
use crate::collections::Bytes;
use likely::unlikely;
use self::header::Header;
use self::header::HeaderParseError;
use self::image_file_directory::ImageFileDirectoriesParseError;
use self::parsing::DirectoryElementContentParseError;
use self::parsing::DirectoryElementParseError;
use self::parsing::TiffHeaderParseError;
use self::parsing::ImageFileDirectoryParseError;
use self::parsing::OffsetParseError;
use self::tag_types::AsciiCharacter;
use self::tag_types::UnalignedF32;
use self::tag_types::UnalignedF64;
use self::tag_types::UnalignedI16;
use self::tag_types::UnalignedI32;
use self::tag_types::UnalignedI64;
use self::tag_types::UnalignedSignedRational;
use self::tag_types::UnalignedU16;
use self::tag_types::UnalignedU32;
use self::tag_types::UnalignedU64;
use self::tag_types::UnalignedUnsignedRational;
use swiss_army_knife::get_unchecked::GetUnchecked;
use swiss_army_knife::non_zero::new_non_zero_u16;
use swiss_army_knife::non_zero::new_non_zero_u32;
use swiss_army_knife::non_zero::new_non_zero_u64;
use std::error;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::mem::size_of;
use std::num::NonZeroU16;
use std::num::NonZeroU64;
use std::num::NonZeroU32;
use std::ops::RangeInclusive;
use std::alloc::Allocator;
use std::slice::from_raw_parts;


/// Header.
pub mod header;


/// Image File Directory (IFD).
pub mod image_file_directory;


/// Offset.
pub mod offset;


include!("TiffParseError.rs");
