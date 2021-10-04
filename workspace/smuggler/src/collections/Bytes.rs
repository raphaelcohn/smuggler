// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


pub(crate) trait Bytes
{
	fn file_length(&self) -> FileLength;
	
	#[inline(always)]
	fn image_file_directory_pointer(&self, index: u64, header: Header) -> Result<Option<ImageFileDirectoryPointer>, ImageFileDirectoryPointerParseError>
	{
		ImageFileDirectoryPointer::new_unchecked(self.offset(index, header))
	}
	
	#[inline(always)]
	fn image_file_directory_pointer_version_6(&self, index: u64, byte_order: ByteOrder) -> Result<Option<ImageFileDirectoryPointer>, ImageFileDirectoryPointerParseError>
	{
		ImageFileDirectoryPointer::new_unchecked(self.offset_version_6(index, byte_order))
	}
	
	#[inline(always)]
	fn image_file_directory_pointer_version_big_tiff(&self, index: u64, byte_order: ByteOrder) -> Result<Option<ImageFileDirectoryPointer>, ImageFileDirectoryPointerParseError>
	{
		ImageFileDirectoryPointer::new_unchecked(self.offset_version_big_tiff(index, byte_order))
	}
	
	#[inline(always)]
	fn offset(&self, index: u64, header: Header) -> Result<Offset, OffsetParseError>
	{
		use Version::*;
		
		match header.version
		{
			_6 => self.offset_version_6(index, header.byte_order),
			
			BigTiff => self.offset_version_big_tiff(index, header.byte_order),
		}
	}
	
	#[inline(always)]
	fn offset_version_6(&self, index: u64, byte_order: ByteOrder) -> Result<Offset, OffsetParseError>
	{
		Offset::version_6(self, index, byte_order)
	}
	
	#[inline(always)]
	fn offset_version_big_tiff(&self, index: u64, byte_order: ByteOrder) -> Result<Offset, OffsetParseError>
	{
		Offset::version_big_tiff(self, index, byte_order)
	}
	
	#[inline(always)]
	fn u8_array<const size: usize>(&self, index: u64) -> Result<&[u8; size], OverflowError>
	{
		let pointer_to_index = self.pointer_to_index_checked::<[u8; size]>(index)?;
		Ok(unsafe { & * pointer_to_index })
	}
	
	#[inline(always)]
	fn slice<X>(&self, index: u64, length: u64) -> Result<&[X], OverflowError>
	{
		let size_in_bytes = length.checked_mul(size_of::<X>() as u64).ok_or(OverflowError::SizeOverflowsIndex { index, size: length })?;
		CheckedPointerToIndexLength::check_inner(index, size_in_bytes, self.file_length())?;
		
		Ok(unsafe { from_raw_parts(self.pointer_to_index_length::<X>(index), u64_index_to_usize_index(length)) })
	}
	
	#[inline(always)]
	fn slice_mut<X>(&mut self, index: u64, length: u64) -> Result<&mut [X], OverflowError>
	{
		let size_in_bytes = length.checked_mul(size_of::<X>() as u64).ok_or(OverflowError::SizeOverflowsIndex { index, size: length })?;
		CheckedPointerToIndexLength::check_inner(index, size_in_bytes, self.file_length())?;
		
		Ok(unsafe { from_raw_parts_mut(self.pointer_to_index_length_mut::<X>(index), u64_index_to_usize_index(length)) })
	}
	
	#[inline(always)]
	fn unaligned_u16_native_endian_byte_order(&self, index: u64) -> Result<u16, OverflowError>
	{
		let pointer_to_index = self.pointer_to_index_checked::<u16>(index)?;
		Ok(unsafe { pointer_to_index.read_unaligned() })
	}
	
	#[inline(always)]
	fn u8(&self, index: u64) -> u8
	{
		let pointer_to_index = self.pointer_to_index_checked::<u8>(index);
		unsafe { pointer_to_index.read() }
	}
	
	#[inline(always)]
	fn u8_unchecked(&self, index: u64) -> u8
	{
		let pointer_to_index = self.pointer_to_index_unchecked::<u8>(index);
		unsafe { pointer_to_index.read() }
	}
	
	#[inline(always)]
	fn unaligned_u16(&self, index: u64, byte_order: ByteOrder) -> Result<u16, OverflowError>
	{
		let pointer_to_index = self.pointer_to_index_checked::<u16>(index)?;
		Ok(byte_order.read_unaligned_u16(pointer_to_index))
	}
	
	#[inline(always)]
	fn unaligned_u16_unchecked(&self, index: u64, byte_order: ByteOrder) -> u16
	{
		let pointer_to_index = self.pointer_to_index_unchecked::<u16>(index);
		byte_order.read_unaligned_u16(pointer_to_index)
	}
	
	#[inline(always)]
	fn unaligned_u32(&self, index: u64, byte_order: ByteOrder) -> Result<u32, OverflowError>
	{
		let pointer_to_index = self.pointer_to_index_checked::<u32>(index)?;
		Ok(byte_order.read_unaligned_u32(pointer_to_index))
	}
	
