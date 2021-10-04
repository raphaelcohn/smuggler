// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


/// Public tag key (or name).
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u16)]
pub enum PublicTagKey
{
	/// A general indication of the kind of data contained in this subfile.
	NewSubfileType = NewSubfileType,
	
	/// A general indication of the kind of data contained in this subfile.
	SubfileType = SubfileType,
	
	/// The number of columns in the image, ie the number of pixels per row.
	///
	/// US Library of Congress & Still Image Working Group Suggested Minimum.
	ImageWidth = ImageWidth,
	
	/// The number of rows of pixels in the image.
	///
	/// US Library of Congress & Still Image Working Group Suggested Minimum.
	ImageLength = ImageLength,
	
	/// Number of bits per component.
	///
	/// US Library of Congress & Still Image Working Group Suggested Minimum.
	BitsPerSample = BitsPerSample,
	
	/// Compression scheme used on the image data.
	///
	/// US Library of Congress & Still Image Working Group Suggested Minimum.
	Compression = Compression,
	
	/// The color space of the image data.
	///
	/// US Library of Congress & Still Image Working Group Suggested Minimum.
	PhotometricInterpretation = PhotometricInterpretation,
	
	/// For black and white TIFF files that represent shades of gray, the technique used to convert from gray to black and white pixels.
	Threshholding = Threshholding,
	
	/// The width of the dithering or half-toning matrix used to create a dithered or half-toned bilevel file.
	CellWidth = CellWidth,
	
	/// The length of the dithering or half-toning matrix used to create a dithered or half-toned bilevel file.
	CellLength = CellLength,
	
	/// The logical order of bits within a byte.
	FillOrder = FillOrder,
	
	/// A string that describes the subject of the image.
	///
	/// US Library of Congress & Still Image Working Group Suggested Minimum (this or the extended tag `DocumentName` or both).
	ImageDescription = ImageDescription,
	
	/// The scanner manufacturer.
	///
	/// US Library of Congress & Still Image Working Group Additional Recommended.
	Make = Make,
	
	/// The scanner model name or number.
	///
	/// US Library of Congress & Still Image Working Group Additional Recommended.
	Model = Model,
	
	/// For each strip, the byte offset of that strip.
	StripOffsets = StripOffsets,
	
	/// The orientation of the image with respect to the rows and columns.
	Orientation = Orientation,
	
	/// The number of components per pixel.
	///
	/// US Library of Congress & Still Image Working Group Suggested Minimum.
	SamplesPerPixel = SamplesPerPixel,
	
	/// The number of rows per strip.
	RowsPerStrip = RowsPerStrip,
	
	/// For each strip, the number of bytes in the strip after compression.
	StripByteCounts = StripByteCounts,
	
	/// The minimum component value used.
	MinSampleValue = MinSampleValue,
	
	/// The maximum component value used.
	MaxSampleValue = MaxSampleValue,
	
	/// The number of pixels per ResolutionUnit in the ImageWidth direction.
	///
	/// US Library of Congress & Still Image Working Group Suggested Minimum.
	XResolution = XResolution,
	
	/// The number of pixels per ResolutionUnit in the ImageLength direction.
	///
	/// US Library of Congress & Still Image Working Group Suggested Minimum.
	YResolution = YResolution,
	
	/// How the components of each pixel are stored.
	PlanarConfiguration = PlanarConfiguration,
	
	/// For each string of contiguous unused bytes in a TIFF file, the byte offset of the string.
	FreeOffsets = FreeOffsets,
	
	/// For each string of contiguous unused bytes in a TIFF file, the number of bytes in the string.
	FreeByteCounts = FreeByteCounts,
	
	/// The precision of the information contained in the GrayResponseCurve.
	GrayResponseUnit = GrayResponseUnit,
	
	/// For grayscale data, the optical density of each possible pixel value.
	GrayResponseCurve = GrayResponseCurve,
	
	/// The unit of measurement for XResolution and YResolution.
	///
	/// US Library of Congress & Still Image Working Group Suggested Minimum.
	ResolutionUnit = ResolutionUnit,
	
