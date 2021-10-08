// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


/// A parse error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OffsetsArrayParseError
{
	#[allow(missing_docs)]
	ByteCountsWithoutOffsets,
	
	#[allow(missing_docs)]
	MismatchedArrayLengths
	{
		offsets_length: u64,
		
		byte_counts_length: u64,
	},
	
	#[allow(missing_docs)]
	IntegerValuesParse(IntegerValuesParseError),
	
	#[allow(missing_docs)]
	CouldNotAllocateMemoryForOffsets(TryReserveError),
	
	#[allow(missing_docs)]
	Overflow(OverflowError),
}

impl Display for OffsetsArrayParseError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for OffsetsArrayParseError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use OffsetsArrayParseError::*;
		
		match self
		{
			IntegerValuesParse(cause) => Some(cause),
			
			CouldNotAllocateMemoryForOffsets(cause) => Some(cause),
			
			Overflow(cause) => Some(cause),
			
			_ => None,
		}
	}
}
