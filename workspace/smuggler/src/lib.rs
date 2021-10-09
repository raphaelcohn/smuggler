// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/smuggler/master/COPYRIGHT.


#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![deny(absolute_paths_not_starting_with_crate)]
#![deny(invalid_html_tags)]
#![deny(macro_use_extern_crate)]
#![deny(missing_crate_level_docs)]
#![deny(missing_docs)]
#![deny(pointer_structural_match)]
#![deny(unaligned_references)]
#![deny(unconditional_recursion)]
#![deny(unreachable_patterns)]
#![deny(unused_import_braces)]
#![deny(unused_must_use)]
#![deny(unused_qualifications)]
#![deny(unused_results)]
#![warn(unreachable_pub)]
#![warn(unused_lifetimes)]
#![warn(unused_crate_dependencies)]


#![feature(allocator_api)]
#![feature(arbitrary_enum_discriminant)]
#![feature(const_fn_trait_bound)]
#![feature(core_intrinsics)]
#![feature(maybe_uninit_array_assume_init)]
#![feature(maybe_uninit_uninit_array)]
#![feature(nonnull_slice_from_raw_parts)]
#![feature(option_result_unwrap_unchecked)]
#![feature(slice_ptr_get)]
#![feature(slice_ptr_len)]
#![feature(try_reserve)]


#![doc = include_str!("../README.md")]


use std::alloc::Allocator;
use self::bytes::TiffBytes;
use self::header::HeaderParseError;
use self::image_file_directory::ImageFileDirectoriesParseError;
use std::error;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use crate::bytes::ByteOrder;
use crate::bytes::TiffBytesWithOrder;
use crate::header::parse_header_zeroth_image_file_directory_pointer;
use crate::header::Version;
use crate::header::parse_header_byte_order;
use crate::header::VersionParseError;
use crate::image_file_directory::ImageFileDirectories;
use crate::image_file_directory::pointer::ImageFileDirectoryPointer;
use crate::image_file_directory::tags::parsers::{FreeSpace, TagParserCommon};
use crate::image_file_directory::tags::parsers::Version6;
use crate::image_file_directory::tags::parsers::Version6OrBigTiffUnit;
use crate::image_file_directory::tags::parsers::VersionBigTiff;
use crate::image_file_directory::tags::public::PublicTag;


/// Collections.
pub mod bytes;


/// Header.
pub mod header;


/// Image File Directory (IFD).
pub mod image_file_directory;


/// Offset.
pub mod offset;


include!("Tiff.rs");
include!("TiffParseError.rs");
