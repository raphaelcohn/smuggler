// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Converts a C string pointer.
#[inline(always)]
pub fn c_string_pointer_to_path_buf(nul_terminated: *mut c_char) -> Result<Option<PathBuf>, ()>
{
	if unlikely!(nul_terminated.is_null())
	{
		return Ok(None);
	}

	let c_str = unsafe { CStr::from_ptr(nul_terminated) };

	let bytes = c_str.to_bytes();
	if bytes.len() == 0
	{
		Err(())
	}
	else
	{
		let os_str: &OsStr = OsStrExt::from_bytes(bytes);
		Ok(Some(PathBuf::from(os_str)))
	}
}