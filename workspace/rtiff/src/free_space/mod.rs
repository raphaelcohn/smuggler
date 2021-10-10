// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


use crate::bytes::{FileLength, TiffBytesWithOrder};
use crate::bytes::Index;
use crate::bytes::TiffBytes;
use crate::collections::VecExt;
use likely::unlikely;
use std::alloc::Allocator;
use std::cmp::Ordering;
use std::collections::TryReserveError;
use std::error;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::num::NonZeroUsize;
use std::ops::Deref;
use std::slice::from_raw_parts;
use swiss_army_knife::get_unchecked::GetUnchecked;
use swiss_army_knife::non_zero::new_non_zero_usize;


include!("FreeSpace.rs");
include!("FreeSpaceOutOfMemoryError.rs");
include!("FreeSpaceVector.rs");
include!("SpaceRange.rs");
