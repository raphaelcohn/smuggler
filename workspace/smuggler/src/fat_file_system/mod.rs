// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.



use arrayvec::ArrayVec;
use likely::likely;
use std::alloc::Allocator;
use std::collections::TryReserveError;
use fatfs::{Date, DateTime, Dir, DirEntry};
use fatfs::ReadWriteSeek;
use fatfs::FatType;
use fatfs::FileAttributes;
use fatfs::FileSystem;
use fatfs::FsOptions;
use fatfs::Time;
use fatfs::TimeProvider;
use fscommon::BufStream;
use std::error;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::path::Path;
use swiss_army_knife::get_unchecked::GetUnchecked;
use rtiff::collections::VecExt;


include!("FatFileSystemImageError.rs");
include!("ReadOnlyTimeProvider.rs");

struct FatFileSystemAuthenticatedData<A: Allocator>
{
	fat_type: FatType,
	
	volume_identifier: u32,

	volume_label: Vec<u8, A>,
	
	root_directory_volume_label: Option<[u8; 11]>,
	
	cluster_size: u32,
}

trait DirectoryExt
{
}

struct FileInformation<A: Allocator>
{
	// These just don't live very long - no longer than &DirEntry.
	short_file_name: &'a [u8],
	
	long_file_name: Option<&'a [u16]>,

	length: u64,

	attributes: FileAttributes,

	created: DateTime,

	accessed: Date,

	modified: DateTime,
}

impl<T: ReadWriteSeek> DirectoryExt for Dir<T>
{
	fn list_files<A: Allocator>(&self) -> Vec<FileInformation<A>, A>
	{
		for directory_entry in self.iter()
		{
			directory_entry.long_file_name_as_ucs2_units();
			
			directory_entry.short_file_name_as_bytes();
			
			let length = directory_entry.len();
			let attributes = directory_entry.attributes();
			let created = directory_entry.created();
			let accessed = directory_entry.accessed();
			let modified = directory_entry.modified();
		}
	}
}

trait OpenReadOnlyFatFileSystem
{
	fn open_read_only(fat_file_system_image_file_path: &impl AsRef<Path>) -> Result<Self, FatFileSystemImageError>;
}

impl OpenReadOnlyFatFileSystem for FileSystem<BufStream<File>>
{
	fn open_read_only(fat_file_system_image_file_path: &impl AsRef<Path>) -> Result<Self, FatFileSystemImageError>
	{
		use FatFileSystemImageError::*;
		
		let file = OpenOptions::new().read(true).open(fat_file_system_image_file_path).map_err(CouldNotOpenImage)?;
		let stream = BufStream::new(file);
		
		static ReadOnlyTimeProviderSingleton: Self = Self;
		let file_system_options = FsOptions::new().update_accessed_data(false).time_provider(&mut ReadOnlyTimeProviderSingleton);
		FileSystem::new(stream, file_system_options).map_err(CouldNotParse)
	}
}

trait ReadOnlyFatFileSystem<T: ReadWriteSeek>: Sized
{
	fn information_for_authenticated_data<A: Allocator>(&self, allocator: A) -> Result<FatFileSystemAuthenticatedData<A>, TryReserveError>;
	
	fn find_directory<A: Allocator>(&self, directory_path_components: &[FileName<A>]) -> Option<Dir<BufStream<File>>>;
}

impl<T: ReadWriteSeek> ReadOnlyFatFileSytem for FileSystem<T>
{
	fn information_for_authenticated_data<A: Allocator>(&self, allocator: A) -> Result<FatFileSystemAuthenticatedData<A>, TryReserveError>
	{
		Ok
		(
			FatFileSystemAuthenticatedData
			{
				fat_type: self.fat_type(),
				
				volume_identifier: self.volume_id(),
				
				volume_label: Vec::new_from(self.volume_label_as_bytes(), allocator)?,
				
				root_directory_volume_label: self.read_volume_label_from_root_dir_as_bytes().map_err(XXXX)?,
			
				cluster_size: self.cluster_size(),
			}
		)
	}
	
	fn find_directory<A: Allocator>(&self, directory_path_components: &[FileName<A>]) -> Option<Dir<T>>
	{
		fn find_directory_entry<T: ReadWriteSeek>(directory: &Dir<T>, directory_path_component: &FileName<impl Allocator>) -> Option<Dir<T>>
		{
			for directory_entry in directory.iter()
			{
				if likely!(self.is_file())
				{
					continue
				}
				
				if directory_path_component.loosely_matches_file(directory_entry.long_file_name_as_ucs2_units(), directory_entry.short_file_name_as_bytes())
				{
					return Some(directory_entry.to_dir())
				}
			}
			None
		}
		
		let mut directory = self.root_dir();
		
		for directory_path_component in directory_path_components
		{
			let lower_directory = match find_directory_entry(&directory, directory_path_component)
			{
				None => return None,
				
				Some(directory) => directory,
			};
			directory = lower_directory;
		}
		
		Some(directory)
	}
}

type ShortFileNameCodePageEncoded = ArrayVec<u8, 11>;

/// Maximum size is 128.
type LongFileNameUcs2LittleEndianEncoded<A: Allocator> = Vec<u8, A>;

enum FileName<A: Allocator>
{
	LongOnly(LongFileNameUcs2LittleEndianEncoded<A>),
	
	ShortOnly(ShortFileNameCodePageEncoded),
	
	Both(LongFileNameUcs2LittleEndianEncoded<A>, ShortFileNameCodePageEncoded),
}

impl<A: Allocator> FileName<A>
{
	fn loosely_matches_file(&self, long: Option<&[u16]>, short: &[u8]) -> bool
	{
		use FileName::*;
		
		match self
		{
			LongOnly(our_long) => Some(our_long.as_slice()) == long,
			
			ShortOnly(our_short) => our_short.as_slice() == short,
			
			Both(our_long, short) => if Some(our_long.as_slice()) == long
			{
				true
			}
			else
			{
				our_short.as_slice() == short
			},
			
		}
	}
}