	/// Name and version number of the software package(s) used to create the image.
	///
	/// US Library of Congress & Still Image Working Group Additional Recommended.
	Software = Software,
	
	/// Date and time of image creation.
	///
	/// US Library of Congress & Still Image Working Group Suggested Minimum.
	DateTime = DateTime,
	
	/// Person who created the image.
	///
	/// US Library of Congress & Still Image Working Group Suggested Minimum.
	Artist = Artist,
	
	/// The computer and/or operating system in use at the time of image creation.
	HostComputer = HostComputer,
	
	/// A color map for palette color images.
	ColorMap = ColorMap,
	
	/// Description of extra components.
	///
	/// US Library of Congress & Still Image Working Group Suggested Minimum (if applicable).
	ExtraSamples = ExtraSamples,
	
	/// Copyright notice.
	///
	/// US Library of Congress & Still Image Working Group Additional Recommended.
	Copyright = Copyright,

	/// The name of the document from which this image was scanned.
	///
	/// US Library of Congress & Still Image Working Group Suggested Minimum (this or the baseline tag `ImageDescription` or both).
	DocumentName = DocumentName,
	
	/// The name of the page from which this image was scanned.
	PageName = PageName,
	
	/// X position of the image.
	XPosition = XPosition,
	
	/// Y position of the image.
	YPosition = YPosition,
	
	/// Options for Group 3 Fax compression
	T4Options = T4Options,
	
	/// Options for Group 4 Fax compression
	T6Options = T6Options,
	
	/// The page number of the page from which this image was scanned.
	PageNumber = PageNumber,
	
	/// Describes a transfer function for the image in tabular style.
	TransferFunction = TransferFunction,
	
	/// A mathematical operator that is applied to the image data before an encoding scheme is applied.
	Predictor = Predictor,
	
	/// The chromaticity of the white point of the image.
	WhitePoint = WhitePoint,
	
	/// The chromaticities of the primaries of the image.
	PrimaryChromaticities = PrimaryChromaticities,
	
	/// Conveys to the halftone function the range of gray levels within a colorimetrically-specified image that should retain tonal detail.
	HalftoneHints = HalftoneHints,
	
	/// The tile width in pixels. This is the number of columns in each tile.
	TileWidth = TileWidth,
	
	/// The tile length (height) in pixels. This is the number of rows in each tile.
	TileLength = TileLength,
	
	/// For each tile, the byte offset of that tile, as compressed and stored on disk.
	TileOffsets = TileOffsets,
	
	/// For each tile, the number of (compressed) bytes in that tile.
	TileByteCounts = TileByteCounts,
	
	/// Used in the TIFF-F standard, denotes the number of 'bad' scan lines encountered by the facsimile device.
	BadFaxLines = BadFaxLines,
	
	/// Used in the TIFF-F standard, indicates if 'bad' lines encountered during reception are stored in the data, or if 'bad' lines have been replaced by the receiver.
	CleanFaxData = CleanFaxData,
	
	/// Used in the TIFF-F standard, denotes the maximum number of consecutive 'bad' scanlines received.
	ConsecutiveBadFaxLines = ConsecutiveBadFaxLines,
	
	/// Offset to child IFDs.
	SubIFDs = SubIFDs,
	
	/// The set of inks used in a separated (PhotometricInterpretation=5) image.
	InkSet = InkSet,
	
	/// The name of each ink used in a separated image.
	InkNames = InkNames,
	
	/// The number of inks.
	NumberOfInks = NumberOfInks,
	
	/// The component values that correspond to a 0% dot and 100% dot.
	DotRange = DotRange,
	
	/// A description of the printing environment for which this separation is intended.
	TargetPrinter = TargetPrinter,
	
	/// Specifies how to interpret each data sample in a pixel.
	SampleFormat = SampleFormat,
	
	/// Specifies the minimum sample value.
	SMinSampleValue = SMinSampleValue,
	
	/// Specifies the maximum sample value.
	SMaxSampleValue = SMaxSampleValue,
	
	/// Expands the range of the TransferFunction.
	TransferRange = TransferRange,
	
