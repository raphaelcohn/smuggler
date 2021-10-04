// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// An extension trait for `OsStr`.
pub trait OsStrExtMore
{
	/// Converts as `OsStr` to a `CString`.
	fn os_str_to_c_string(&self) -> CString;
}

impl OsStrExtMore for OsStr
{
	#[inline(always)]
	fn os_str_to_c_string(&self) -> CString
	{
		CString::new(self.as_bytes()).expect("os_str should not contain interior ASCII NULs")
	}
}
