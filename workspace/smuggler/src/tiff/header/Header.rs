// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


/// Header.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Header
{
	#[allow(missing_docs)]
	pub byte_order: ByteOrder,
	
	#[allow(missing_docs)]
	pub version: Version,
}

impl Header
{
	#[inline(always)]
	fn parse<TB: TiffBytes>(tiff_bytes: &mut TB, free_space: &mut FreeSpace) -> Result<(Self, ImageFileDirectoryPointer), HeaderParseError>
	{
		use HeaderParseError::*;
		
		let byte_order = Self::parse_byte_order(tiff_bytes).map_err(ByteOrderParse)?;
		let tiff_bytes_with_order = TiffBytesWithOrder::new(tiff_bytes, byte_order);
		let version = Self::parse_version(&tiff_bytes_with_order).map_err(|cause| VersionParse { cause, byte_order })?;
		let zeroth_image_file_directory_pointer = Self::zeroth_image_file_directory_pointer(version, &tiff_bytes_with_order)?;
		
		free_space.record_header(version);
		
		Ok
		(
			(
				Self
				{
					byte_order,
					
					version,
				},
				
				zeroth_image_file_directory_pointer,
			)
		)
	}
	
	#[inline(always)]
	fn parse_byte_order(tiff_bytes: &impl TiffBytes) -> Result<ByteOrder, ByteOrderParseError>
	{
		use ByteOrderParseError::*;
		use ByteOrder::*;
		
		/// `II`.
		const IntelByteOrder: u16 = 0x4949;
		
		/// `MM`.
		const MotorolaByteOrder: u16 = 0x4D4D;
		
		match tiff_bytes.unaligned_u16_checked_native_endian_byte_order(0).map_err(TooFewBytesForByteOrder)?
		{
			IntelByteOrder => Ok(LittleEndian),
			
			MotorolaByteOrder => Ok(BigEndian),
			
			native_endian_byte_order_bytes @ _ => Err(InvalidByteOrder { native_endian_byte_order_bytes })
		}
	}
	
	#[inline(always)]
	fn parse_version<TB: TiffBytes>(tiff_bytes_with_order: &TiffBytesWithOrder<TB>) -> Result<Version, VersionParseError>
	{
		use VersionParseError::*;
		use Version::*;
		
		match tiff_bytes_with_order.unaligned_checked(2).map_err(TooFewBytesForVersion)?
		{
			42 => Ok(_6),
			
			43 => Ok(BigTiff),
			
			version @ _ => Err(UnknownVersion { version })
		}
	}
	
	#[inline(always)]
	fn zeroth_image_file_directory_pointer<TB: TiffBytes>(version: Version, tiff_bytes_with_order: &TiffBytesWithOrder<TB>) -> Result<ImageFileDirectoryPointer, HeaderParseError>
	{
		use HeaderParseError::*;
		use Version::*;
		
		let index = match version
		{
			_6 => 4,
			
			BigTiff =>
			{
				Self::parse_version_big_tiff_constants(tiff_bytes_with_order).map_err(|cause| BigTiffHeaderParse { cause, version })?;
				8
			}
		};
		
		Self::parse_zeroth_image_file_directory_pointer(version, tiff_bytes_with_order, index).map_err(|cause| ZerothImageFileDirectoryPointerParse { cause, version })
	}
	
	#[inline(always)]
	fn parse_version_big_tiff_constants<TB: TiffBytes>(tiff_bytes_with_order: &TiffBytesWithOrder<TB>) -> Result<(), BigTiffHeaderParseError>
	{
		use BigTiffHeaderParseError::*;
		
		{
			let offset_size_in_bytes = tiff_bytes_with_order.unaligned_checked(4).map_err(TooFewBytesForOffsetSize)?;
			if unlikely!(offset_size_in_bytes != 0x0008)
			{
				return Err(OffsetSizeWasNot8 { offset_size_in_bytes })
			}
		}
		
		{
			let constant = tiff_bytes_with_order.unaligned_checked(6).map_err(TooFewBytesForConstant)?;
			if unlikely!(constant != 0x0000)
			{
				return Err(ConstantWasNot0 { constant })
			}
		}
		
		Ok(())
	}
	
	#[inline(always)]
	fn parse_zeroth_image_file_directory_pointer<TB: TiffBytes>(version: Version, tiff_bytes_with_order: &TiffBytesWithOrder<TB>, index: Index) -> Result<ImageFileDirectoryPointer, ZerothImageFileDirectoryPointerParseError>
	{
		use ZerothImageFileDirectoryPointerParseError::*;
		
		use Version::*;
		
		let offset = match version
		{
			_6 => tiff_bytes_with_order.offset_version_6(index),
			
			BigTiff => tiff_bytes_with_order.offset_version_big_tiff(index),
		};
		
		let pointer = ImageFileDirectoryPointer::new_unchecked(offset).map_err(PointerToZerothImageFileDirectory)?;
		pointer.ok_or(ThereMustBeAtLeastOneImageFileDirectory)
	}
}