	/// Mirrors the essentials of PostScript's path creation functionality.
	ClipPath = ClipPath,
	
	/// The number of units that span the width of the image, in terms of integer ClipPath coordinates.
	XClipPathUnits = XClipPathUnits,
	
	/// The number of units that span the height of the image, in terms of integer ClipPath coordinates.
	YClipPathUnits = YClipPathUnits,
	
	/// Aims to broaden the support for indexed images to include support for any color space.
	Indexed = Indexed,
	
	/// JPEG quantization and/or Huffman tables.
	JPEGTables = JPEGTables,
	
	/// OPI-related.
	OPIProxy = OPIProxy,
	
	/// Used in the TIFF-FX standard to point to an IFD containing tags that are globally applicable to the complete TIFF file.
	GlobalParametersIFD = GlobalParametersIFD,
	
	/// Used in the TIFF-FX standard, denotes the type of data stored in this file or IFD.
	ProfileType = ProfileType,
	
	/// Used in the TIFF-FX standard, denotes the 'profile' that applies to this file.
	FaxProfile = FaxProfile,
	
	/// Used in the TIFF-FX standard, indicates which coding methods are used in the file.
	CodingMethods = CodingMethods,
	
	/// Used in the TIFF-FX standard, denotes the year of the standard specified by the FaxProfile field.
	VersionYear = VersionYear,
	
	/// Used in the TIFF-FX standard, denotes the mode of the standard specified by the FaxProfile field.
	ModeNumber = ModeNumber,
	
	/// Used in the TIFF-F and TIFF-FX standards, holds information about the ITULAB (PhotometricInterpretation = 10) encoding.
	Decode = Decode,
	
	/// Defined in the Mixed Raster Content part of RFC 2301, is the default color needed in areas where no image is available.
	DefaultImageColor = DefaultImageColor,
	
	/// Old-style JPEG compression field. TechNote2 invalidates this part of the specification.
	JPEGProc = JPEGProc,
	
	/// Old-style JPEG compression field. TechNote2 invalidates this part of the specification.
	JPEGInterchangeFormat = JPEGInterchangeFormat,
	
	/// Old-style JPEG compression field. TechNote2 invalidates this part of the specification.
	JPEGInterchangeFormatLength = JPEGInterchangeFormatLength,
	
	/// Old-style JPEG compression field. TechNote2 invalidates this part of the specification.
	JPEGRestartInterval = JPEGRestartInterval,
	
	/// Old-style JPEG compression field. TechNote2 invalidates this part of the specification.
	JPEGLosslessPredictors = JPEGLosslessPredictors,
	
	/// Old-style JPEG compression field. TechNote2 invalidates this part of the specification.
	JPEGPointTransforms = JPEGPointTransforms,
	
	/// Old-style JPEG compression field. TechNote2 invalidates this part of the specification.
	JPEGQTables = JPEGQTables,
	
	/// Old-style JPEG compression field. TechNote2 invalidates this part of the specification.
	JPEGDCTables = JPEGDCTables,
	
	/// Old-style JPEG compression field. TechNote2 invalidates this part of the specification.
	JPEGACTables = JPEGACTables,
	
	/// The transformation from RGB to YCbCr image data.
	YCbCrCoefficients = YCbCrCoefficients,
	
	/// Specifies the subsampling factors used for the chrominance components of a YCbCr image.
	YCbCrSubSampling = YCbCrSubSampling,
	
	/// Specifies the positioning of subsampled chrominance components relative to luminance samples.
	YCbCrPositioning = YCbCrPositioning,
	
	/// Specifies a pair of headroom and footroom image data values (codes) for each pixel component.
	ReferenceBlackWhite = ReferenceBlackWhite,
	
	/// Defined in the Mixed Raster Content part of RFC 2301, used to replace RowsPerStrip for IFDs with variable-sized strips.
	StripRowCounts = StripRowCounts,
	
	/// XML packet containing XMP metadata.
	///
	/// US Library of Congress & Still Image Working Group Additional Recommended.
	XMP = XMP,
	
	/// OPI-related.
	ImageID = ImageID,
	
