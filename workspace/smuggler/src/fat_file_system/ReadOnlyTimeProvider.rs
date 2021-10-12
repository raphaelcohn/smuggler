// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct ReadOnlyTimeProvider;

impl TimeProvider for ReadOnlyTimeProvider
{
	fn get_current_date(&self) -> Date
	{
		panic!("Should never be called")
	}
	
	fn get_current_date_time(&self) -> DateTime
	{
		panic!("Should never be called")
	}
}
