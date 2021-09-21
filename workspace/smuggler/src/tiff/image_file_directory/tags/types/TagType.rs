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
	SBYTE = 6,
	
	/// Defined in TIFF version 6.
	UNDEFINED = 7,
	
	/// Defined in TIFF version 6.
	SSHORT = 8,
	
	/// Defined in TIFF version 6.
	SLONG = 9,
	
	/// Defined in TIFF version 6.
	SRATIONAL = 10,
	
	/// Defined in TIFF version 6.
	FLOAT = 11,
	
	/// Defined in TIFF version 6.
	DOUBLE = 12,
	
	/// Proposed by Adobe in 1995 in PageMaker technical notes and unofficially adopted.
	IFD = TagType::Ifd,
	
	// Values 14 and 15 don't seem to be defined.
	
	/// Defined in TIFF version BigTiff.
	LONG8 = TagType::Long8,
	
	/// Defined in TIFF version BigTiff.
	SLONG8 = 17,
	
	/// Defined in TIFF version BigTiff.
	IFD8 = TagType::Ifd8,
}

impl TagType
{
	#[inline(always)]
	pub(in crate::tiff::image_file_directory) fn parse(tag_type: u16) -> Result<Self, TagParseError>
	{
		use TagParseError::UnrecognizedTagType;
		
		#[inline(always)]
		const fn ok(tag_type: u16) -> Result<TagType, TagParseError>
		{
			Ok(unsafe { transmute(tag_type) })
		}
		
		#[inline(always)]
		const fn error(tag_type: u16) -> Result<TagType, TagParseError>
		{
			Err(UnrecognizedTagType(tag_type))
		}
		
		match tag_type
		{
			Self::Unrecognized0 => Self::unrecognized(tag_type),
			
			Self::Byte ..= Self::Ifd => Self::ok(tag_type),
			
			Self::Unrecognized14 ..= Self::Unrecognized15 => Self::unrecognized(tag_type),
			
			Self::Long8 ..= Self::Ifd8 => Self::ok(tag_type),
			
			_ => Self::unrecognized(tag_type)
		}
	}
	
	#[inline(always)]
	pub(in crate::tiff::image_file_directory) fn parse_unsigned(tag_type: u16, tag_identifier: TagIdentifier) -> Result<Self, TagParseError>
	{
		use TagType::*;
		
		match tag_type
		{
			Self::Unrecognized0 => Self::unrecognized(tag_type),
			
			Self::Byte => Ok(BYTE),
			
			Self::Ascii => Self::invalid(tag_type, tag_identifier),
			
			Self::Short => Ok(SHORT),
			
			Self::Long => Ok(LONG),
			
			Self::Rational ..= Self::Ifd => Self::invalid(tag_type, tag_identifier),
			
			Self::Unrecognized14 ..= Self::Unrecognized15 => Self::unrecognized(tag_type),
			
			Self::Long8 => Ok(LONG8),
			
			_ => Self::unrecognized(tag_type)
		}
	}
	
	const Unrecognized0: u16 = 0;
	
	const Unrecognized14: u16 = 14;
	
	const Unrecognized15: u16 = 15;
	
	const Byte: u16 = 1;
	
	const Ascii: u16 = 2;
	
	const Short: u16 = 3;
	
	const Long: u16 = 4;
	
	const Rational: u16 = 5;
	
	const Ifd: u16 = 13;
	
	const Long8: u16 = 16;
	
	const Ifd8: u16 = 18;
	
	#[inline(always)]
	const fn ok<TT>(tag_type: u16) -> Result<TT, TagParseError>
	{
		Ok(unsafe { transmute(tag_type) })
	}
	
	#[inline(always)]
	const fn unrecognized<TT>(tag_type: u16) -> Result<TT, TagParseError>
	{
		Err(TagParseError::UnrecognizedTagType(tag_type))
	}
	
	#[inline(always)]
	const fn invalid<TT>(tag_type: u16, tag_identifier: TagIdentifier) -> Result<TT, TagParseError>
	{
		Err(TagParseError::InvalidTagTypeForTagIdentifier(unsafe { transmute(tag_type) }, tag_identifier))
	}
}