	/// Defined in the Mixed Raster Content part of RFC 2301, used to denote the particular function of this Image in the mixed raster scheme.
	ImageLayer = ImageLayer,
	
	/// Annotation data, as used in 'Imaging for Windows'.
	WangAnnotation = WangAnnotation,
	
	/// Specifies the pixel data format encoding in the Molecular Dynamics GEL file format.
	MDFileTag = MDFileTag,
	
	/// Specifies a scale factor in the Molecular Dynamics GEL file format.
	MDScalePixel = MDScalePixel,
	
	/// Used to specify the conversion from 16bit to 8bit in the Molecular Dynamics GEL file format.
	MDColorTable = MDColorTable,
	
	/// Name of the lab that scanned this file, as used in the Molecular Dynamics GEL file format.
	MDLabName = MDLabName,
	
	/// Information about the sample, as used in the Molecular Dynamics GEL file format.
	MDSampleInfo = MDSampleInfo,
	
	/// Date the sample was prepared, as used in the Molecular Dynamics GEL file format.
	MDPrepDate = MDPrepDate,
	
	/// Time the sample was prepared, as used in the Molecular Dynamics GEL file format.
	MDPrepTime = MDPrepTime,
	
	/// Units for data in this file, as used in the Molecular Dynamics GEL file format.
	MDFileUnits = MDFileUnits,
	
	/// Used in interchangeable GeoTIFF files.
	ModelPixelScaleTag = ModelPixelScaleTag,
	
	/// IPTC (International Press Telecommunications Council) metadata.
	///
	/// US Library of Congress & Still Image Working Group Additional Recommended.
	IPTC = IPTC,
	
	/// Intergraph Application specific storage.
	INGRPacketDataTag = INGRPacketDataTag,
	
	/// Intergraph Application specific flags.
	INGRFlagRegisters = INGRFlagRegisters,
	
	/// Originally part of Intergraph's GeoTIFF tags, but likely understood by IrasB only.
	IrasBTransformationMatrix = IrasBTransformationMatrix,
	
	/// Originally part of Intergraph's GeoTIFF tags, but now used in interchangeable GeoTIFF files.
	ModelTiepointTag = ModelTiepointTag,
	
	/// Used in interchangeable GeoTIFF files.
	ModelTransformationTag = ModelTransformationTag,
	
	/// Collection of Photoshop 'Image Resource Blocks'.
	Photoshop = Photoshop,
	
	/// A pointer to the Exif IFD.
	///
	/// US Library of Congress & Still Image Working Group Additional Recommended.
	ExifIFD = ExifIFD,
	
	/// ICC profile data.
	///
	/// US Library of Congress & Still Image Working Group Additional Recommended.
	ICCProfile = ICCProfile,
	
	/// Used in interchangeable GeoTIFF files.
	GeoKeyDirectoryTag = GeoKeyDirectoryTag,
	
	/// Used in interchangeable GeoTIFF files.
	GeoDoubleParamsTag = GeoDoubleParamsTag,
	
	/// Used in interchangeable GeoTIFF files.
	GeoAsciiParamsTag = GeoAsciiParamsTag,
	
	/// A pointer to the Exif-related GPS Info IFD.
	GPSIFD = GPSIFD,
	
	/// Used by HylaFAX.
	HylaFAXFaxRecvParams = HylaFAXFaxRecvParams,
	
	/// Used by HylaFAX.
	HylaFAXFaxSubAddress = HylaFAXFaxSubAddress,
	
	/// Used by HylaFAX.
	HylaFAXFaxRecvTime = HylaFAXFaxRecvTime,
	
	/// Used by Adobe Photoshop.
	ImageSourceData = ImageSourceData,
	
	/// A pointer to the Exif-related Interoperability IFD.
	InteroperabilityIFD = InteroperabilityIFD,
	
	/// Used by the GDAL library, holds an XML list of name=value 'metadata' values about the image as a whole, and about specific samples.
	GDAL_METADATA = GDAL_METADATA,
	
	/// Used by the GDAL library, contains an ASCII encoded no data or background pixel value.
	GDAL_NODATA = GDAL_NODATA,
	
