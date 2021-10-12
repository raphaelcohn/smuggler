// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


/*
	We should use the file system metadata that can reasonably be present on a VFAT / FAT32 / exFAT file system.
 */

// pub struct FileAuthenticatedData
// {
// 	/// 10ms accuracy.
// 	created_date_time: (),
//
// 	/// Two second accuracy on FAT32, 10ms accuracy on exFAT.
// 	last_modified_date_time: (),
//
// 	/// Two second accuracy on FAT32, 10ms accuracy on exFAT.
// 	last_accessed_data_time: (),
//
// 	/// UTF-16 Little Endian.
// 	///
// 	/// Theoretically, Control Code group C1 (0x00 to 0x1F), slash, backslash, colon, asterisk, question mark, double quite, less than, greater than and pipe are not permitted.
// 	file_name: ArrayVec<u8, 256>,
//
// 	read_only_attribute: bool,
//
// 	hidden_attribute: bool,
//
// 	system_attribute: bool,
//
// 	archive_attribute: bool,
// }
