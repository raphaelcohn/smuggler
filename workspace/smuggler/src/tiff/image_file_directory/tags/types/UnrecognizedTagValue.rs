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
	IFD(Vec<ImageFileDirectories<'a, A, UnrecognizedTag<'a, A>>, A>) = TagType::Ifd,
	
	// Values 14 and 15 don't seem to be defined.
	
	/// Defined in TIFF version BigTiff.
	LONG8(&'a [Unaligned<u64>]) = TagType::Long8,
	
	/// Defined in TIFF version BigTiff.
	SLONG8(&'a [Unaligned<i64>]) = TagType::Slong8,
	
	/// Defined in TIFF version BigTiff.
	IFD8(Vec<ImageFileDirectories<'a, A, UnrecognizedTag<'a, A>>, A>) = TagType::Ifd8,
}

impl<'a, A: Allocator + Copy> UnrecognizedTagValue<'a, A>
{
	#[inline(always)]
	pub(in crate::tiff::image_file_directory::tags) fn parse<Unit: Version6OrBigTiffUnit, TB: TiffBytes>(tag_type: TagType, tiff_bytes: &mut TB, count: u64, byte_order: ByteOrder, slice: NonNull<[u8]>, recursion_guard: &RecursionGuard, allocator: A) -> Result<Self, SpecificTagParseError>
	{
		use UnrecognizedTagValue::*;
		
		let this = match tag_type
		{
			TagType::BYTE => BYTE(u8::byte_slice(slice)),
			
			TagType::ASCII => ASCII(AsciiStrings::parse(slice, allocator)?),
			
			TagType::SHORT => SHORT(u16::slice_unaligned_and_byte_swap_as_appropriate(count, byte_order, slice)),
			
			TagType::LONG => LONG(u32::slice_unaligned_and_byte_swap_as_appropriate(count, byte_order, slice)),
			
			TagType::RATIONAL => RATIONAL(RationalFraction::<u32>::slice_unaligned_and_byte_swap_as_appropriate(count, byte_order, slice)),
			
			TagType::SBYTE => SBYTE(i8::byte_slice(slice)),
			
			TagType::UNDEFINED => UNDEFINED(u8::byte_slice(slice)),
			
			TagType::SSHORT => SSHORT(i16::slice_unaligned_and_byte_swap_as_appropriate(count, byte_order, slice)?),
			
			TagType::SLONG => SLONG(i32::slice_unaligned_and_byte_swap_as_appropriate(count, byte_order, slice)),
			
			TagType::SRATIONAL => SRATIONAL(RationalFraction::<i32>::slice_unaligned_and_byte_swap_as_appropriate(count, byte_order, slice)),
			
			TagType::FLOAT => FLOAT(f32::slice_unaligned_and_byte_swap_as_appropriate(count, byte_order, slice)),
			
			TagType::DOUBLE => DOUBLE(f64::slice_unaligned_and_byte_swap_as_appropriate(count, byte_order, slice)),
			
			TagType::IFD => IFD(ImageFileDirectories::parse_child_image_file_directories::<Unit, TB, u32>(tiff_bytes, count, byte_order, slice, recursion_guard, allocator)?),
			
			TagType::LONG8 => LONG8(u64::slice_unaligned_and_byte_swap_as_appropriate(count, byte_order, slice)),
			
			TagType::SLONG8 => SLONG8(i64::slice_unaligned_and_byte_swap_as_appropriate(count, byte_order, slice)),
			
			TagType::IFD8 => IFD8(ImageFileDirectories::parse_child_image_file_directories::<Unit, TB, u64>(tiff_bytes, count, byte_order, slice, recursion_guard, allocator)?),
		};
		Ok(this)
	}
}