	/// Used in the Oce scanning process.
	OceScanJobDescription = OceScanJobDescription,
	
	/// Used in the Oce scanning process.
	OceApplicationSelector = OceApplicationSelector,
	
	/// Used in the Oce scanning process.
	OceIdentificationNumber = OceIdentificationNumber,
	
	/// Used in the Oce scanning process.
	OceImageLogicCharacteristics = OceImageLogicCharacteristics,
	
	/// Used in IFD 0 of DNG files.
	DNGVersion = DNGVersion,
	
	/// Used in IFD 0 of DNG files.
	DNGBackwardVersion = DNGBackwardVersion,
	
	/// Used in IFD 0 of DNG files.
	UniqueCameraModel = UniqueCameraModel,
	
	/// Used in IFD 0 of DNG files.
	LocalizedCameraModel = LocalizedCameraModel,
	
	/// Used in Raw IFD of DNG files.
	CFAPlaneColor = CFAPlaneColor,
	
	/// Used in Raw IFD of DNG files.
	CFALayout = CFALayout,
	
	/// Used in Raw IFD of DNG files.
	LinearizationTable = LinearizationTable,
	
	/// Used in Raw IFD of DNG files.
	BlackLevelRepeatDim = BlackLevelRepeatDim,
	
	/// Used in Raw IFD of DNG files.
	BlackLevel = BlackLevel,
	
	/// Used in Raw IFD of DNG files.
	BlackLevelDeltaH = BlackLevelDeltaH,
	
	/// Used in Raw IFD of DNG files.
	BlackLevelDeltaV = BlackLevelDeltaV,
	
	/// Used in Raw IFD of DNG files.
	WhiteLevel = WhiteLevel,
	
	/// Used in Raw IFD of DNG files.
	DefaultScale = DefaultScale,
	
	/// Used in Raw IFD of DNG files.
	DefaultCropOrigin = DefaultCropOrigin,
	
	/// Used in Raw IFD of DNG files.
	DefaultCropSize = DefaultCropSize,
	
	/// Used in IFD 0 of DNG files.
	ColorMatrix1 = ColorMatrix1,
	
	/// Used in IFD 0 of DNG files.
	ColorMatrix2 = ColorMatrix2,
	
	/// Used in IFD 0 of DNG files.
	CameraCalibration1 = CameraCalibration1,
	
	/// Used in IFD 0 of DNG files.
	CameraCalibration2 = CameraCalibration2,
	
	/// Used in IFD 0 of DNG files.
	ReductionMatrix1 = ReductionMatrix1,
	
	/// Used in IFD 0 of DNG files.
	ReductionMatrix2 = ReductionMatrix2,
	
	/// Used in IFD 0 of DNG files.
	AnalogBalance = AnalogBalance,
	
	/// Used in IFD 0 of DNG files.
	AsShotNeutral = AsShotNeutral,
	
	/// Used in IFD 0 of DNG files.
	AsShotWhiteXY = AsShotWhiteXY,
	
	/// Used in IFD 0 of DNG files.
	BaselineExposure = BaselineExposure,
	
	/// Used in IFD 0 of DNG files.
	BaselineNoise = BaselineNoise,
	
	/// Used in IFD 0 of DNG files.
	BaselineSharpness = BaselineSharpness,
	
	/// Used in Raw IFD of DNG files.
	BayerGreenSplit = BayerGreenSplit,
	
	/// Used in IFD 0 of DNG files.
	LinearResponseLimit = LinearResponseLimit,
	
	/// Used in IFD 0 of DNG files.
	CameraSerialNumber = CameraSerialNumber,
	
	/// Used in IFD 0 of DNG files.
	LensInfo = LensInfo,
	
	/// Used in Raw IFD of DNG files.
	ChromaBlurRadius = ChromaBlurRadius,
	
	/// Used in Raw IFD of DNG files.
	AntiAliasStrength = AntiAliasStrength,
	
	/// Used in IFD 0 of DNG files.
	ShadowScale = ShadowScale,
	
	/// Used in IFD 0 of DNG files.
	DNGPrivateData = DNGPrivateData,
	
