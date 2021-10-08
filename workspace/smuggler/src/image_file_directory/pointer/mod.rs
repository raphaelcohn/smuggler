// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


use crate::collections::Index;
use crate::offset::Offset;
use crate::offset::OffsetParseError;
use likely::unlikely;
use std::error;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt;
use std::num::NonZeroU64;
use swiss_army_knife::non_zero::new_non_zero_u64;


include!("ImageFileDirectoryPointer.rs");
include!("ImageFileDirectoryPointerParseError.rs");
