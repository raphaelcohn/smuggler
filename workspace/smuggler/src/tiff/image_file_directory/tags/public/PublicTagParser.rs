// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(in crate::tiff::image_file_directory) struct PublicTagParser<'tiff_bytes, A: Allocator + Copy>
{
	strip_offsets: Option<()>,

	free_offsets: Option<()>,

	tile_offsets: Option<()>,
	
	//  The JPEGQTables, JPEGDCTables, and JPEGACTables tags can be present, but they can point to incorrect positions or even positions beyond EOF.
	// We've seen old-style JPEG in TIFF files where some or all Table offsets, contained the JPEGQTables, JPEGDCTables, and JPEGACTables tags are incorrect values beyond EOF. However, these files do always seem to contain a useful JPEGInterchangeFormat tag. Therefore, we recommend a careful attempt to read the Tables tags only as a last resort, if no table data is found in a JPEGInterchangeFormat stream.
	// Length was optional
	jpeg_interchange_format: Option<()>,

	// StripRowCounts
	// For strips with more than one layer there is a maximum strip size of 256 scanlines or full page size. The 256 maximum SHOULD be used unless the capability to receive longer strips has been negotiated. This field replaces RowsPerStrip for IFDs with variable-sized strips, and, as such, only one of the two fields, StripRowCounts and RowsPerStrip, may be used in an IFD.
	
	marker: PhantomData<(&'tiff_bytes (), A)>,
}

impl<'tiff_bytes, A: Allocator + Copy> Default for PublicTagParser<'tiff_bytes, A>
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

impl<'tiff_bytes, 'recursion: 'recursion_guard, 'recursion_guard, A: Allocator + Copy, TEH: TagEventHandler<PublicTag<'tiff_bytes, A>, A>> TagParser<'tiff_bytes, 'recursion, 'recursion_guard, A, TEH> for PublicTagParser<'tiff_bytes, A>
{
	#[inline(always)]
	fn finish<TB: TiffBytes, Unit: Version6OrBigTiffUnit>(self, common: &TagParserCommon<'tiff_bytes, 'recursion, 'recursion_guard, TB, A>, tag_event_handler: &mut TEH)
	{
	}
	
	#[inline(always)]
	fn parse<TB: TiffBytes, Unit: Version6OrBigTiffUnit>(&mut self, common: &TagParserCommon<'tiff_bytes, 'recursion, 'recursion_guard, TB, A>, tag_event_handler: &mut TEH, tag_identifier: TagIdentifier, tag_type: TagType, raw_tag_value: RawTagValue<'tiff_bytes>) -> Result<(), SpecificTagParseError>
	{
		// <https://www.awaresystems.be/imaging/tiff/tifftags.html>
		let tag = match tag_identifier
		{
			// Baseline tags.
			
			NewSubfileType => PublicTag::NewSubfileType(raw_tag_value.bitfield_integer(common, tag_type)?),
			
			SubfileType => PublicTag::SubfileType(raw_tag_value.enum_unsigned_integer(common, tag_type)?),
			
			ImageWidth => PublicTag::ImageWidth(raw_tag_value.unsigned_integer(common, tag_type)?),
			
			ImageLength => PublicTag::ImageLength(raw_tag_value.unsigned_integer(common, tag_type)?),
			
			// TODO: should be an array of short. do we want to be lenient and specify an array of any unsigned type?
			BitsPerSample => PublicTag::BitsPerSample
			(
				match tag_type
				{
					TagType::SHORT => raw_tag_value.unaligned_slice(common),
					
					_ => return TagType::invalid()
				}
			),
			
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
			// // TODO: Needs special handling.
			// StripOffsets => PublicTag::StripOffsets
			// (
			//
			// ),
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
			// // TODO: Needs special handling - related to StripOffsets but does not occur immediately after StripOffsets!
			// StripByteCounts => PublicTag::StripByteCounts
			// (
			//
			// ),
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
