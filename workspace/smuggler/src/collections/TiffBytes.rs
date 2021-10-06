// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


pub(crate) trait TiffBytes
{
	fn file_length(&self) -> FileLength;
	
	#[inline(always)]
	fn image_file_directory_pointer_version_6(&self, index: Index, byte_order: ByteOrder) -> Result<Option<ImageFileDirectoryPointer>, ImageFileDirectoryPointerParseError>
	{
		ImageFileDirectoryPointer::new_unchecked(self.offset_version_6(index, byte_order))
	}
	
	#[inline(always)]
	fn image_file_directory_pointer_version_big_tiff(&self, index: Index, byte_order: ByteOrder) -> Result<Option<ImageFileDirectoryPointer>, ImageFileDirectoryPointerParseError>
	{
		ImageFileDirectoryPointer::new_unchecked(self.offset_version_big_tiff(index, byte_order))
	}
	
	#[inline(always)]
	fn offset_version_6(&self, index: Index, byte_order: ByteOrder) -> Result<Offset, OffsetParseError>
	{
		Offset::version_6(self, index, byte_order)
	}
	
	#[inline(always)]
	fn offset_version_big_tiff(&self, index: Index, byte_order: ByteOrder) -> Result<Offset, OffsetParseError>
	{
		Offset::version_big_tiff(self, index, byte_order)
	}
	
	#[inline(always)]
	fn slice_mut_checked<X>(&mut self, index: Index, length: u64) -> Result<&mut [X], OverflowError>
	{
		let size_in_bytes = length.checked_mul(size_of::<X>() as u64).ok_or(OverflowError::SizeOverflowsIndex { index, size_in_bytes: length })?;
		CheckedPointerToIndexLength::check_inner(index, size_in_bytes, self.file_length())?;
		
		Ok(unsafe { from_raw_parts_mut(self.pointer_to_index_length_mut::<X>(index), length as usize) })
	}
	
	#[inline(always)]
	fn unaligned_u16_checked_native_endian_byte_order(&self, index: Index) -> Result<u16, OverflowError>
	{
		let this = self.non_null_to_index_checked(index)?;
		Ok(this.read_unaligned())
	}
	
	#[inline(always)]
	fn byte_checked<B: Byte>(&self, index: Index) -> Result<B, OverflowError>
	{
		let this = self.non_null_to_index_checked(index)?;
		Ok(this.read())
	}
	
	#[inline(always)]
	fn byte_unchecked<B: Byte>(&self, index: Index) -> B
	{
		let this = self.non_null_to_index_unchecked(index);
		this.read()
	}
	
	#[inline(always)]
	fn unaligned_checked<CBU: CanBeUnaligned>(&self, index: Index, byte_order: ByteOrder) -> Result<CBU, OverflowError>
	{
		let this = self.non_null_to_index_checked(index)?;
		Ok(CBU::read_unaligned_and_byte_swap_as_appropriate(this, byte_order))
	}
	
	#[inline(always)]
	fn unaligned_unchecked<CBU: CanBeUnaligned>(&self, index: Index, byte_order: ByteOrder) -> CBU
	{
		let this = self.non_null_to_index_unchecked(index);
		CBU::read_unaligned_and_byte_swap_as_appropriate(this, byte_order)
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn non_null_to_index_checked<X: Sized>(&self, index: Index) -> Result<NonNull<X>, OverflowError>
	{
		CheckedPointerToIndexLength::check::<X, _>(self, index)?;
		Ok(new_non_null(Ok(self.pointer_to_index_length::<X>(index))?))
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn non_null_to_index_unchecked<X: Sized>(&self, index: Index) -> NonNull<X>
	{
		UncheckedPointerToIndexLength::check::<X, _>(self, index);
		new_non_null(self.pointer_to_index_length::<X>(index))
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn non_null_to_index_unchecked_mut<X: Sized>(&mut self, index: Index) -> NonNull<X>
	{
		self.non_null_to_index_unchecked(index)
	}
	
	#[doc(hidden)]
	fn pointer_to_index_length<X: Sized>(&self, index: Index) -> *const X;
	
	#[doc(hidden)]
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
