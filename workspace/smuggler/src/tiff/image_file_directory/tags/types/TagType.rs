// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


/// A tag type code.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(u16)]
pub enum TagType
{
	// Value 0 is not defined.
	
	/// Legacy from before TIFF version 6.
	BYTE = TagType::Byte,
	
	/// Legacy from before TIFF version 6.
	ASCII = TagType::Ascii,
	
	/// Legacy from before TIFF version 6.
	SHORT = TagType::Short,
	
	/// Legacy from before TIFF version 6.
	LONG = TagType::Long,
	
	/// Legacy from before TIFF version 6.
	RATIONAL = TagType::Rational,
	
	/// Defined in TIFF version 6.
	SBYTE = TagType::Sbyte,
	
	/// Defined in TIFF version 6.
	UNDEFINED = TagType::Undefined,
	
	/// Defined in TIFF version 6.
	SSHORT = TagType::Sshort,
	
	/// Defined in TIFF version 6.
	SLONG = TagType::Slong,
	
	/// Defined in TIFF version 6.
	SRATIONAL = TagType::Srational,
	
	/// Defined in TIFF version 6.
	FLOAT = TagType::Float,
	
	/// Defined in TIFF version 6.
	DOUBLE = TagType::Double,
	
	/// Proposed by Adobe in 1995 in PageMaker technical notes and unofficially adopted.
	IFD = TagType::Ifd,
	
	// Values 14 and 15 don't seem to be defined.
	
	/// Defined in TIFF version BigTiff.
	LONG8 = TagType::Long8,
	
	/// Defined in TIFF version BigTiff.
	SLONG8 = TagType::Slong8,
	
	/// Defined in TIFF version BigTiff.
	IFD8 = TagType::Ifd8,
}

impl TagType
{
	const Unrecognized0: u16 = 0;
	
	const Byte: u16 = 1;
	
	const Ascii: u16 = 2;
	
	const Short: u16 = 3;
	
	const Long: u16 = 4;
	
	const Rational: u16 = 5;
	
	const Sbyte: u16 = 6;
	
	const Undefined: u16 = 7;
	
	const Sshort: u16 = 8;
	
	const Slong: u16 = 9;
	
	const Srational: u16 = 10;
	
	const Float: u16 = 11;
	
	const Double: u16 = 12;
	
	const Ifd: u16 = 13;
	
	const Unrecognized14: u16 = 14;
	
	const Unrecognized15: u16 = 15;
	
	const Long8: u16 = 16;
	
	const Slong8: u16 = 17;
	
	const Ifd8: u16 = 18;
	
	#[inline(always)]
	pub(in crate::tiff::image_file_directory) fn parse(tag_type: u16) -> Result<(Self, u64), TagParseError>
	{
		#[inline(always)]
		const fn unrecognized(tag_type: u16) -> Result<(Self, u64), TagParseError>
		{
			Err(TagParseError::UnrecognizedTagType { tag_type })
		}
		
		use TagType::*;
		
		let ok = match tag_type
		{
			TagType::Unrecognized0 => return unrecognized(TagType::Unrecognized0),
			
			TagType::Byte => (BYTE, 1),
			
			TagType::Ascii => (ASCII, 1),
			
			TagType::Short => (SHORT, 2),
			
			TagType::Long => (LONG, 4),
			
			TagType::Rational => (RATIONAL, 8),
			
			TagType::Sbyte => (SBYTE, 1),
			
			TagType::Undefined => (UNDEFINED, 1),
			
			TagType::Sshort => (SSHORT, 1),
			
			TagType::Slong => (SLONG, 4),
			
			TagType::Srational => (SRATIONAL, 8),
			
			TagType::Float => (FLOAT, 4),
			
			TagType::Double => (DOUBLE, 8),
			
			TagType::Ifd => (IFD, 4),
			
			TagType::Unrecognized14 => return unrecognized(TagType::Unrecognized14),
			
			TagType::Unrecognized15 => return unrecognized(TagType::Unrecognized15),
			
			TagType::Long8 => (LONG8, 8),
			
			TagType::Slong8 => (SLONG8, 8),
			
			TagType::Ifd8 => (IFD8, 8),
			
			_ => return unrecognized(tag_type),
		};
		Ok(ok)
	}
	
	#[inline(always)]
	const fn invalid<TT>(tag_type: Self) -> Result<TT, SpecificTagParseError>
	{
		Err(SpecificTagParseError::InvalidTagTypeForTagIdentifier(tag_type))
	}
}
