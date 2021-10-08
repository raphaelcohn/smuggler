// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(in crate::image_file_directory) struct PublicTagParser<'tiff_bytes, A: Allocator + Clone>
{
	strip_offsets: Option<UnsignedIntegers<'tiff_bytes, u32>>,

	free_offsets: Option<UnsignedIntegers<'tiff_bytes, u32>>,

	tile_offsets: Option<UnsignedIntegers<'tiff_bytes, u32>>,
	
	//  The JPEGQTables, JPEGDCTables, and JPEGACTables tags can be present, but they can point to incorrect positions or even positions beyond EOF.
	// We've seen old-style JPEG in TIFF files where some or all Table offsets, contained the JPEGQTables, JPEGDCTables, and JPEGACTables tags are incorrect values beyond EOF. However, these files do always seem to contain a useful JPEGInterchangeFormat tag. Therefore, we recommend a careful attempt to read the Tables tags only as a last resort, if no table data is found in a JPEGInterchangeFormat stream.
	// Length was optional.
	jpeg_interchange_format: Option<UnsignedInteger<u32>>,

	// TODO: StripRowCounts
	// For strips with more than one layer there is a maximum strip size of 256 scanlines or full page size. The 256 maximum SHOULD be used unless the capability to receive longer strips has been negotiated. This field replaces RowsPerStrip for IFDs with variable-sized strips, and, as such, only one of the two fields, StripRowCounts and RowsPerStrip, may be used in an IFD.
	
	marker: PhantomData<A>,
}

impl<'tiff_bytes, A: Allocator + Clone> Default for PublicTagParser<'tiff_bytes, A>
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			strip_offsets: None,
		
			free_offsets: None,
		
			tile_offsets: None,
		
			jpeg_interchange_format: None,
		
			marker: PhantomData,
		}
	}
}

impl<'tiff_bytes, 'allocator, A: Allocator + Clone, TEH: TagEventHandler<PublicTag<'tiff_bytes, A>>> TagParser<'tiff_bytes, 'allocator, A, TEH, PublicTag<'tiff_bytes, A>> for PublicTagParser<'tiff_bytes, A>
{
	type FinishTagParseError = PublicTagFinishParseError;
	
	type TagParseError = PublicTagParseError;
	
	#[inline(always)]
	fn finish<TB: TiffBytes, Unit: Version6OrBigTiffUnit>(self, _common: &mut TagParserCommon<'tiff_bytes, 'allocator, TB, A, Unit>, _tag_event_handler: &mut TEH) -> Result<(), Self::FinishTagParseError>
	{
		use PublicTagFinishParseError::*;
		
		if unlikely!(self.strip_offsets.is_some())
		{
			Err(StripOffsetsWithoutStripByteCounts)?
		}
		if unlikely!(self.free_offsets.is_some())
		{
			Err(FreeOffsetsWithoutFreeByteCounts)?
		}
		if unlikely!(self.tile_offsets.is_some())
		{
			Err(TileOffsetsWithoutTileByteCounts)?
		}
		
		// TODO: Length was optional.
		if let Some(jpeg_interchange_format) = self.jpeg_interchange_format
		{
			todo!("JPEG interchange format");
		}
		
		Ok(())
	}
	
