// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


use super::super::TagIdentifier;


/// The name of the document from which this image was scanned.
///
/// US Library of Congress & Still Image Working Group Suggested Minimum (this or the baseline tag `ImageDescription` or both).
pub(in crate::image_file_directory::tags) const DocumentName: TagIdentifier = 0x010D;

/// The name of the page from which this image was scanned.
pub(in crate::image_file_directory::tags) const PageName: TagIdentifier = 0x011D;

/// X position of the image.
pub(in crate::image_file_directory::tags) const XPosition: TagIdentifier = 0x011E;

/// Y position of the image.
pub(in crate::image_file_directory::tags) const YPosition: TagIdentifier = 0x011F;

/// Options for Group 3 Fax compression
pub(in crate::image_file_directory::tags) const T4Options: TagIdentifier = 0x0124;

/// Options for Group 4 Fax compression
pub(in crate::image_file_directory::tags) const T6Options: TagIdentifier = 0x0125;

/// The page number of the page from which this image was scanned.
pub(in crate::image_file_directory::tags) const PageNumber: TagIdentifier = 0x0129;

/// Describes a transfer function for the image in tabular style.
pub(in crate::image_file_directory::tags) const TransferFunction: TagIdentifier = 0x012D;

/// A mathematical operator that is applied to the image data before an encoding scheme is applied.
pub(in crate::image_file_directory::tags) const Predictor: TagIdentifier = 0x013D;

/// The chromaticity of the white point of the image.
pub(in crate::image_file_directory::tags) const WhitePoint: TagIdentifier = 0x013E;

/// The chromaticities of the primaries of the image.
pub(in crate::image_file_directory::tags) const PrimaryChromaticities: TagIdentifier = 0x013F;

/// Conveys to the halftone function the range of gray levels within a colorimetrically-specified image that should retain tonal detail.
pub(in crate::image_file_directory::tags) const HalftoneHints: TagIdentifier = 0x0141;

/// The tile width in pixels. This is the number of columns in each tile.
pub(in crate::image_file_directory::tags) const TileWidth: TagIdentifier = 0x0142;

/// The tile length (height) in pixels. This is the number of rows in each tile.
pub(in crate::image_file_directory::tags) const TileLength: TagIdentifier = 0x0143;

/// For each tile, the byte offset of that tile, as compressed and stored on disk.
pub(in crate::image_file_directory::tags) const TileOffsets: TagIdentifier = 0x0144;

/// For each tile, the number of (compressed) bytes in that tile.
pub(in crate::image_file_directory::tags) const TileByteCounts: TagIdentifier = 0x0145;

/// Used in the TIFF-F standard, denotes the number of 'bad' scan lines encountered by the facsimile device.
pub(in crate::image_file_directory::tags) const BadFaxLines: TagIdentifier = 0x0146;

/// Used in the TIFF-F standard, indicates if 'bad' lines encountered during reception are stored in the data, or if 'bad' lines have been replaced by the receiver.
pub(in crate::image_file_directory::tags) const CleanFaxData: TagIdentifier = 0x0147;

/// Used in the TIFF-F standard, denotes the maximum number of consecutive 'bad' scanlines received.
pub(in crate::image_file_directory::tags) const ConsecutiveBadFaxLines: TagIdentifier = 0x0148;

/// Offset to child IFDs.
pub(in crate::image_file_directory::tags) const SubIFDs: TagIdentifier = 0x014A;

/// The set of inks used in a separated (PhotometricInterpretation=5) image.
pub(in crate::image_file_directory::tags) const InkSet: TagIdentifier = 0x014C;

/// The name of each ink used in a separated image.
pub(in crate::image_file_directory::tags) const InkNames: TagIdentifier = 0x014D;

/// The number of inks.
pub(in crate::image_file_directory::tags) const NumberOfInks: TagIdentifier = 0x014E;

/// The component values that correspond to a 0% dot and 100% dot.
pub(in crate::image_file_directory::tags) const DotRange: TagIdentifier = 0x0150;

/// A description of the printing environment for which this separation is intended.
pub(in crate::image_file_directory::tags) const TargetPrinter: TagIdentifier = 0x0151;

/// Specifies how to interpret each data sample in a pixel.
pub(in crate::image_file_directory::tags) const SampleFormat: TagIdentifier = 0x0153;

/// Specifies the minimum sample value.
pub(in crate::image_file_directory::tags) const SMinSampleValue: TagIdentifier = 0x0154;

/// Specifies the maximum sample value.
pub(in crate::image_file_directory::tags) const SMaxSampleValue: TagIdentifier = 0x0155;

/// Expands the range of the TransferFunction.
pub(in crate::image_file_directory::tags) const TransferRange: TagIdentifier = 0x0156;

/// Mirrors the essentials of PostScript's path creation functionality.
pub(in crate::image_file_directory::tags) const ClipPath: TagIdentifier = 0x0157;

/// The number of units that span the width of the image, in terms of integer ClipPath coordinates.
pub(in crate::image_file_directory::tags) const XClipPathUnits: TagIdentifier = 0x0158;

/// The number of units that span the height of the image, in terms of integer ClipPath coordinates.
pub(in crate::image_file_directory::tags) const YClipPathUnits: TagIdentifier = 0x0159;

