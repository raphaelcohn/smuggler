// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


use crate::collections::ByteOrder;
use crate::collections::CanBeUnaligned;
use crate::collections::FileLength;
use crate::collections::Index;
use crate::collections::OverflowError;
use crate::collections::TiffBytes;
use likely::unlikely;
use std::error;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt;


include!("Offset.rs");
include!("OffsetParseError.rs");
