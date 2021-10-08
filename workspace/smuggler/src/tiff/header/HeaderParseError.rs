// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


/// A parse error.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) enum HeaderParseError
{
	#[allow(missing_docs)]
	ByteOrderParse(ByteOrderParseError),
	
	#[allow(missing_docs)]
	VersionParse
	{
		cause: VersionParseError,
	
		byte_order: ByteOrder,
	},
	
	#[allow(missing_docs)]
	BigTiffHeaderParse
	{
		cause: BigTiffHeaderParseError,
		
		version: Version,
	},
	
	#[allow(missing_docs)]
	ZerothImageFileDirectoryPointerParse
	{
		cause: ZerothImageFileDirectoryPointerParseError,
		
		version: Version,
	},
}

impl Display for HeaderParseError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for HeaderParseError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use HeaderParseError::*;
		
		match self
		{
			ByteOrderParse(cause) => Some(cause),
			
			VersionParse { cause, .. } => Some(cause),
			
			BigTiffHeaderParse { cause, .. } => Some(cause),
			
			ZerothImageFileDirectoryPointerParse { cause, .. } => Some(cause),
			
			_ => None,
		}
	}
}
