// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


/// An unrecognized tag value.
#[derive(Debug, Clone, PartialOrd, PartialEq)]
#[repr(u16)]
pub enum UnrecognizedTagValue<'tiff_bytes, A: Allocator>
{
	// Value 0 is not defined.
	
	/// From before TIFF version 6.
	BYTE(&'tiff_bytes [u8]) = TagType::Byte,
	
	/// From before TIFF version 6.
	ASCII(AsciiStrings<'tiff_bytes, A>) = TagType::Ascii,
	
	/// From before TIFF version 6.
	SHORT(&'tiff_bytes [Unaligned<u16>]) = TagType::Short,
	
	/// From before TIFF version 6.
	LONG(&'tiff_bytes [Unaligned<u32>]) = TagType::Long,
	
	/// From before TIFF version 6.
	RATIONAL(&'tiff_bytes [Unaligned<RationalFraction<u32>>]) = TagType::Rational,
	
	/// Defined in TIFF version 6.
	SBYTE(&'tiff_bytes [i8]) = TagType::Sbyte,
	
	/// Defined in TIFF version 6.
	UNDEFINED(&'tiff_bytes [u8]) = TagType::Undefined,
	
	/// Defined in TIFF version 6.
	SSHORT(&'tiff_bytes [Unaligned<i16>]) = TagType::Sshort,
	
	/// Defined in TIFF version 6.
	SLONG(&'tiff_bytes [Unaligned<i32>]) = TagType::Slong,
	
	/// Defined in TIFF version 6.
	SRATIONAL(&'tiff_bytes [Unaligned<RationalFraction<i32>>]) = TagType::Srational,
	
	/// Defined in TIFF version 6.
	FLOAT(&'tiff_bytes [Unaligned<f32>]) = TagType::Float,
	
	/// Defined in TIFF version 6.
	DOUBLE(&'tiff_bytes [Unaligned<f64>]) = TagType::Double,
	
	/// Proposed by Adobe in 1995 in PageMaker technical notes and unofficially adopted.
	IFD(Vec<ImageFileDirectories<A, UnrecognizedTag<'tiff_bytes, A>>, A>) = TagType::Ifd,
	
	// Values 14 and 15 don't seem to be defined.
	
	/// Defined in TIFF version BigTiff.
	LONG8(&'tiff_bytes [Unaligned<u64>]) = TagType::Long8,
	
	/// Defined in TIFF version BigTiff.
	SLONG8(&'tiff_bytes [Unaligned<i64>]) = TagType::Slong8,
	
	/// Defined in TIFF version BigTiff.
	IFD8(Vec<ImageFileDirectories<A, UnrecognizedTag<'tiff_bytes, A>>, A>) = TagType::Ifd8,
}

impl<'tiff_bytes, A: Allocator + Clone> UnrecognizedTagValue<'tiff_bytes, A>
{
	#[inline(always)]
	pub(in crate::image_file_directory::tags) fn parse<'allocator, TB: TiffBytes, Version: 'tiff_bytes + Version6OrBigTiffVersion>(common: &mut TagParserCommon<'tiff_bytes, 'allocator, TB, A, Version>, tag_type: TagType, raw_tag_value: RawTagValue<'tiff_bytes>) -> Result<Self, UnrecognizedTagParseError>
	{
		use UnrecognizedTagValue::*;
		
		let this = match tag_type
		{
			TagType::BYTE => BYTE(raw_tag_value.byte_slice()),
			
			TagType::ASCII => ASCII(raw_tag_value.ascii_strings(common)?),
			
			TagType::SHORT => SHORT(raw_tag_value.unaligned_slice(common)),
			
			TagType::LONG => LONG(raw_tag_value.unaligned_slice(common)),
			
			TagType::RATIONAL => RATIONAL(raw_tag_value.unaligned_slice(common)),
			
			TagType::SBYTE => SBYTE(raw_tag_value.byte_slice()),
			
			TagType::UNDEFINED => UNDEFINED(raw_tag_value.byte_slice()),
			
			TagType::SSHORT => SSHORT(raw_tag_value.unaligned_slice(common)),
			
			TagType::SLONG => SLONG(raw_tag_value.unaligned_slice(common)),
			
			TagType::SRATIONAL => SRATIONAL(raw_tag_value.unaligned_slice(common)),
			
			TagType::FLOAT => FLOAT(raw_tag_value.unaligned_slice(common)),
			
			TagType::DOUBLE => DOUBLE(raw_tag_value.unaligned_slice(common)),
			
			TagType::IFD => IFD(ImageFileDirectories::parse_child_image_file_directories::<UnrecognizedTagParser, _, Version>(common, raw_tag_value)?),
			
			TagType::LONG8 => LONG8(raw_tag_value.unaligned_slice(common)),
			
			TagType::SLONG8 => SLONG8(raw_tag_value.unaligned_slice(common)),
			
			TagType::IFD8 => IFD8(ImageFileDirectories::parse_child_image_file_directories::<UnrecognizedTagParser, _, Version>(common, raw_tag_value)?),
		};
		Ok(this)
	}
}
