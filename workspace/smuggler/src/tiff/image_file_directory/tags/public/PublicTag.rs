// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


/// A public tag.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(u16)]
pub enum PublicTag<'a, A: Allocator>
{
	/// A general indication of the kind of data contained in this subfile.
	NewSubfileType(BitFieldInteger<NewSubfileTypeBitField>) = NewSubfileType,
	
	/// A general indication of the kind of data contained in this subfile.
	SubfileType(EnumUnsignedInteger<u16, SubfileTypeEnum>) = SubfileType,
	
	/// The number of columns in the image, ie the number of pixels per row.
	///
	/// US Library of Congress & Still Image Working Group Suggested Minimum.
	ImageWidth(UnsignedInteger<u16>) = ImageWidth,
	
	/// The number of rows of pixels in the image.
	///
	/// US Library of Congress & Still Image Working Group Suggested Minimum.
	ImageLength(UnsignedInteger<u16>) = ImageLength,
	
	/// Number of bits per component.
	///
	/// US Library of Congress & Still Image Working Group Suggested Minimum.
	BitsPerSample(&'a [Unaligned<u16>]) = BitsPerSample,
	
	/// Compression scheme used on the image data.
	///
	/// US Library of Congress & Still Image Working Group Suggested Minimum.
	Compression(UnsignedEnum) = Compression,
	
	/// The color space of the image data.
	///
	/// US Library of Congress & Still Image Working Group Suggested Minimum.
	PhotometricInterpretation(UnsignedEnum) = PhotometricInterpretation,
	
	/// For black and white TIFF files that represent shades of gray, the technique used to convert from gray to black and white pixels.
	Threshholding(UnsignedEnum) = Threshholding,
	
	/// The width of the dithering or half-toning matrix used to create a dithered or half-toned bilevel file.
	CellWidth(UnsignedInteger) = CellWidth,
	
	/// The length of the dithering or half-toning matrix used to create a dithered or half-toned bilevel file.
	CellLength(UnsignedInteger) = CellLength,
	
	/// The logical order of bits within a byte.
	FillOrder(UnsignedEnum) = FillOrder,
	
	/// A string that describes the subject of the image.
	///
	/// US Library of Congress & Still Image Working Group Suggested Minimum (this or the extended tag `DocumentName` or both).
	ImageDescription(&'a [AsciiCharacter]) = ImageDescription,
	
	/// The scanner manufacturer.
	///
	/// US Library of Congress & Still Image Working Group Additional Recommended.
	Make(u64) = Make,
	
	/// The scanner model name or number.
	///
	/// US Library of Congress & Still Image Working Group Additional Recommended.
	Model(u64) = Model,
	
	/// For each strip, the byte offset of that strip.
	StripOffsets(u64) = StripOffsets,
	
	/// The orientation of the image with respect to the rows and columns.
	Orientation(u64) = Orientation,
	
	/// The number of components per pixel.
	///
	/// US Library of Congress & Still Image Working Group Suggested Minimum.
	SamplesPerPixel(u64) = SamplesPerPixel,
	
	/// The number of rows per strip.
	RowsPerStrip(u64) = RowsPerStrip,
	
	/// For each strip, the number of bytes in the strip after compression.
	StripByteCounts(u64) = StripByteCounts,
	
	/// The minimum component value used.
	MinSampleValue(u64) = MinSampleValue,
	
	/// The maximum component value used.
	MaxSampleValue(u64) = MaxSampleValue,
	
	/// The number of pixels per ResolutionUnit in the ImageWidth direction.
	///
	/// US Library of Congress & Still Image Working Group Suggested Minimum.
	XResolution(UnsignedRational) = XResolution,
	
	/// The number of pixels per ResolutionUnit in the ImageLength direction.
	///
	/// US Library of Congress & Still Image Working Group Suggested Minimum.
	YResolution(UnsignedRational) = YResolution,
	
	/// How the components of each pixel are stored.
	PlanarConfiguration(u64) = PlanarConfiguration,
	
	/// For each string of contiguous unused bytes in a TIFF file, the byte offset of the string.
	FreeOffsets(u64) = FreeOffsets,
	
	/// For each string of contiguous unused bytes in a TIFF file, the number of bytes in the string.
	FreeByteCounts(u64) = FreeByteCounts,
	
	/// The precision of the information contained in the GrayResponseCurve.
	GrayResponseUnit(u64) = GrayResponseUnit,
	
	/// For grayscale data, the optical density of each possible pixel value.
	GrayResponseCurve(u64) = GrayResponseCurve,
	
	/// The unit of measurement for XResolution and YResolution.
	///
	/// US Library of Congress & Still Image Working Group Suggested Minimum.
	ResolutionUnit(u64) = ResolutionUnit,
	
	/// Name and version number of the software package(s) used to create the image.
	///
	/// US Library of Congress & Still Image Working Group Additional Recommended.
	Software(u64) = Software,
	
	/// Date and time of image creation.
	///
	/// US Library of Congress & Still Image Working Group Suggested Minimum.
	DateTime(u64) = DateTime,
	
	/// Person who created the image.
	///
	/// US Library of Congress & Still Image Working Group Suggested Minimum.
	Artist(u64) = Artist,
	
	/// The computer and/or operating system in use at the time of image creation.
	HostComputer(u64) = HostComputer,
	
	/// A color map for palette color images.
	ColorMap(u64) = ColorMap,
	
	/// Description of extra components.
	///
	/// US Library of Congress & Still Image Working Group Suggested Minimum (if applicable).
	ExtraSamples(u64) = ExtraSamples,
	
	/// Copyright notice.
	///
	/// US Library of Congress & Still Image Working Group Additional Recommended.
	Copyright(u64) = Copyright,
	
	/// The name of the document from which this image was scanned.
	///
	/// US Library of Congress & Still Image Working Group Suggested Minimum (this or the baseline tag `ImageDescription` or both).
	DocumentName(u64) = DocumentName,
	
	/// The name of the page from which this image was scanned.
	PageName(u64) = PageName,
	
	/// X position of the image.
	XPosition(u64) = XPosition,
	
	/// Y position of the image.
	YPosition(u64) = YPosition,
	
	/// Options for Group 3 Fax compression
	T4Options(u64) = T4Options,
	
	/// Options for Group 4 Fax compression
	T6Options(u64) = T6Options,
	
	/// The page number of the page from which this image was scanned.
	PageNumber(u64) = PageNumber,
	
	/// Describes a transfer function for the image in tabular style.
	TransferFunction(u64) = TransferFunction,
	
	/// A mathematical operator that is applied to the image data before an encoding scheme is applied.
	Predictor(u64) = Predictor,
	
	/// The chromaticity of the white point of the image.
	WhitePoint(u64) = WhitePoint,
	
	/// The chromaticities of the primaries of the image.
	PrimaryChromaticities(u64) = PrimaryChromaticities,
	
	/// Conveys to the halftone function the range of gray levels within a colorimetrically-specified image that should retain tonal detail.
	HalftoneHints(u64) = HalftoneHints,
	
	/// The tile width in pixels. This is the number of columns in each tile.
	TileWidth(u64) = TileWidth,
	
	/// The tile length (height) in pixels. This is the number of rows in each tile.
	TileLength(u64) = TileLength,
	
	/// For each tile, the byte offset of that tile, as compressed and stored on disk.
	TileOffsets(u64) = TileOffsets,
	
	/// For each tile, the number of (compressed) bytes in that tile.
	TileByteCounts(u64) = TileByteCounts,
	
	/// Used in the TIFF-F standard, denotes the number of 'bad' scan lines encountered by the facsimile device.
	BadFaxLines(u64) = BadFaxLines,
	
	/// Used in the TIFF-F standard, indicates if 'bad' lines encountered during reception are stored in the data, or if 'bad' lines have been replaced by the receiver.
	CleanFaxData(u64) = CleanFaxData,
	
	/// Used in the TIFF-F standard, denotes the maximum number of consecutive 'bad' scanlines received.
	ConsecutiveBadFaxLines(u64) = ConsecutiveBadFaxLines,
	
	/// Offset to child IFDs.
	SubIFDs(u64) = SubIFDs,
	
	/// The set of inks used in a separated (PhotometricInterpretation=5) image.
	InkSet(u64) = InkSet,
	
	/// The name of each ink used in a separated image.
	InkNames(u64) = InkNames,
	
	/// The number of inks.
	NumberOfInks(u64) = NumberOfInks,
	
	/// The component values that correspond to a 0% dot and 100% dot.
	DotRange(u64) = DotRange,
	
	/// A description of the printing environment for which this separation is intended.
	TargetPrinter(u64) = TargetPrinter,
	
	/// Specifies how to interpret each data sample in a pixel.
	SampleFormat(u64) = SampleFormat,
	
	/// Specifies the minimum sample value.
	SMinSampleValue(u64) = SMinSampleValue,
	
	/// Specifies the maximum sample value.
	SMaxSampleValue(u64) = SMaxSampleValue,
	
	/// Expands the range of the TransferFunction.
	TransferRange(u64) = TransferRange,
	
	/// Mirrors the essentials of PostScript's path creation functionality.
	ClipPath(u64) = ClipPath,
	
	/// The number of units that span the width of the image, in terms of integer ClipPath coordinates.
	XClipPathUnits(u64) = XClipPathUnits,
	
	/// The number of units that span the height of the image, in terms of integer ClipPath coordinates.
	YClipPathUnits(u64) = YClipPathUnits,
	
	/// Aims to broaden the support for indexed images to include support for any color space.
	Indexed(u64) = Indexed,
	
	/// JPEG quantization and/or Huffman tables.
	JPEGTables(u64) = JPEGTables,
	
	/// OPI-related.
	OPIProxy(u64) = OPIProxy,
	
	/// Used in the TIFF-FX standard to point to an IFD containing tags that are globally applicable to the complete TIFF file.
	GlobalParametersIFD(u64) = GlobalParametersIFD,
	
	/// Used in the TIFF-FX standard, denotes the type of data stored in this file or IFD.
	ProfileType(u64) = ProfileType,
	
	/// Used in the TIFF-FX standard, denotes the 'profile' that applies to this file.
	FaxProfile(u64) = FaxProfile,
	
	/// Used in the TIFF-FX standard, indicates which coding methods are used in the file.
	CodingMethods(u64) = CodingMethods,
	
	/// Used in the TIFF-FX standard, denotes the year of the standard specified by the FaxProfile field.
	VersionYear(u64) = VersionYear,
	
	/// Used in the TIFF-FX standard, denotes the mode of the standard specified by the FaxProfile field.
	ModeNumber(u64) = ModeNumber,
	
	/// Used in the TIFF-F and TIFF-FX standards, holds information about the ITULAB (PhotometricInterpretation = 10) encoding.
	Decode(u64) = Decode,
	
	/// Defined in the Mixed Raster Content part of RFC 2301, is the default color needed in areas where no image is available.
	DefaultImageColor(u64) = DefaultImageColor,
	
	/// Old-style JPEG compression field. TechNote2 invalidates this part of the specification.
	JPEGProc(u64) = JPEGProc,
	
	/// Old-style JPEG compression field. TechNote2 invalidates this part of the specification.
	JPEGInterchangeFormat(u64) = JPEGInterchangeFormat,
	
	/// Old-style JPEG compression field. TechNote2 invalidates this part of the specification.
	JPEGInterchangeFormatLength(u64) = JPEGInterchangeFormatLength,
	
	/// Old-style JPEG compression field. TechNote2 invalidates this part of the specification.
	JPEGRestartInterval(u64) = JPEGRestartInterval,
	
	/// Old-style JPEG compression field. TechNote2 invalidates this part of the specification.
	JPEGLosslessPredictors(u64) = JPEGLosslessPredictors,
	
	/// Old-style JPEG compression field. TechNote2 invalidates this part of the specification.
	JPEGPointTransforms(u64) = JPEGPointTransforms,
	
	/// Old-style JPEG compression field. TechNote2 invalidates this part of the specification.
	JPEGQTables(u64) = JPEGQTables,
	
	/// Old-style JPEG compression field. TechNote2 invalidates this part of the specification.
	JPEGDCTables(u64) = JPEGDCTables,
	
	/// Old-style JPEG compression field. TechNote2 invalidates this part of the specification.
	JPEGACTables(u64) = JPEGACTables,
	
	/// The transformation from RGB to YCbCr image data.
	YCbCrCoefficients(u64) = YCbCrCoefficients,
	
	/// Specifies the subsampling factors used for the chrominance components of a YCbCr image.
	YCbCrSubSampling(u64) = YCbCrSubSampling,
	
	/// Specifies the positioning of subsampled chrominance components relative to luminance samples.
	YCbCrPositioning(u64) = YCbCrPositioning,
	
	/// Specifies a pair of headroom and footroom image data values (codes) for each pixel component.
	ReferenceBlackWhite(u64) = ReferenceBlackWhite,
	
	/// Defined in the Mixed Raster Content part of RFC 2301, used to replace RowsPerStrip for IFDs with variable-sized strips.
	StripRowCounts(u64) = StripRowCounts,
	
	/// XML packet containing XMP metadata.
	///
	/// US Library of Congress & Still Image Working Group Additional Recommended.
	XMP(u64) = XMP,
	
	/// OPI-related.
	ImageID(u64) = ImageID,
	
	/// Defined in the Mixed Raster Content part of RFC 2301, used to denote the particular function of this Image in the mixed raster scheme.
	ImageLayer(u64) = ImageLayer,
	
	/// Annotation data, as used in 'Imaging for Windows'.
	WangAnnotation(u64) = WangAnnotation,
	
	/// Specifies the pixel data format encoding in the Molecular Dynamics GEL file format.
	MDFileTag(u64) = MDFileTag,
	
	/// Specifies a scale factor in the Molecular Dynamics GEL file format.
	MDScalePixel(u64) = MDScalePixel,
	
	/// Used to specify the conversion from 16bit to 8bit in the Molecular Dynamics GEL file format.
	MDColorTable(u64) = MDColorTable,
	
	/// Name of the lab that scanned this file, as used in the Molecular Dynamics GEL file format.
	MDLabName(u64) = MDLabName,
	
	/// Information about the sample, as used in the Molecular Dynamics GEL file format.
	MDSampleInfo(u64) = MDSampleInfo,
	
	/// Date the sample was prepared, as used in the Molecular Dynamics GEL file format.
	MDPrepDate(u64) = MDPrepDate,
	
	/// Time the sample was prepared, as used in the Molecular Dynamics GEL file format.
	MDPrepTime(u64) = MDPrepTime,
	
	/// Units for data in this file, as used in the Molecular Dynamics GEL file format.
	MDFileUnits(u64) = MDFileUnits,
	
	/// Used in interchangeable GeoTIFF files.
	ModelPixelScaleTag(u64) = ModelPixelScaleTag,
	
	/// IPTC (International Press Telecommunications Council) metadata.
	///
	/// US Library of Congress & Still Image Working Group Additional Recommended.
	IPTC(u64) = IPTC,
	
	/// Intergraph Application specific storage.
	INGRPacketDataTag(u64) = INGRPacketDataTag,
	
	/// Intergraph Application specific flags.
	INGRFlagRegisters(u64) = INGRFlagRegisters,
	
	/// Originally part of Intergraph's GeoTIFF tags, but likely understood by IrasB only.
	IrasBTransformationMatrix(u64) = IrasBTransformationMatrix,
	
	/// Originally part of Intergraph's GeoTIFF tags, but now used in interchangeable GeoTIFF files.
	ModelTiepointTag(u64) = ModelTiepointTag,
	
	/// Used in interchangeable GeoTIFF files.
	ModelTransformationTag(u64) = ModelTransformationTag,
	
	/// Collection of Photoshop 'Image Resource Blocks'.
	Photoshop(u64) = Photoshop,
	
	/// A pointer to the Exif IFD.
	///
	/// US Library of Congress & Still Image Working Group Additional Recommended.
	ExifIFD(u64) = ExifIFD,
	
	/// ICC profile data.
	///
	/// US Library of Congress & Still Image Working Group Additional Recommended.
	ICCProfile(u64) = ICCProfile,
	
	/// Used in interchangeable GeoTIFF files.
	GeoKeyDirectoryTag(u64) = GeoKeyDirectoryTag,
	
	/// Used in interchangeable GeoTIFF files.
	GeoDoubleParamsTag(u64) = GeoDoubleParamsTag,
	
	/// Used in interchangeable GeoTIFF files.
	GeoAsciiParamsTag(u64) = GeoAsciiParamsTag,
	
	/// A pointer to the Exif-related GPS Info IFD.
	GPSIFD(u64) = GPSIFD,
	
	/// Used by HylaFAX.
	HylaFAXFaxRecvParams(u64) = HylaFAXFaxRecvParams,
	
	/// Used by HylaFAX.
	HylaFAXFaxSubAddress(u64) = HylaFAXFaxSubAddress,
	
	/// Used by HylaFAX.
	HylaFAXFaxRecvTime(u64) = HylaFAXFaxRecvTime,
	
	/// Used by Adobe Photoshop.
	ImageSourceData(u64) = ImageSourceData,
	
	/// A pointer to the Exif-related Interoperability IFD.
	InteroperabilityIFD(u64) = InteroperabilityIFD,
	
	/// Used by the GDAL library, holds an XML list of name=value 'metadata' values about the image as a whole, and about specific samples.
	GDAL_METADATA(u64) = GDAL_METADATA,
	
	/// Used by the GDAL library, contains an ASCII encoded no data or background pixel value.
	GDAL_NODATA(u64) = GDAL_NODATA,
	
	/// Used in the Oce scanning process.
	OceScanJobDescription(u64) = OceScanJobDescription,
	
	/// Used in the Oce scanning process.
	OceApplicationSelector(u64) = OceApplicationSelector,
	
	/// Used in the Oce scanning process.
	OceIdentificationNumber(u64) = OceIdentificationNumber,
	
	/// Used in the Oce scanning process.
	OceImageLogicCharacteristics(u64) = OceImageLogicCharacteristics,
	
	/// Used in IFD 0 of DNG files.
	DNGVersion(u64) = DNGVersion,
	
	/// Used in IFD 0 of DNG files.
	DNGBackwardVersion(u64) = DNGBackwardVersion,
	
	/// Used in IFD 0 of DNG files.
	UniqueCameraModel(u64) = UniqueCameraModel,
	
	/// Used in IFD 0 of DNG files.
	LocalizedCameraModel(u64) = LocalizedCameraModel,
	
	/// Used in Raw IFD of DNG files.
	CFAPlaneColor(u64) = CFAPlaneColor,
	
	/// Used in Raw IFD of DNG files.
	CFALayout(u64) = CFALayout,
	
	/// Used in Raw IFD of DNG files.
	LinearizationTable(u64) = LinearizationTable,
	
	/// Used in Raw IFD of DNG files.
	BlackLevelRepeatDim(u64) = BlackLevelRepeatDim,
	
	/// Used in Raw IFD of DNG files.
	BlackLevel(u64) = BlackLevel,
	
	/// Used in Raw IFD of DNG files.
	BlackLevelDeltaH(u64) = BlackLevelDeltaH,
	
	/// Used in Raw IFD of DNG files.
	BlackLevelDeltaV(u64) = BlackLevelDeltaV,
	
	/// Used in Raw IFD of DNG files.
	WhiteLevel(u64) = WhiteLevel,
	
	/// Used in Raw IFD of DNG files.
	DefaultScale(u64) = DefaultScale,
	
	/// Used in Raw IFD of DNG files.
	DefaultCropOrigin(u64) = DefaultCropOrigin,
	
	/// Used in Raw IFD of DNG files.
	DefaultCropSize(u64) = DefaultCropSize,
	
	/// Used in IFD 0 of DNG files.
	ColorMatrix1(u64) = ColorMatrix1,
	
	/// Used in IFD 0 of DNG files.
	ColorMatrix2(u64) = ColorMatrix2,
	
	/// Used in IFD 0 of DNG files.
	CameraCalibration1(u64) = CameraCalibration1,
	
	/// Used in IFD 0 of DNG files.
	CameraCalibration2(u64) = CameraCalibration2,
	
	/// Used in IFD 0 of DNG files.
	ReductionMatrix1(u64) = ReductionMatrix1,
	
	/// Used in IFD 0 of DNG files.
	ReductionMatrix2(u64) = ReductionMatrix2,
	
	/// Used in IFD 0 of DNG files.
	AnalogBalance(u64) = AnalogBalance,
	
	/// Used in IFD 0 of DNG files.
	AsShotNeutral(u64) = AsShotNeutral,
	
	/// Used in IFD 0 of DNG files.
	AsShotWhiteXY(u64) = AsShotWhiteXY,
	
	/// Used in IFD 0 of DNG files.
	BaselineExposure(u64) = BaselineExposure,
	
	/// Used in IFD 0 of DNG files.
	BaselineNoise(u64) = BaselineNoise,
	
	/// Used in IFD 0 of DNG files.
	BaselineSharpness(u64) = BaselineSharpness,
	
	/// Used in Raw IFD of DNG files.
	BayerGreenSplit(u64) = BayerGreenSplit,
	
	/// Used in IFD 0 of DNG files.
	LinearResponseLimit(u64) = LinearResponseLimit,
	
	/// Used in IFD 0 of DNG files.
	CameraSerialNumber(u64) = CameraSerialNumber,
	
	/// Used in IFD 0 of DNG files.
	LensInfo(u64) = LensInfo,
	
	/// Used in Raw IFD of DNG files.
	ChromaBlurRadius(u64) = ChromaBlurRadius,
	
	/// Used in Raw IFD of DNG files.
	AntiAliasStrength(u64) = AntiAliasStrength,
	
	/// Used in IFD 0 of DNG files.
	ShadowScale(u64) = ShadowScale,
	
	/// Used in IFD 0 of DNG files.
	DNGPrivateData(u64) = DNGPrivateData,
	
	/// Used in IFD 0 of DNG files.
	MakerNoteSafety(u64) = MakerNoteSafety,
	
	/// Used in IFD 0 of DNG files.
	CalibrationIlluminant1(u64) = CalibrationIlluminant1,
	
	/// Used in IFD 0 of DNG files.
	CalibrationIlluminant2(u64) = CalibrationIlluminant2,
	
	/// Used in Raw IFD of DNG files.
	BestQualityScale(u64) = BestQualityScale,
	
	/// Used in IFD 0 of DNG files.
	RawDataUniqueID(u64) = RawDataUniqueID,
	
	/// Alias Sketchbook Pro layer usage description.
	AliasLayerMetadata(u64) = AliasLayerMetadata,
	
	/// Used in IFD 0 of DNG files.
	OriginalRawFileName(u64) = OriginalRawFileName,
	
	/// Used in IFD 0 of DNG files.
	OriginalRawFileData(u64) = OriginalRawFileData,
	
	/// Used in Raw IFD of DNG files.
	ActiveArea(u64) = ActiveArea,
	
	/// Used in Raw IFD of DNG files.
	MaskedAreas(u64) = MaskedAreas,
	
	/// Used in IFD 0 of DNG files.
	AsShotICCProfile(u64) = AsShotICCProfile,
	
	/// Used in IFD 0 of DNG files.
	AsShotPreProfileMatrix(u64) = AsShotPreProfileMatrix,
	
	/// Used in IFD 0 of DNG files.
	CurrentICCProfile(u64) = CurrentICCProfile,
	
	/// Used in IFD 0 of DNG files.
	CurrentPreProfileMatrix(u64) = CurrentPreProfileMatrix,
	
	/// Used in IFD 0 of DNG files.
	ColorimetricReference(u64) = ColorimetricReference,
	
	/// This private tag is used in a GEOTIFF standard by DGIWG.
	TIFF_RSID(u64) = TIFF_RSID,
	
	/// This private tag is used in a GEOTIFF standard by DGIWG.
	GEO_METADATA(u64) = GEO_METADATA,
	
	/// Used in IFD 0 of DNG files.
	CameraCalibrationSignature(u64) = CameraCalibrationSignature,
	
	/// Used in IFD 0 or Camera Profile IFD of DNG files.
	ProfileCalibrationSignature(u64) = ProfileCalibrationSignature,
	
	/// Used in IFD 0 of DNG files.
	ExtraCameraProfiles(u64) = ExtraCameraProfiles,
	
	/// Used in IFD 0 of DNG files.
	AsShotProfileName(u64) = AsShotProfileName,
	
	/// Used in Raw IFD or Enhanced IFD of DNG files.
	NoiseReductionApplied(u64) = NoiseReductionApplied,
	
	/// Used in IFD 0 or Camera Profile IFD of DNG files.
	ProfileName(u64) = ProfileName,
	
	/// Used in IFD 0 or Camera Profile IFD of DNG files.
	ProfileHueSatMapDims(u64) = ProfileHueSatMapDims,
	
	/// Used in IFD 0 or Camera Profile IFD of DNG files.
	ProfileHueSatMapData1(u64) = ProfileHueSatMapData1,
	
	/// Used in IFD 0 or Camera Profile IFD of DNG files.
	ProfileHueSatMapData2(u64) = ProfileHueSatMapData2,
	
	/// Used in IFD 0 or Camera Profile IFD of DNG files.
	ProfileToneCurve(u64) = ProfileToneCurve,
	
	/// Used in IFD 0 or Camera Profile IFD of DNG files.
	ProfileEmbedPolicy(u64) = ProfileEmbedPolicy,
	
	/// Used in IFD 0 or Camera Profile IFD of DNG files.
	ProfileCopyright(u64) = ProfileCopyright,
	
	/// Used in IFD 0 or Camera Profile IFD of DNG files.
	ForwardMatrix1(u64) = ForwardMatrix1,
	
	/// Used in IFD 0 or Camera Profile IFD of DNG files.
	ForwardMatrix2(u64) = ForwardMatrix2,
	
	/// Used in Preview IFD of DNG files.
	PreviewApplicationName(u64) = PreviewApplicationName,
	
	/// Used in Preview IFD of DNG files.
	PreviewApplicationVersion(u64) = PreviewApplicationVersion,
	
	/// Used in Preview IFD of DNG files.
	PreviewSettingsName(u64) = PreviewSettingsName,
	
	/// Used in Preview IFD of DNG files.
	PreviewSettingsDigest(u64) = PreviewSettingsDigest,
	
	/// Used in Preview IFD of DNG files.
	PreviewColorSpace(u64) = PreviewColorSpace,
	
	/// Used in Preview IFD of DNG files.
	PreviewDateTime(u64) = PreviewDateTime,
	
	/// Used in IFD 0 of DNG files.
	RawImageDigest(u64) = RawImageDigest,
	
	/// Used in IFD 0 of DNG files.
	OriginalRawFileDigest(u64) = OriginalRawFileDigest,
	
	/// Used in Raw IFD of DNG files.
	SubTileBlockSize(u64) = SubTileBlockSize,
	
	/// Used in Raw IFD of DNG files.
	RowInterleaveFactor(u64) = RowInterleaveFactor,
	
	/// Used in IFD 0 or Camera Profile IFD of DNG files.
	ProfileLookTableDims(u64) = ProfileLookTableDims,
	
	/// Used in IFD 0 or Camera Profile IFD of DNG files.
	ProfileLookTableData(u64) = ProfileLookTableData,
	
	/// Used in Raw IFD of DNG files.
	OpcodeList1(u64) = OpcodeList1,
	
	/// Used in Raw IFD of DNG files.
	OpcodeList2(u64) = OpcodeList2,
	
	/// Used in Raw IFD of DNG files.
	OpcodeList3(u64) = OpcodeList3,
	
	/// Used in Raw IFD or Enhanced IFD of DNG files.
	NoiseProfile(u64) = NoiseProfile,
	
	/// Used in IFD 0 of DNG files.
	OriginalDefaultFinalSize(u64) = OriginalDefaultFinalSize,
	
	/// Used in IFD 0 of DNG files.
	OriginalBestQualityFinalSize(u64) = OriginalBestQualityFinalSize,
	
	/// Used in IFD 0 of DNG files.
	OriginalDefaultCropSize(u64) = OriginalDefaultCropSize,
	
	/// Used in IFD 0 or Camera Profile IFD of DNG files.
	ProfileHueSatMapEncoding(u64) = ProfileHueSatMapEncoding,
	
	/// Used in IFD 0 or Camera Profile IFD of DNG files.
	ProfileLookTableEncoding(u64) = ProfileLookTableEncoding,
	
	/// Used in IFD 0 or Camera Profile IFD of DNG files.
	BaselineExposureOffset(u64) = BaselineExposureOffset,
	
	/// Used in IFD 0 or Camera Profile IFD of DNG files.
	DefaultBlackRender(u64) = DefaultBlackRender,
	
	/// Used in IFD 0 of DNG files.
	NewRawImageDigest(u64) = NewRawImageDigest,
	
	/// Used in Preview IFD of DNG files.
	RawToPreviewGain(u64) = RawToPreviewGain,
	
	/// Used in Raw IFD of DNG files.
	DefaultUserCrop(u64) = DefaultUserCrop,
	
	/// Used in IFD 0 of DNG files.
	DepthFormat(u64) = DepthFormat,
	
	/// Used in IFD 0 of DNG files.
	DepthNear(u64) = DepthNear,
	
	/// Used in IFD 0 of DNG files.
	DepthFar(u64) = DepthFar,
	
	/// Used in IFD 0 of DNG files.
	DepthUnits(u64) = DepthUnits,
	
	/// Used in IFD 0 of DNG files.
	DepthMeasureType(u64) = DepthMeasureType,
	
	/// Used in Enhanced IFD of DNG files.
	EnhanceParams(u64) = EnhanceParams,
	
	#[allow(missing_docs)]
	Unrecognized(TagIdentifier, UnrecognizedTagValue<'a>) = UnrecognizedRepresentationValue,
}

impl<'a, A: Allocator> EnumRepresentationU16 for PublicTag<'a, A>
{
}

impl<'a, A: Allocator> Tag for PublicTag<'a, A>
{
	type Key = PublicTagKey;
	
	#[inline(always)]
	fn key(&self) -> Self::Key
	{
		unsafe { transmute(self.raw_tag_key()) }
	}
}
