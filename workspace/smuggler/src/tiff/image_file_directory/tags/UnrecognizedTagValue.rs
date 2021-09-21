// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


/// An unrecognized tag value.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(u16)]
pub enum UnrecognizedTagValue<'a, TTT: TagTypeTrait>
{
	// Value 0 is not defined.
	
	/// Legacy from before TIFF version 6.
	BYTE(&'a [u8]) = 1,
	
	/// Legacy from before TIFF version 6.
	ASCII(&'a [AsciiCharacter]) = 2,
	
	/// Legacy from before TIFF version 6.
	SHORT(&'a [UnalignedU16]) = 3,
	
	/// Legacy from before TIFF version 6.
	LONG(&'a [UnalignedU32]) = 4,
	
	/// Legacy from before TIFF version 6.
	RATIONAL(&'a [UnalignedUnsignedRational]) = 5,
	
	/// Defined in TIFF version 6.
	SBYTE(&'a [i8]) = 6,
	
	/// Defined in TIFF version 6.
	UNDEFINED(&'a [u8]) = 7,
	
	/// Defined in TIFF version 6.
	SSHORT(&'a [UnalignedI16]) = 8,
	
	/// Defined in TIFF version 6.
	SLONG(&'a [UnalignedI32]) = 9,
	
	/// Defined in TIFF version 6.
	SRATIONAL(&'a [UnalignedSignedRational]) = 10,
	
	/// Defined in TIFF version 6.
	FLOAT(&'a [UnalignedF32]) = 11,
	
	/// Defined in TIFF version 6.
	DOUBLE(&'a [UnalignedF64]) = 12,
	
	/// Proposed by Adobe in 1995 in PageMaker technical notes and unofficially adopted.
	IFD(ImageFileDirectoryPointer) = 13,
	
	// Values 14 and 15 don't seem to be defined.
	
	/// Defined in TIFF version BigTiff.
	LONG8(&'a [UnalignedU64]) = 16,
	
	/// Defined in TIFF version BigTiff.
	SLONG8(&'a [UnalignedI64]) = 17,
	
	/// Defined in TIFF version BigTiff.
	IFD8(ImageFileDirectoryPointer) = 18,
}