	#[inline(always)]
	fn parse<TB: TiffBytes, Unit: 'tiff_bytes + Version6OrBigTiffUnit>(&mut self, common: &mut TagParserCommon<'tiff_bytes, 'allocator, TB, A, Unit>, tag_event_handler: &mut TEH, tag_identifier: TagIdentifier, tag_type: TagType, raw_tag_value: RawTagValue<'tiff_bytes>) -> Result<(), Self::TagParseError>
	{
		// <https://www.awaresystems.be/imaging/tiff/tifftags.html>
		let tag = match tag_identifier
		{
			// Baseline tags.
			
			NewSubfileType => PublicTag::NewSubfileType(raw_tag_value.bitfield_integer(common, tag_type)?),
			
			SubfileType => PublicTag::SubfileType(raw_tag_value.enum_unsigned_integer(common, tag_type)?),
			
			ImageWidth => PublicTag::ImageWidth(raw_tag_value.unsigned_integer(common, tag_type)?),
			
			ImageLength => PublicTag::ImageLength(raw_tag_value.unsigned_integer(common, tag_type)?),
			
			BitsPerSample => PublicTag::BitsPerSample(raw_tag_value.unsigned_integers(common, tag_type)?),
			
			// Compression => PublicTag::Compression
			// (
			//
			// ),
			//
			// PhotometricInterpretation => PublicTag::PhotometricInterpretation
			// (
			//
			// ),
			//
			// Threshholding => PublicTag::Threshholding
			// (
			//
			// ),
			//
			// CellWidth => PublicTag::CellWidth
			// (
			//
			// ),
			//
			// CellLength => PublicTag::CellLength
			// (
			//
			// ),
			//
			// FillOrder => PublicTag::FillOrder
			// (
			//
			// ),
			//
			// ImageDescription => PublicTag::ImageDescription
			// (
			//
			// ),
			//
			// Make => PublicTag::Make
			// (
			//
			// ),
			//
			// Model => PublicTag::Model
			// (
			//
			// ),
			//
			StripOffsets =>
			{
				self.strip_offsets = Some(raw_tag_value.unsigned_integers(common, tag_type)?);
				return Ok(())
			}
			//
			// Orientation => PublicTag::Orientation
			// (
			//
			// ),
			//
			// SamplesPerPixel => PublicTag::SamplesPerPixel
			// (
			//
			// ),
			//
			// RowsPerStrip => PublicTag::RowsPerStrip
			// (
			//
			// ),
			//
			StripByteCounts => PublicTag::Strips(Self::parse_offsets_array(common, &mut self.strip_offsets, tag_type, raw_tag_value)?),
			//
			// MinSampleValue => PublicTag::MinSampleValue
			// (
			//
			// ),
			//
			// MaxSampleValue => PublicTag::MaxSampleValue
			// (
			//
			// ),
			//
			// XResolution => PublicTag::XResolution
			// (
			//
			// ),
			//
			// YResolution => PublicTag::YResolution
			// (
			//
			// ),
			//
			// PlanarConfiguration => PublicTag::PlanarConfiguration
			// (
			//
			// ),
			//
			// // TODO: Needs special handling.
			// FreeOffsets => PublicTag::FreeOffsets
			// (
			//
			// ),
			// FreeByteCounts => PublicTag::FreeByteCounts
			// (
			//
			// ),
			//
			// GrayResponseUnit => PublicTag::GrayResponseUnit
			// (
			//
			// ),
			//
			// GrayResponseCurve => PublicTag::GrayResponseCurve
			// (
			//
			// ),
			//
			// ResolutionUnit => PublicTag::ResolutionUnit
			// (
			//
			// ),
			//
			// Software => PublicTag::Software
			// (
			//
			// ),
			//
			// DateTime => PublicTag::DateTime
			// (
			//
			// ),
			//
			// Artist => PublicTag::Artist
			// (
			//
			// ),
			//
			// HostComputer => PublicTag::HostComputer
			// (
			//
			// ),
			//
			// ColorMap => PublicTag::ColorMap
			// (
			//
			// ),
			//
			// ExtraSamples => PublicTag::ExtraSamples
			// (
			//
			// ),
			//
			// Copyright => PublicTag::Copyright
			// (
			//
			// ),
			//
			// // Extension tags.
			// // TODO: Needs special handling.
			// TileOffsets,
			// TileByteCounts,
			//
			// // TODO: Can have 0 to n count.
			// SubIFDs,
			//
			// // TODO: A Specialized IFD; public tag defaults, but not clear if recursion is allowed within it.
			// // TODO: Can have count only of 1.
			// GlobalParametersIFD,
			//
			//
			// JPEGInterchangeFormat,
			// JPEGInterchangeFormatLength,
			//
			// // TODO: Also EXIF, GPS, Interoperability;
			// // TODO: ICC Profile has its own weird format.
			// // TODO: Maker note sub IFDs with the Microsoft hack.
			
			_ => PublicTag::Unrecognized(UnrecognizedTag::parse::<_, Unit>(common, tag_identifier, tag_type, raw_tag_value)?),
		};
		
		tag_event_handler.handle_tag_event(tag);
		Ok(())
	}
}

