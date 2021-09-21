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
	fn parse(tiff_bytes: &impl Bytes) -> Result<(Self, ImageFileDirectoryPointer), HeaderParseError>
	{
		use HeaderParseError::*;
		
		let byte_order = Self::parse_byte_order(tiff_bytes).map_err(ByteOrderParse)?;
		let version = Self::parse_version(tiff_bytes, byte_order).map_err(|cause| VersionParse { cause, byte_order })?;
		
		let header = Self
		{
			byte_order,
			
			version,
		};
		
		let zeroth_image_file_directory_pointer = header.zeroth_image_file_directory_pointer(tiff_bytes)?;
		
		Ok
		(
			(
				header,
				zeroth_image_file_directory_pointer,
			)
		)
	}
	
	#[inline(always)]
	fn parse_byte_order(tiff_bytes: &impl Bytes) -> Result<ByteOrder, ByteOrderParseError>
	{
		use ByteOrderParseError::*;
		use ByteOrder::*;
		
		/// `II`.
		const IntelByteOrder: u16 = 0x4949;
		
		/// `MM`.
		const MotorolaByteOrder: u16 = 0x4D4D;
		
		match tiff_bytes.unaligned_u16_native_endian_byte_order_value(0).map_err(TooFewBytesForByteOrder)?
		{
			IntelByteOrder => Ok(LittleEndian),
			
			MotorolaByteOrder => Ok(BigEndian),
			
			native_endian_byte_order_bytes @ _ => Err(InvalidByteOrder { native_endian_byte_order_bytes })
		}
	}
	
	#[inline(always)]
	fn parse_version(tiff_bytes: &impl Bytes, byte_order: ByteOrder) -> Result<Version, VersionParseError>
	{
		use VersionParseError::*;
		use Version::*;
		
		match tiff_bytes.unaligned_u16_value(2, byte_order).map_err(TooFewBytesForVersion)?
		{
			42 => Ok(_6),
			
			43 => Ok(BigTiff),
			
			version @ _ => Err(UnknownVersion { version })
		}
	}
	
	#[inline(always)]
	fn zeroth_image_file_directory_pointer(self, tiff_bytes: &impl Bytes) -> Result<ImageFileDirectoryPointer, HeaderParseError>
	{
		use HeaderParseError::*;
		use Version::*;
		
		let index = match self.version
		{
			_6 => 4,
			
			BigTiff =>
			{
				self.parse_version_big_tiff_constants(tiff_bytes).map_err(|cause| BigTiffHeaderParse { cause, header: self })?;
				8
			}
		};
		
		self.parse_zeroth_image_file_directory_pointer(tiff_bytes, index).map_err(|cause| ZerothImageFileDirectoryPointerParse { cause, header: self })
	}
	
	#[inline(always)]
	fn parse_version_big_tiff_constants(&self, tiff_bytes: &impl Bytes) -> Result<(), BigTiffHeaderParseError>
	{
		let byte_order = self.byte_order;
		
		use BigTiffHeaderParseError::*;
		
		{
			let offset_size_in_bytes = tiff_bytes.unaligned_u16_value(4, byte_order).map_err(TooFewBytesForOffsetSize)?;
			if unlikely!(offset_size_in_bytes != 0x0008)
			{
				return Err(OffsetSizeWasNot8 { offset_size_in_bytes })
			}
		}
		
		{
			let constant = tiff_bytes.unaligned_u16_value(6, byte_order).map_err(TooFewBytesForConstant)?;
			if unlikely!(constant != 0x0000)
			{
				return Err(ConstantWasNot0 { constant })
			}
		}
		
		Ok(())
	}
	
	#[inline(always)]
	fn parse_zeroth_image_file_directory_pointer(self, tiff_bytes: &impl Bytes, index: u64) -> Result<x, ZerothImageFileDirectoryPointerParseError>
	{
		use ZerothImageFileDirectoryPointerParseError::*;
		
		let pointer = tiff_bytes.image_file_directory_pointer(index, self).map_err(PointerToZerothImageFileDirectory)?;
		pointer.ok_or(ThereMustBeAtLeastOneImageFileDirectory)
	}
}
