// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


/// Represents TIFF bytes from a file or other source.
pub trait TiffBytes
{
	/// File length.
	fn file_length(&self) -> FileLength;
	
	#[doc(hidden)]
	#[inline(always)]
	fn image_file_directory_pointer<Version: Version6OrBigTiffVersion>(&self, index: Index, byte_order: ByteOrder) -> Result<Option<ImageFileDirectoryPointer>, ImageFileDirectoryPointerParseError>
	{
		ImageFileDirectoryPointer::new_unchecked(self.offset::<Version>(index, byte_order))
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn offset<Version: Version6OrBigTiffVersion>(&self, index: Index, byte_order: ByteOrder) -> Result<Offset, OffsetParseError>
	{
		let raw_offset = match self.unaligned_checked::<Version>(index, byte_order)
		{
			Ok(raw_offset) => raw_offset.into(),
			
			Err(cause) => return Err(OffsetParseError::Overflow(cause))
		};
		Offset::parse_offset_value(self, raw_offset)
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn unaligned_u16_checked_native_endian_byte_order(&self, index: Index) -> Result<u16, OverflowError>
	{
		let this = self.non_null_to_index_checked(index, size_of_u64::<u16>())?;
		Ok(this.read_unaligned())
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn byte_checked<B: Byte>(&self, index: Index) -> Result<B, OverflowError>
	{
		let this = self.non_null_to_index_checked(index, 1)?;
		Ok(this.read())
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn byte_unchecked<B: Byte>(&self, index: Index) -> B
	{
		let this = self.non_null_to_index_unchecked(index, size_of_u64::<B>());
		this.read()
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn unaligned_checked<CBU: CanBeUnaligned>(&self, index: Index, byte_order: ByteOrder) -> Result<CBU, OverflowError>
	{
		let this = self.non_null_to_index_checked(index, size_of_u64::<CBU>())?;
		Ok(CBU::read_unaligned_and_byte_swap_as_appropriate(this, byte_order))
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn unaligned_unchecked<CBU: CanBeUnaligned>(&self, index: Index, byte_order: ByteOrder) -> CBU
	{
		let this = self.non_null_to_index_unchecked(index, size_of_u64::<CBU>());
		CBU::read_unaligned_and_byte_swap_as_appropriate(this, byte_order)
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn non_null_to_index_checked<X: Sized>(&self, index: Index, count: u64) -> Result<NonNull<X>, OverflowError>
	{
		CheckedPointerToIndexLength::check::<X, _>(self, index, count)?;
		Ok(new_non_null(self.pointer_to_index_length::<X>(index) as *mut _))
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn non_null_to_index_unchecked<X: Sized>(&self, index: Index, count: u64) -> NonNull<X>
	{
		UncheckedPointerToIndexLength::check::<X, _>(self, index, count);
		new_non_null(self.pointer_to_index_length::<X>(index) as *mut _)
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn non_null_to_index_checked_mut<X: Sized>(&mut self, index: Index, count: u64) -> Result<NonNull<X>, OverflowError>
	{
		self.non_null_to_index_checked(index, count)
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn non_null_to_index_unchecked_mut<X: Sized>(&mut self, index: Index, count: u64) -> NonNull<X>
	{
		self.non_null_to_index_unchecked(index, count)
	}
	
	#[allow(missing_docs)]
	fn pointer_to_index_length<X: Sized>(&self, index: Index) -> *const X;
	
	#[allow(missing_docs)]
	fn pointer_to_index_length_mut<X: Sized>(&mut self, index: Index) -> *mut X;
}

impl TiffBytes for [u8]
{
	#[inline(always)]
	fn file_length(&self) -> FileLength
	{
		self.len() as u64
	}
	
	#[inline(always)]
	fn pointer_to_index_length<X: Sized>(&self, index: Index) -> *const X
	{
		let pointer = self.as_ptr();
		let pointer_to_index = unsafe { pointer.add(index as usize) };
		pointer_to_index as *const X
	}
	
	#[inline(always)]
	fn pointer_to_index_length_mut<X: Sized>(&mut self, index: Index) -> *mut X
	{
		let pointer = self.as_mut_ptr();
		let pointer_to_index = unsafe { pointer.add(index as usize) };
		pointer_to_index as *mut X
	}
}