	/// Used in IFD 0 of DNG files.
	MakerNoteSafety = MakerNoteSafety,
	
	/// Used in IFD 0 of DNG files.
	CalibrationIlluminant1 = CalibrationIlluminant1,
	
	/// Used in IFD 0 of DNG files.
	CalibrationIlluminant2 = CalibrationIlluminant2,
	
	/// Used in Raw IFD of DNG files.
	BestQualityScale = BestQualityScale,
	
	/// Used in IFD 0 of DNG files.
	RawDataUniqueID = RawDataUniqueID,
	
	/// Alias Sketchbook Pro layer usage description.
	AliasLayerMetadata = AliasLayerMetadata,
	
	/// Used in IFD 0 of DNG files.
	OriginalRawFileName = OriginalRawFileName,
	
	/// Used in IFD 0 of DNG files.
	OriginalRawFileData = OriginalRawFileData,
	
	/// Used in Raw IFD of DNG files.
	ActiveArea = ActiveArea,
	
	/// Used in Raw IFD of DNG files.
	MaskedAreas = MaskedAreas,
	
	/// Used in IFD 0 of DNG files.
	AsShotICCProfile = AsShotICCProfile,
	
	/// Used in IFD 0 of DNG files.
	AsShotPreProfileMatrix = AsShotPreProfileMatrix,
	
	/// Used in IFD 0 of DNG files.
	CurrentICCProfile = CurrentICCProfile,
	
	/// Used in IFD 0 of DNG files.
	CurrentPreProfileMatrix = CurrentPreProfileMatrix,
	
	/// Used in IFD 0 of DNG files.
	ColorimetricReference = ColorimetricReference,
	
	/// This private tag is used in a GEOTIFF standard by DGIWG.
	TIFF_RSID = TIFF_RSID,
	
	/// This private tag is used in a GEOTIFF standard by DGIWG.
	GEO_METADATA = GEO_METADATA,
	
	/// Used in IFD 0 of DNG files.
	CameraCalibrationSignature = CameraCalibrationSignature,
	
	/// Used in IFD 0 or Camera Profile IFD of DNG files.
	ProfileCalibrationSignature = ProfileCalibrationSignature,
	
	/// Used in IFD 0 of DNG files.
	ExtraCameraProfiles = ExtraCameraProfiles,
	
	/// Used in IFD 0 of DNG files.
	AsShotProfileName = AsShotProfileName,
	
	/// Used in Raw IFD or Enhanced IFD of DNG files.
	NoiseReductionApplied = NoiseReductionApplied,
	
	/// Used in IFD 0 or Camera Profile IFD of DNG files.
	ProfileName = ProfileName,
	
	/// Used in IFD 0 or Camera Profile IFD of DNG files.
	ProfileHueSatMapDims = ProfileHueSatMapDims,
	
	/// Used in IFD 0 or Camera Profile IFD of DNG files.
	ProfileHueSatMapData1 = ProfileHueSatMapData1,
	
	/// Used in IFD 0 or Camera Profile IFD of DNG files.
	ProfileHueSatMapData2 = ProfileHueSatMapData2,
	
	/// Used in IFD 0 or Camera Profile IFD of DNG files.
	ProfileToneCurve = ProfileToneCurve,
	
	/// Used in IFD 0 or Camera Profile IFD of DNG files.
	ProfileEmbedPolicy = ProfileEmbedPolicy,
	
	/// Used in IFD 0 or Camera Profile IFD of DNG files.
	ProfileCopyright = ProfileCopyright,
	
	/// Used in IFD 0 or Camera Profile IFD of DNG files.
	ForwardMatrix1 = ForwardMatrix1,
	
	/// Used in IFD 0 or Camera Profile IFD of DNG files.
	ForwardMatrix2 = ForwardMatrix2,
	
	/// Used in Preview IFD of DNG files.
	PreviewApplicationName = PreviewApplicationName,
	
	/// Used in Preview IFD of DNG files.
	PreviewApplicationVersion = PreviewApplicationVersion,
	
	/// Used in Preview IFD of DNG files.
	PreviewSettingsName = PreviewSettingsName,
	