	#[inline(always)]
	fn unaligned_u32_unchecked(&self, index: u64, byte_order: ByteOrder) -> u32
	{
		let pointer_to_index = self.pointer_to_index_unchecked::<u32>(index);
		byte_order.read_unaligned_u32(pointer_to_index)
	}
	
	#[inline(always)]
	fn unaligned_u64(&self, index: u64, byte_order: ByteOrder) -> Result<u64, OverflowError>
	{
		let pointer_to_index = self.pointer_to_index_checked::<u64>(index)?;
		Ok(byte_order.read_unaligned_u64(pointer_to_index))
	}
	
	#[inline(always)]
	fn unaligned_u64_unchecked(&self, index: u64, byte_order: ByteOrder) -> u64
	{
		let pointer_to_index = self.pointer_to_index_unchecked::<u64>(index);
		byte_order.read_unaligned_u64(pointer_to_index)
	}
	
	#[inline(always)]
	fn i8(&self, index: u64) -> i8
	{
		let pointer_to_index = self.pointer_to_index_checked::<i8>(index);
		unsafe { pointer_to_index.read() }
	}
	
	#[inline(always)]
	fn i8_unchecked(&self, index: u64) -> i8
	{
		let pointer_to_index = self.pointer_to_index_unchecked::<i8>(index);
		unsafe { pointer_to_index.read() }
	}
	
	#[inline(always)]
	fn unaligned_i16(&self, index: u64, byte_order: ByteOrder) -> Result<i16, OverflowError>
	{
		let pointer_to_index = self.pointer_to_index_checked::<i16>(index)?;
		Ok(byte_order.read_unaligned_i16(pointer_to_index))
	}
	
	#[inline(always)]
	fn unaligned_i16_unchecked(&self, index: u64, byte_order: ByteOrder) -> i16
	{
		let pointer_to_index = self.pointer_to_index_unchecked::<i16>(index);
		byte_order.read_unaligned_i16(pointer_to_index)
	}
	
	#[inline(always)]
	fn unaligned_i32(&self, index: u64, byte_order: ByteOrder) -> Result<i32, OverflowError>
	{
		let pointer_to_index = self.pointer_to_index_checked::<i32>(index)?;
		Ok(byte_order.read_unaligned_i32(pointer_to_index))
	}
	
	#[inline(always)]
	fn unaligned_i32_unchecked(&self, index: u64, byte_order: ByteOrder) -> i32
	{
		let pointer_to_index = self.pointer_to_index_unchecked::<i32>(index);
		byte_order.read_unaligned_i32(pointer_to_index)
	}
	
	#[inline(always)]
	fn unaligned_i64(&self, index: u64, byte_order: ByteOrder) -> Result<i64, OverflowError>
	{
		let pointer_to_index = self.pointer_to_index_checked::<i64>(index)?;
		Ok(byte_order.read_unaligned_i64(pointer_to_index))
	}
	
	#[inline(always)]
	fn unaligned_i64_unchecked(&self, index: u64, byte_order: ByteOrder) -> i64
	{
		let pointer_to_index = self.pointer_to_index_unchecked::<i64>(index);
		byte_order.read_unaligned_i64(pointer_to_index)
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn pointer_to_index_checked<X: Sized>(&self, index: u64) -> Result<*const X, OverflowError>
	{
		CheckedPointerToIndexLength::check::<X, _>(self, index)?;
		Ok(self.pointer_to_index_length::<X>(index))
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn pointer_to_index_unchecked<X: Sized>(&self, index: u64) -> *const X
	{
		UncheckedPointerToIndexLength::check::<X, _>(self, index);
		self.pointer_to_index_length::<X>(index)
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn pointer_to_index_unchecked_mut<X: Sized>(&mut self, index: u64) -> *mut X
	{
		UncheckedPointerToIndexLength::check::<X, _>(self, index);
		self.pointer_to_index_length_mut::<X>(index)
	}
	
	#[doc(hidden)]
	fn pointer_to_index_length<X: Sized>(&self, index: u64) -> *const X;
	
	#[doc(hidden)]
	fn pointer_to_index_length_mut<X: Sized>(&self, index: u64) -> *mut X;
}

impl Bytes for [u8]
{
	#[inline(always)]
	fn file_length(&self) -> FileLength
	{
		self.len() as u64
	}
	
	#[inline(always)]
	fn pointer_to_index_length<X: Sized>(&self, index: u64) -> *const X
	{
		let pointer = self.as_ptr();
		let usize_index = u64_index_to_usize_index(index);
		let pointer_to_index = unsafe { pointer.add(usize_index) };
		pointer_to_index as *const X
	}
	
	#[inline(always)]
	fn pointer_to_index_length_mut<X: Sized>(&mut self, index: u64) -> *mut X
	{
		let pointer = self.as_mut_ptr();
		let usize_index = u64_index_to_usize_index(index);
		let pointer_to_index = unsafe { pointer.add(usize_index) };
		pointer_to_index as *mut X
	}
}
