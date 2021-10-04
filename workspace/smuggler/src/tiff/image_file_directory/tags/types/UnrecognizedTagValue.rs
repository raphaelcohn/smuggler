// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


/// An unrecognized tag value.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(u16)]
pub enum UnrecognizedTagValue<'a, A: Allocator>
{
	// Value 0 is not defined.
	
	/// From before TIFF version 6.
	BYTE(&'a [u8]) = TagType::Byte,
	
	/// From before TIFF version 6.
	ASCII(AsciiStrings<'a, A>) = TagType::Ascii,
	
	/// From before TIFF version 6.
	SHORT(&'a [Unaligned<u16>]) = TagType::Short,
	
	/// From before TIFF version 6.
	LONG(&'a [Unaligned<u32>]) = TagType::Long,
	
	/// From before TIFF version 6.
	RATIONAL(&'a [Unaligned<RationalFraction<u32>>]) = TagType::Rational,
	
	/// Defined in TIFF version 6.
	SBYTE(&'a [i8]) = TagType::Sbyte,
	
	/// Defined in TIFF version 6.
	UNDEFINED(&'a [u8]) = TagType::Undefined,
	
	/// Defined in TIFF version 6.
	SSHORT(&'a [Unaligned<i16>]) = TagType::Sshort,
	
	/// Defined in TIFF version 6.
	SLONG(&'a [Unaligned<i32>]) = TagType::Slong,
	
	/// Defined in TIFF version 6.
	SRATIONAL(&'a [Unaligned<RationalFraction<i32>>]) = TagType::Srational,
	
	/// Defined in TIFF version 6.
	FLOAT(&'a [Unaligned<f32>]) = TagType::Float,
	
	/// Defined in TIFF version 6.
	DOUBLE(&'a [Unaligned<f64>]) = TagType::Double,
	
	/// Proposed by Adobe in 1995 in PageMaker technical notes and unofficially adopted.
	IFD(ImageFileDirectoryPointer) = TagType::Ifd,
	
	// Values 14 and 15 don't seem to be defined.
	
	/// Defined in TIFF version BigTiff.
	LONG8(&'a [Unaligned<u64>]) = TagType::Long8,
	
	/// Defined in TIFF version BigTiff.
	SLONG8(&'a [Unaligned<i64>]) = TagType::Slong8,
	
	/// Defined in TIFF version BigTiff.
	IFD8(ImageFileDirectoryPointer) = TagType::Ifd8,
}

impl<'a, A: Allocator> UnrecognizedTagValue<'a, A>
{
	pub(in crate::tiff::image_file_directory::tags) fn parse<Unit: Version6OrBigTiffUnit, B: Bytes>(tag_type: u16, tiff_bytes: &mut B, count: u64, byte_order: ByteOrder, offset_or_value_union_index: u64, allocator: A) -> Result<Self, SpecificTagParseError>
	{
		use SpecificTagParseError::*;
		use UnrecognizedTagValue::*;
		
		let this = match tag_type
		{
			TagType::Unrecognized0 => return TagType::unrecognized(),
			
			TagType::Byte => BYTE(tiff_bytes.byte_slice::<Unit>(count, offset_or_value_union_index)?),
			
			TagType::Ascii => ASCII(tiff_bytes.ascii_strings::<Unit>(count, offset_or_value_union_index, allocator)?),
			
			TagType::Short => SHORT(tiff_bytes.short_slice::<Unit>(count, offset_or_value_union_index, byte_order)?),
			
			TagType::Long => LONG(tiff_bytes.long_slice::<Unit>(count, offset_or_value_union_index, byte_order)?),
			
			TagType::Rational => RATIONAL(tiff_bytes.rational_slice::<Unit>(count, offset_or_value_union_index)?),
			
			TagType::Sbyte => SBYTE(tiff_bytes.sbyte_slice::<Unit>(count, offset_or_value_union_index)?),
			
			TagType::Undefined => UNDEFINED(tiff_bytes.undefined_slice::<Unit>(count, offset_or_value_union_index)?),
			
			TagType::Sshort => SSHORT(tiff_bytes.sshort_slice::<Unit>(count, offset_or_value_union_index, byte_order)?),
			
			TagType::Slong => SLONG(tiff_bytes.slong_slice::<Unit>(count, offset_or_value_union_index, byte_order)?),
			
			TagType::Srational => SRATIONAL(tiff_bytes.srational_slice::<Unit>(count, offset_or_value_union_index)?),
			
			TagType::Float => FLOAT(tiff_bytes.float_slice::<Unit>(count, offset_or_value_union_index)?),
			
			TagType::Double => DOUBLE(tiff_bytes.double_slice::<Unit>(count, offset_or_value_union_index)?),
			
			TagType::Ifd => IFD(),
			
			TagType::Unrecognized14 => return TagType::unrecognized(),
			
			TagType::Unrecognized15 => return TagType::unrecognized(),
			
			TagType::Long8 => LONG8(tiff_bytes.long8_slice::<Unit>(count, offset_or_value_union_index, byte_order)?),
			
			TagType::Slong8 => SLONG8(tiff_bytes.slong8_slice::<Unit>(count, offset_or_value_union_index, byte_order)?),
			
			TagType::Ifd8 => IFD8(),
			
			_ => return TagType::unrecognized(),
		};
		Ok(this)
	}
}
