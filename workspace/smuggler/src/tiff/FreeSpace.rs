// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


#[derive(Debug)]
struct FreeSpace
{
}

impl FreeSpace
{
	#[inline(always)]
	fn record_header(&mut self, version: Version)
	{
		use Version::*;
		
		let size_in_bytes = match version
		{
			_6 => 8,
			
			BigTiff => 16,
		};
		self.record_used_space_slice(0, size_in_bytes);
	}
	
	#[inline(always)]
	fn record_used_space_slice(&mut self, index: Index, size_in_bytes: u64)
	{
	}
}