	/// Used in Preview IFD of DNG files.
	PreviewSettingsDigest = PreviewSettingsDigest,
	
	/// Used in Preview IFD of DNG files.
	PreviewColorSpace = PreviewColorSpace,
	
	/// Used in Preview IFD of DNG files.
	PreviewDateTime = PreviewDateTime,
	
	/// Used in IFD 0 of DNG files.
	RawImageDigest = RawImageDigest,
	
	/// Used in IFD 0 of DNG files.
	OriginalRawFileDigest = OriginalRawFileDigest,
	
	/// Used in Raw IFD of DNG files.
	SubTileBlockSize = SubTileBlockSize,
	
	/// Used in Raw IFD of DNG files.
	RowInterleaveFactor = RowInterleaveFactor,
	
	/// Used in IFD 0 or Camera Profile IFD of DNG files.
	ProfileLookTableDims = ProfileLookTableDims,
	
	/// Used in IFD 0 or Camera Profile IFD of DNG files.
	ProfileLookTableData = ProfileLookTableData,
	
	/// Used in Raw IFD of DNG files.
	OpcodeList1 = OpcodeList1,
	
	/// Used in Raw IFD of DNG files.
	OpcodeList2 = OpcodeList2,
	
	/// Used in Raw IFD of DNG files.
	OpcodeList3 = OpcodeList3,
	
	/// Used in Raw IFD or Enhanced IFD of DNG files.
	NoiseProfile = NoiseProfile,
	
	/// Used in IFD 0 of DNG files.
	OriginalDefaultFinalSize = OriginalDefaultFinalSize,
	
	/// Used in IFD 0 of DNG files.
	OriginalBestQualityFinalSize = OriginalBestQualityFinalSize,
	
	/// Used in IFD 0 of DNG files.
	OriginalDefaultCropSize = OriginalDefaultCropSize,
	
	/// Used in IFD 0 or Camera Profile IFD of DNG files.
	ProfileHueSatMapEncoding = ProfileHueSatMapEncoding,
	
	/// Used in IFD 0 or Camera Profile IFD of DNG files.
	ProfileLookTableEncoding = ProfileLookTableEncoding,
	
	/// Used in IFD 0 or Camera Profile IFD of DNG files.
	BaselineExposureOffset = BaselineExposureOffset,
	
	/// Used in IFD 0 or Camera Profile IFD of DNG files.
	DefaultBlackRender = DefaultBlackRender,
	
	/// Used in IFD 0 of DNG files.
	NewRawImageDigest = NewRawImageDigest,
	
	/// Used in Preview IFD of DNG files.
	RawToPreviewGain = RawToPreviewGain,
	
	/// Used in Raw IFD of DNG files.
	DefaultUserCrop = DefaultUserCrop,
	
	/// Used in IFD 0 of DNG files.
	DepthFormat = DepthFormat,
	
	/// Used in IFD 0 of DNG files.
	DepthNear = DepthNear,
	
	/// Used in IFD 0 of DNG files.
	DepthFar = DepthFar,
	
	/// Used in IFD 0 of DNG files.
	DepthUnits = DepthUnits,
	
	/// Used in IFD 0 of DNG files.
	DepthMeasureType = DepthMeasureType,
	
	/// Used in Enhanced IFD of DNG files.
	EnhanceParams = EnhanceParams,

	/// Unrecognized.
	Unrecognized(TagIdentifier) = UnrecognizedRepresentationValue,
}

impl PartialOrd for PublicTagKey
{
	#[inline(always)]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering>
	{
		Some(self.cmp(other))
	}
}

impl Ord for PublicTagKey
{
	#[inline(always)]
	fn cmp(&self, other: &Self) -> Ordering
	{
		self.tag_identifier().cmp(&other.tag_identifier())
	}
}

impl TagKey for PublicTagKey
{
	#[inline(always)]
	fn tag_identifier(self) -> TagIdentifier
	{
		let raw_tag_key: RawTagKey = unsafe { transmute(self) };
		raw_tag_key.tag_identifier()
	}
}
