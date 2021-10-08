// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


use super::super::super::TagIdentifier;


/// A general indication of the kind of data contained in this subfile.
pub(in crate::image_file_directory::tags) const NewSubfileType: TagIdentifier = 0x00FE;

/// A general indication of the kind of data contained in this subfile.
pub(in crate::image_file_directory::tags) const SubfileType: TagIdentifier = 0x00FF;

/// The number of columns in the image, ie the number of pixels per row.
///
/// US Library of Congress & Still Image Working Group Suggested Minimum.
pub(in crate::image_file_directory::tags) const ImageWidth: TagIdentifier = 0x0100;

/// The number of rows of pixels in the image.
///
/// US Library of Congress & Still Image Working Group Suggested Minimum.
pub(in crate::image_file_directory::tags) const ImageLength: TagIdentifier = 0x0101;

/// Number of bits per component.
///
/// US Library of Congress & Still Image Working Group Suggested Minimum.
pub(in crate::image_file_directory::tags) const BitsPerSample: TagIdentifier = 0x0102;

/// Compression scheme used on the image data.
///
/// US Library of Congress & Still Image Working Group Suggested Minimum.
pub(in crate::image_file_directory::tags) const Compression: TagIdentifier = 0x0103;

/// The color space of the image data.
///
/// US Library of Congress & Still Image Working Group Suggested Minimum.
pub(in crate::image_file_directory::tags) const PhotometricInterpretation: TagIdentifier = 0x0106;

/// For black and white TIFF files that represent shades of gray, the technique used to convert from gray to black and white pixels.
pub(in crate::image_file_directory::tags) const Threshholding: TagIdentifier = 0x0107 ;

/// The width of the dithering or half-toning matrix used to create a dithered or half-toned bilevel file.
pub(in crate::image_file_directory::tags) const CellWidth: TagIdentifier = 0x0108 ;

/// The length of the dithering or half-toning matrix used to create a dithered or half-toned bilevel file.
pub(in crate::image_file_directory::tags) const CellLength: TagIdentifier = 0x0109 ;

/// The logical order of bits within a byte.
pub(in crate::image_file_directory::tags) const FillOrder: TagIdentifier = 0x010A;

/// A string that describes the subject of the image.
///
/// US Library of Congress & Still Image Working Group Suggested Minimum (this or the extended tag `DocumentName` or both).
pub(in crate::image_file_directory::tags) const ImageDescription: TagIdentifier = 0x010E;

/// The scanner manufacturer.
///
/// US Library of Congress & Still Image Working Group Additional Recommended.
pub(in crate::image_file_directory::tags) const Make: TagIdentifier = 0x010F;

/// The scanner model name or number.
///
/// US Library of Congress & Still Image Working Group Additional Recommended.
pub(in crate::image_file_directory::tags) const Model: TagIdentifier = 0x0110;

/// For each strip, the byte offset of that strip.
pub(in crate::image_file_directory::tags) const StripOffsets: TagIdentifier = 0x0111;

/// The orientation of the image with respect to the rows and columns.
pub(in crate::image_file_directory::tags) const Orientation: TagIdentifier = 0x0112;

/// The number of components per pixel.
///
/// US Library of Congress & Still Image Working Group Suggested Minimum.
pub(in crate::image_file_directory::tags) const SamplesPerPixel: TagIdentifier = 0x0115;

/// The number of rows per strip.
pub(in crate::image_file_directory::tags) const RowsPerStrip: TagIdentifier = 0x0116;

/// For each strip, the number of bytes in the strip after compression.
pub(in crate::image_file_directory::tags) const StripByteCounts: TagIdentifier = 0x0117;

/// The minimum component value used.
pub(in crate::image_file_directory::tags) const MinSampleValue: TagIdentifier = 0x0118;

/// The maximum component value used.
pub(in crate::image_file_directory::tags) const MaxSampleValue: TagIdentifier = 0x0119;

/// The number of pixels per ResolutionUnit in the ImageWidth direction.
///
/// US Library of Congress & Still Image Working Group Suggested Minimum.
pub(in crate::image_file_directory::tags) const XResolution: TagIdentifier = 0x011A;

/// The number of pixels per ResolutionUnit in the ImageLength direction.
///
/// US Library of Congress & Still Image Working Group Suggested Minimum.
pub(in crate::image_file_directory::tags) const YResolution: TagIdentifier = 0x011B;

/// How the components of each pixel are stored.
pub(in crate::image_file_directory::tags) const PlanarConfiguration: TagIdentifier = 0x011C;

/// For each string of contiguous unused bytes in a TIFF file, the byte offset of the string.
pub(in crate::image_file_directory::tags) const FreeOffsets: TagIdentifier = 0x0120;

/// For each string of contiguous unused bytes in a TIFF file, the number of bytes in the string.
pub(in crate::image_file_directory::tags) const FreeByteCounts: TagIdentifier = 0x0121 ;

/// The precision of the information contained in the GrayResponseCurve.
pub(in crate::image_file_directory::tags) const GrayResponseUnit: TagIdentifier = 0x0122;

/// For grayscale data, the optical density of each possible pixel value.
pub(in crate::image_file_directory::tags) const GrayResponseCurve: TagIdentifier = 0x0123;

/// The unit of measurement for XResolution and YResolution.
///
/// US Library of Congress & Still Image Working Group Suggested Minimum.
pub(in crate::image_file_directory::tags) const ResolutionUnit: TagIdentifier = 0x0128;

/// Name and version number of the software package(s) used to create the image.
///
/// US Library of Congress & Still Image Working Group Additional Recommended.
pub(in crate::image_file_directory::tags) const Software: TagIdentifier = 0x0131;

/// Date and time of image creation.
///
/// US Library of Congress & Still Image Working Group Suggested Minimum.
pub(in crate::image_file_directory::tags) const DateTime: TagIdentifier = 0x0132;

/// Person who created the image.
///
/// US Library of Congress & Still Image Working Group Suggested Minimum.
pub(in crate::image_file_directory::tags) const Artist: TagIdentifier = 0x013B;

/// The computer and/or operating system in use at the time of image creation.
pub(in crate::image_file_directory::tags) const HostComputer: TagIdentifier = 0x013C;

/// A color map for palette color images.
pub(in crate::image_file_directory::tags) const ColorMap: TagIdentifier = 0x0140;

/// Description of extra components.
///
/// US Library of Congress & Still Image Working Group Suggested Minimum (if applicable).
pub(in crate::image_file_directory::tags) const ExtraSamples: TagIdentifier = 0x0152;

/// Copyright notice.
///
/// US Library of Congress & Still Image Working Group Additional Recommended.
pub(in crate::image_file_directory::tags) const Copyright: TagIdentifier = 0x8298;