impl<'tiff_bytes, A: Allocator + Clone> PublicTagParser<'tiff_bytes, A>
{
	fn parse_offsets_array<'allocator, TB: TiffBytes, Unit: 'tiff_bytes + Version6OrBigTiffUnit>(common: &mut TagParserCommon<'tiff_bytes, 'allocator, TB, A, Unit>, offsets: &mut Option<UnsignedIntegers<'tiff_bytes, u32>>, tag_type: TagType, raw_tag_value: RawTagValue<'tiff_bytes>) -> Result<Vec<&'tiff_bytes [u8], A>, OffsetsArrayParseError>
	{
		use OffsetsArrayParseError::*;
		
		let offsets = match offsets.take()
		{
			None => return Err(ByteCountsWithoutOffsets),
	
			Some(offsets) => offsets.into(),
		};
		
		let number_of_offsets = Self::verify_array_lengths_match(&raw_tag_value, offsets)?;
		
		let byte_counts =
		{
			let byte_counts: UnsignedIntegers<u32> = raw_tag_value.unsigned_integers(common, tag_type).map_err(IntegerValuesParse)?;
			byte_counts.into()
		};
		
		let mut array = Vec::new_with_capacity(number_of_offsets, common.allocator()).map_err(CouldNotAllocateMemoryForOffsets)?;
		
		use UnsignedIntegerValues::*;
		match (offsets, byte_counts)
		{
			(U8(offsets), U8(byte_counts)) => Self::loop_over(array, common, offsets, byte_counts),
			
			(U8(offsets), U16(byte_counts)) => Self::loop_over(array, common, offsets, byte_counts),
			
			(U8(offsets), U32(byte_counts)) => Self::loop_over(array, common, offsets, byte_counts),
			
			(U8(offsets), U64(byte_counts)) => Self::loop_over(array, common, offsets, byte_counts),
			
			(U16(offsets), U8(byte_counts)) => Self::loop_over(array, common, offsets, byte_counts),
			
			(U16(offsets), U16(byte_counts)) => Self::loop_over(array, common, offsets, byte_counts),
			
			(U16(offsets), U32(byte_counts)) => Self::loop_over(array, common, offsets, byte_counts),
			
			(U16(offsets), U64(byte_counts)) => Self::loop_over(array, common, offsets, byte_counts),
			
			(U32(offsets), U8(byte_counts)) => Self::loop_over(array, common, offsets, byte_counts),
			
			(U32(offsets), U16(byte_counts)) => Self::loop_over(array, common, offsets, byte_counts),
			
			(U32(offsets), U32(byte_counts)) => Self::loop_over(array, common, offsets, byte_counts),
			
			(U32(offsets), U64(byte_counts)) => Self::loop_over(array, common, offsets, byte_counts),
			
			(U64(offsets), U8(byte_counts)) => Self::loop_over(array, common, offsets, byte_counts),
			
			(U64(offsets), U16(byte_counts)) => Self::loop_over(array, common, offsets, byte_counts),
			
			(U64(offsets), U32(byte_counts)) => Self::loop_over(array, common, offsets, byte_counts),
			
			(U64(offsets), U64(byte_counts)) => Self::loop_over(array, common, offsets, byte_counts),
		}
	}
	
	#[inline(always)]
	fn verify_array_lengths_match(raw_tag_value: &RawTagValue, offsets: UnsignedIntegerValues<'tiff_bytes>) -> Result<usize, OffsetsArrayParseError>
	{
		let byte_counts_length = raw_tag_value.count;
		let offsets_length = offsets.len() as u64;
		if unlikely!(offsets_length != byte_counts_length)
		{
			return Err(OffsetsArrayParseError::MismatchedArrayLengths { offsets_length, byte_counts_length })
		}
		Ok(byte_counts_length as usize)
	}
	
	#[inline(always)]
	fn loop_over<'allocator, TB: TiffBytes, Unit: 'tiff_bytes + Version6OrBigTiffUnit, Offset: ByteOrUnaligned, ByteCount: ByteOrUnaligned>(mut array: Vec<&'tiff_bytes [u8], A>, common: &mut TagParserCommon<'tiff_bytes, 'allocator, TB, A, Unit>, offsets: &[Offset], byte_counts: &[ByteCount]) -> Result<Vec<&'tiff_bytes [u8], A>, OffsetsArrayParseError>
	{
		for index in 0 .. array.capacity()
		{
			let offset = Offset::read_unsigned_integer_value(offsets, index);
			let byte_count = ByteCount::read_unsigned_integer_value(byte_counts, index);
			
			let slice =
			{
				let non_null = common.non_null_to_index_checked::<u8>(offset, byte_count).map_err(OffsetsArrayParseError::Overflow)?;
				unsafe { from_raw_parts(non_null.as_ptr() as *const u8, byte_count as usize) }
			};
			
			array.push_unchecked(slice)
		}
		
		Ok(array)
	}
		
}
