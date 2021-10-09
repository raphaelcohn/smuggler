// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


#[doc(hidden)]
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct TiffBytesWithOrder<'tiff_bytes, TB: TiffBytes>
{
	pub(crate) tiff_bytes: &'tiff_bytes mut TB,
	
	pub(crate) byte_order: ByteOrder,
}

impl<'tiff_bytes, TB: TiffBytes> Deref for TiffBytesWithOrder<'tiff_bytes, TB>
{
	type Target = TB;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		self.tiff_bytes
	}
}

impl<'tiff_bytes, TB: TiffBytes> DerefMut for TiffBytesWithOrder<'tiff_bytes, TB>
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target
	{
		self.tiff_bytes
	}
}

impl<'tiff_bytes, TB: TiffBytes> TiffBytesWithOrder<'tiff_bytes, TB>
{
	#[inline(always)]
	pub(crate) fn new(tiff_bytes: &'tiff_bytes mut TB, byte_order: ByteOrder) -> Self
	{
		Self
		{
			tiff_bytes,
			
			byte_order,
		}
	}
	
	#[inline(always)]
	pub(crate) fn image_file_directory_pointer<Version: Version6OrBigTiffVersion>(&self, index: Index) -> Result<Option<ImageFileDirectoryPointer>, ImageFileDirectoryPointerParseError>
	{
		self.tiff_bytes.image_file_directory_pointer::<Version>(index, self.byte_order)
	}
	
	#[inline(always)]
	pub(crate) fn offset<Version: Version6OrBigTiffVersion>(&self, index: Index) -> Result<Offset, OffsetParseError>
	{
		self.tiff_bytes.offset::<Version>(index, self.byte_order)
	}
	
	#[inline(always)]
	pub(crate) fn unaligned_checked<CBU: CanBeUnaligned>(&self, index: Index) -> Result<CBU, OverflowError>
	{
		self.tiff_bytes.unaligned_checked(index, self.byte_order)
	}
	
	#[inline(always)]
	pub(crate) fn unaligned_unchecked<CBU: CanBeUnaligned>(&self, index: Index) -> CBU
	{
		self.tiff_bytes.unaligned_unchecked(index, self.byte_order)
	}
}
