// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


/// Records free space in the TIFF file.
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct FreeSpace
{
}

impl FreeSpace
{
	#[inline(always)]
	fn new() -> Self
	{
		Self
		{
		}
	}
	
	#[inline(always)]
	fn record_used_space_slice(&mut self, index: Index, size_in_bytes: u64)
	{
		/*
			For long slices, it is possible the memory is used more than once, either for the same slice size or for one larger or smaller.
			Nothing in the TIFF spec prevents this, although it seems unlikely to occur in practice as a writer would have to identify 'identical' chunks of memory.
			Overlapping slices are thus possible.
			
			There is a BTreeMap backed collection, RangeMap, but I'm not really convinced.
			
			I know I looked at Hashbrown before for this - it might be worth looking at again.
				We can turn an index into a stable hash, for example, by rounding-down
					eg Index => Index / 64
				
				At each stable hash, we store a small structure, giving the first previous Index and the first next Index
				
				Alternatively, we store an enum
				
					AllUsed
					AllFree
					PartlyFree
					{
						Vec<Sorted Ranges of Free>,
					}
				
			
			Another technique
				Just add the ranges to a Vec; sort at the end; would need lots of memory
				Could sort-of merge as we go but would
		 */
	}
}


/*
	We can replace this with a Hashbrown hash map, and have absent chunks interpreted as 'Free' or 'Used'.
	
	In a typical TIFF, must chunks are going to end up full.
	
	The only common bits of free space are going to be 'crumbs' of 1, 2 and 3 bytes in the offset value field (or up to 7 bytes in BigTIFF).
	Even then, since LONG is much more common thant SHORT and BYTE is rarely used, the 'crumbs' will mostly not exist, and, if they do, they will be 2 bytes.
	
	What we're interested in is free space.
	
	When recording used space, multiple lengths of used space are going to coalesce, with a frequent pattern being
 */
struct FreeSpaceTable(Vec<FreeSpaceChunk>);

impl FreeSpaceTable
{
	fn record_used_space_slice(&mut self, index: Index, size_in_bytes: u64)
	{
		let inclusive_first_chunk_index = FreeSpaceChunk::chunk_index(index);
		let inclusive_last_chunk_index = FreeSpaceChunk::chunk_index(index + size_in_bytes);
		
		/*
			All chunks apart from first and last are 'full' chunks.
		
			Scenarios:-
				Single chunk that probably will be partly full
				Two chunks that probably will be partly full
				Three or more chunks
					only first and last are probably partly full.
					the others are full.
		
		
		 */
		
		
		for chunk_index in inclusive_first_chunk_index ..= inclusive_last_chunk_index
		{
		
		}
	}
}

struct FreeSpaceRange
{
	index: Index,

	size_in_bytes: u64,
}

use std::ops::Range;
enum FreeSpaceChunk
{
	Free,

	Used,

	PartlyFree
	{
		free_crumbs: Vec<FreeSpaceRange>,
	}
}

impl FreeSpaceChunk
{
	const NumberOfBytesRepresented: u64 = 4096;
	
	#[inline(always)]
	const fn chunk_index(index: Index) -> usize
	{
		(index / FreeSpaceChunk::NumberOfBytesRepresented) as usize
	}
}