/// Aims to broaden the support for indexed images to include support for any color space.
pub(in crate::image_file_directory::tags) const Indexed: TagIdentifier = 0x015A;

/// JPEG quantization and/or Huffman tables.
pub(in crate::image_file_directory::tags) const JPEGTables: TagIdentifier = 0x015B;

/// OPI-related.
pub(in crate::image_file_directory::tags) const OPIProxy: TagIdentifier = 0x015F;

/// Used in the TIFF-FX standard to point to an IFD containing tags that are globally applicable to the complete TIFF file.
pub(in crate::image_file_directory::tags) const GlobalParametersIFD: TagIdentifier = 0x0190;

/// Used in the TIFF-FX standard, denotes the type of data stored in this file or IFD.
pub(in crate::image_file_directory::tags) const ProfileType: TagIdentifier = 0x0191;

/// Used in the TIFF-FX standard, denotes the 'profile' that applies to this file.
pub(in crate::image_file_directory::tags) const FaxProfile: TagIdentifier = 0x0192;

/// Used in the TIFF-FX standard, indicates which coding methods are used in the file.
pub(in crate::image_file_directory::tags) const CodingMethods: TagIdentifier = 0x0193;

/// Used in the TIFF-FX standard, denotes the year of the standard specified by the FaxProfile field.
pub(in crate::image_file_directory::tags) const VersionYear: TagIdentifier = 0x0194;

/// Used in the TIFF-FX standard, denotes the mode of the standard specified by the FaxProfile field.
pub(in crate::image_file_directory::tags) const ModeNumber: TagIdentifier = 0x0195;

/// Used in the TIFF-F and TIFF-FX standards, holds information about the ITULAB (PhotometricInterpretation = 10) encoding.
pub(in crate::image_file_directory::tags) const Decode: TagIdentifier = 0x01B1;

/// Defined in the Mixed Raster Content part of RFC 2301, is the default color needed in areas where no image is available.
pub(in crate::image_file_directory::tags) const DefaultImageColor: TagIdentifier = 0x01B2;

/// Old-style JPEG compression field. TechNote2 invalidates this part of the specification.
pub(in crate::image_file_directory::tags) const JPEGProc: TagIdentifier = 0x0200;

/// Old-style JPEG compression field. TechNote2 invalidates this part of the specification.
pub(in crate::image_file_directory::tags) const JPEGInterchangeFormat: TagIdentifier = 0x0201;

/// Old-style JPEG compression field. TechNote2 invalidates this part of the specification.
pub(in crate::image_file_directory::tags) const JPEGInterchangeFormatLength: TagIdentifier = 0x0202;

/// Old-style JPEG compression field. TechNote2 invalidates this part of the specification.
pub(in crate::image_file_directory::tags) const JPEGRestartInterval: TagIdentifier = 0x0203;

/// Old-style JPEG compression field. TechNote2 invalidates this part of the specification.
pub(in crate::image_file_directory::tags) const JPEGLosslessPredictors: TagIdentifier = 0x0205;

/// Old-style JPEG compression field. TechNote2 invalidates this part of the specification.
pub(in crate::image_file_directory::tags) const JPEGPointTransforms: TagIdentifier = 0x0206;

/// Old-style JPEG compression field. TechNote2 invalidates this part of the specification.
pub(in crate::image_file_directory::tags) const JPEGQTables: TagIdentifier = 0x0207;

/// Old-style JPEG compression field. TechNote2 invalidates this part of the specification.
pub(in crate::image_file_directory::tags) const JPEGDCTables: TagIdentifier = 0x0208;

/// Old-style JPEG compression field. TechNote2 invalidates this part of the specification.
pub(in crate::image_file_directory::tags) const JPEGACTables: TagIdentifier = 0x0209;

/// The transformation from RGB to YCbCr image data.
pub(in crate::image_file_directory::tags) const YCbCrCoefficients: TagIdentifier = 0x0211;

/// Specifies the subsampling factors used for the chrominance components of a YCbCr image.
pub(in crate::image_file_directory::tags) const YCbCrSubSampling: TagIdentifier = 0x0212;

/// Specifies the positioning of subsampled chrominance components relative to luminance samples.
pub(in crate::image_file_directory::tags) const YCbCrPositioning: TagIdentifier = 0x0213;

/// Specifies a pair of headroom and footroom image data values (codes) for each pixel component.
pub(in crate::image_file_directory::tags) const ReferenceBlackWhite: TagIdentifier = 0x0214;

/// Defined in the Mixed Raster Content part of RFC 2301, used to replace RowsPerStrip for IFDs with variable-sized strips.
pub(in crate::image_file_directory::tags) const StripRowCounts: TagIdentifier = 0x022F;

/// XML packet containing XMP metadata.
///
/// US Library of Congress & Still Image Working Group Additional Recommended.
pub(in crate::image_file_directory::tags) const XMP: TagIdentifier = 0x02BC;

/// OPI-related.
pub(in crate::image_file_directory::tags) const ImageID: TagIdentifier = 0x800D;

/// Defined in the Mixed Raster Content part of RFC 2301, used to denote the particular function of this Image in the mixed raster scheme.
pub(in crate::image_file_directory::tags) const ImageLayer: TagIdentifier = 0x87AC;
