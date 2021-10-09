// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


use super::super::TagIdentifier;


/// Annotation data, as used in 'Imaging for Windows'.
pub(in crate::image_file_directory::tags) const WangAnnotation: TagIdentifier = 0x80A4;

/// Specifies the pixel data format encoding in the Molecular Dynamics GEL file format.
pub(in crate::image_file_directory::tags) const MDFileTag: TagIdentifier = 0x82A5;

/// Specifies a scale factor in the Molecular Dynamics GEL file format.
pub(in crate::image_file_directory::tags) const MDScalePixel: TagIdentifier = 0x82A6;

/// Used to specify the conversion from 16bit to 8bit in the Molecular Dynamics GEL file format.
pub(in crate::image_file_directory::tags) const MDColorTable: TagIdentifier = 0x82A7;

/// Name of the lab that scanned this file, as used in the Molecular Dynamics GEL file format.
pub(in crate::image_file_directory::tags) const MDLabName: TagIdentifier = 0x82A8;

/// Information about the sample, as used in the Molecular Dynamics GEL file format.
pub(in crate::image_file_directory::tags) const MDSampleInfo: TagIdentifier = 0x82A9;

/// Date the sample was prepared, as used in the Molecular Dynamics GEL file format.
pub(in crate::image_file_directory::tags) const MDPrepDate: TagIdentifier = 0x82AA;

/// Time the sample was prepared, as used in the Molecular Dynamics GEL file format.
pub(in crate::image_file_directory::tags) const MDPrepTime: TagIdentifier = 0x82AB;

/// Units for data in this file, as used in the Molecular Dynamics GEL file format.
pub(in crate::image_file_directory::tags) const MDFileUnits: TagIdentifier = 0x82AC;

/// Used in interchangeable GeoTIFF files.
pub(in crate::image_file_directory::tags) const ModelPixelScaleTag: TagIdentifier = 0x830E;

/// IPTC (International Press Telecommunications Council) metadata.
///
/// US Library of Congress & Still Image Working Group Additional Recommended.
pub(in crate::image_file_directory::tags) const IPTC: TagIdentifier = 0x83BB;

/// Intergraph Application specific storage.
pub(in crate::image_file_directory::tags) const INGRPacketDataTag: TagIdentifier = 0x847E;

/// Intergraph Application specific flags.
pub(in crate::image_file_directory::tags) const INGRFlagRegisters: TagIdentifier = 0x847F;

/// Originally part of Intergraph's GeoTIFF tags, but likely understood by IrasB only.
pub(in crate::image_file_directory::tags) const IrasBTransformationMatrix: TagIdentifier = 0x8480;

/// Originally part of Intergraph's GeoTIFF tags, but now used in interchangeable GeoTIFF files.
pub(in crate::image_file_directory::tags) const ModelTiepointTag: TagIdentifier = 0x8482;

/// Used in interchangeable GeoTIFF files.
pub(in crate::image_file_directory::tags) const ModelTransformationTag: TagIdentifier = 0x85D8;

/// Collection of Photoshop 'Image Resource Blocks'.
pub(in crate::image_file_directory::tags) const Photoshop: TagIdentifier = 0x8649;

/// A pointer to the Exif IFD.
///
/// US Library of Congress & Still Image Working Group Additional Recommended.
pub(in crate::image_file_directory::tags) const ExifIFD: TagIdentifier = 0x8769;

/// ICC profile data.
///
/// US Library of Congress & Still Image Working Group Additional Recommended.
pub(in crate::image_file_directory::tags) const ICCProfile: TagIdentifier = 0x8773;

/// Used in interchangeable GeoTIFF files.
pub(in crate::image_file_directory::tags) const GeoKeyDirectoryTag: TagIdentifier = 0x87AF;

/// Used in interchangeable GeoTIFF files.
pub(in crate::image_file_directory::tags) const GeoDoubleParamsTag: TagIdentifier = 0x87B0;

/// Used in interchangeable GeoTIFF files.
pub(in crate::image_file_directory::tags) const GeoAsciiParamsTag: TagIdentifier = 0x87B1;

/// A pointer to the Exif-related GPS Info IFD.
pub(in crate::image_file_directory::tags) const GPSIFD: TagIdentifier = 0x8825;

/// Used by HylaFAX.
pub(in crate::image_file_directory::tags) const HylaFAXFaxRecvParams: TagIdentifier = 0x885C;

/// Used by HylaFAX.
pub(in crate::image_file_directory::tags) const HylaFAXFaxSubAddress: TagIdentifier = 0x885D;

/// Used by HylaFAX.
pub(in crate::image_file_directory::tags) const HylaFAXFaxRecvTime: TagIdentifier = 0x885E;

/// Used by Adobe Photoshop.
pub(in crate::image_file_directory::tags) const ImageSourceData: TagIdentifier = 0x935C;

/// A pointer to the Exif-related Interoperability IFD.
pub(in crate::image_file_directory::tags) const InteroperabilityIFD: TagIdentifier = 0xA005;

/// Used by the GDAL library, holds an XML list of name=value 'metadata' values about the image as a whole, and about specific samples.
pub(in crate::image_file_directory::tags) const GDAL_METADATA: TagIdentifier = 0xA480;

/// Used by the GDAL library, contains an ASCII encoded no data or background pixel value.
pub(in crate::image_file_directory::tags) const GDAL_NODATA: TagIdentifier = 0xA481;

/// Used in the Oce scanning process.
pub(in crate::image_file_directory::tags) const OceScanJobDescription: TagIdentifier = 0xC427;

/// Used in the Oce scanning process.
pub(in crate::image_file_directory::tags) const OceApplicationSelector: TagIdentifier = 0xC428;

/// Used in the Oce scanning process.
pub(in crate::image_file_directory::tags) const OceIdentificationNumber: TagIdentifier = 0xC429;

/// Used in the Oce scanning process.
pub(in crate::image_file_directory::tags) const OceImageLogicCharacteristics: TagIdentifier = 0xC42A;

/// Used in IFD 0 of DNG files.
pub(in crate::image_file_directory::tags) const DNGVersion: TagIdentifier = 0xC612;

/// Used in IFD 0 of DNG files.
pub(in crate::image_file_directory::tags) const DNGBackwardVersion: TagIdentifier = 0xC613;

/// Used in IFD 0 of DNG files.
pub(in crate::image_file_directory::tags) const UniqueCameraModel: TagIdentifier = 0xC614;

/// Used in IFD 0 of DNG files.
pub(in crate::image_file_directory::tags) const LocalizedCameraModel: TagIdentifier = 0xC615;

/// Used in Raw IFD of DNG files.
pub(in crate::image_file_directory::tags) const CFAPlaneColor: TagIdentifier = 0xC616;

/// Used in Raw IFD of DNG files.
pub(in crate::image_file_directory::tags) const CFALayout: TagIdentifier = 0xC617;

/// Used in Raw IFD of DNG files.
pub(in crate::image_file_directory::tags) const LinearizationTable: TagIdentifier = 0xC618;

/// Used in Raw IFD of DNG files.
pub(in crate::image_file_directory::tags) const BlackLevelRepeatDim: TagIdentifier = 0xC619;

/// Used in Raw IFD of DNG files.
pub(in crate::image_file_directory::tags) const BlackLevel: TagIdentifier = 0xC61A;

/// Used in Raw IFD of DNG files.
pub(in crate::image_file_directory::tags) const BlackLevelDeltaH: TagIdentifier = 0xC61B;

/// Used in Raw IFD of DNG files.
pub(in crate::image_file_directory::tags) const BlackLevelDeltaV: TagIdentifier = 0xC61C;

/// Used in Raw IFD of DNG files.
pub(in crate::image_file_directory::tags) const WhiteLevel: TagIdentifier = 0xC61D;

/// Used in Raw IFD of DNG files.
pub(in crate::image_file_directory::tags) const DefaultScale: TagIdentifier = 0xC61E;

/// Used in Raw IFD of DNG files.
pub(in crate::image_file_directory::tags) const DefaultCropOrigin: TagIdentifier = 0xC61F;

/// Used in Raw IFD of DNG files.
pub(in crate::image_file_directory::tags) const DefaultCropSize: TagIdentifier = 0xC620;

/// Used in IFD 0 of DNG files.
pub(in crate::image_file_directory::tags) const ColorMatrix1: TagIdentifier = 0xC621;

/// Used in IFD 0 of DNG files.
pub(in crate::image_file_directory::tags) const ColorMatrix2: TagIdentifier = 0xC622;

/// Used in IFD 0 of DNG files.
pub(in crate::image_file_directory::tags) const CameraCalibration1: TagIdentifier = 0xC623;

/// Used in IFD 0 of DNG files.
pub(in crate::image_file_directory::tags) const CameraCalibration2: TagIdentifier = 0xC624;

/// Used in IFD 0 of DNG files.
pub(in crate::image_file_directory::tags) const ReductionMatrix1: TagIdentifier = 0xC625;

/// Used in IFD 0 of DNG files.
pub(in crate::image_file_directory::tags) const ReductionMatrix2: TagIdentifier = 0xC626;

/// Used in IFD 0 of DNG files.
pub(in crate::image_file_directory::tags) const AnalogBalance: TagIdentifier = 0xC627;

/// Used in IFD 0 of DNG files.
pub(in crate::image_file_directory::tags) const AsShotNeutral: TagIdentifier = 0xC628;

/// Used in IFD 0 of DNG files.
pub(in crate::image_file_directory::tags) const AsShotWhiteXY: TagIdentifier = 0xC629;

/// Used in IFD 0 of DNG files.
pub(in crate::image_file_directory::tags) const BaselineExposure: TagIdentifier = 0xC62A;

/// Used in IFD 0 of DNG files.
pub(in crate::image_file_directory::tags) const BaselineNoise: TagIdentifier = 0xC62B;

/// Used in IFD 0 of DNG files.
pub(in crate::image_file_directory::tags) const BaselineSharpness: TagIdentifier = 0xC62C;

/// Used in Raw IFD of DNG files.
pub(in crate::image_file_directory::tags) const BayerGreenSplit: TagIdentifier = 0xC62D;

/// Used in IFD 0 of DNG files.
pub(in crate::image_file_directory::tags) const LinearResponseLimit: TagIdentifier = 0xC62E;

/// Used in IFD 0 of DNG files.
pub(in crate::image_file_directory::tags) const CameraSerialNumber: TagIdentifier = 0xC62F;

/// Used in IFD 0 of DNG files.
pub(in crate::image_file_directory::tags) const LensInfo: TagIdentifier = 0xC630;

/// Used in Raw IFD of DNG files.
pub(in crate::image_file_directory::tags) const ChromaBlurRadius: TagIdentifier = 0xC631;

/// Used in Raw IFD of DNG files.
pub(in crate::image_file_directory::tags) const AntiAliasStrength: TagIdentifier = 0xC632;

/// Used in IFD 0 of DNG files.
pub(in crate::image_file_directory::tags) const ShadowScale: TagIdentifier = 0xC633;

/// Used in IFD 0 of DNG files.
pub(in crate::image_file_directory::tags) const DNGPrivateData: TagIdentifier = 0xC634;

/// Used in IFD 0 of DNG files.
pub(in crate::image_file_directory::tags) const MakerNoteSafety: TagIdentifier = 0xC635;

/// Used in IFD 0 of DNG files.
pub(in crate::image_file_directory::tags) const CalibrationIlluminant1: TagIdentifier = 0xC65A;

/// Used in IFD 0 of DNG files.
pub(in crate::image_file_directory::tags) const CalibrationIlluminant2: TagIdentifier = 0xC65B;

/// Used in Raw IFD of DNG files.
pub(in crate::image_file_directory::tags) const BestQualityScale: TagIdentifier = 0xC65C;

/// Used in IFD 0 of DNG files.
pub(in crate::image_file_directory::tags) const RawDataUniqueID: TagIdentifier = 0xC65D;

/// Alias Sketchbook Pro layer usage description.
pub(in crate::image_file_directory::tags) const AliasLayerMetadata: TagIdentifier = 0xC660;

/// Used in IFD 0 of DNG files.
pub(in crate::image_file_directory::tags) const OriginalRawFileName: TagIdentifier = 0xC68B;

/// Used in IFD 0 of DNG files.
pub(in crate::image_file_directory::tags) const OriginalRawFileData: TagIdentifier = 0xC68C;

/// Used in Raw IFD of DNG files.
pub(in crate::image_file_directory::tags) const ActiveArea: TagIdentifier = 0xC68D;

/// Used in Raw IFD of DNG files.
pub(in crate::image_file_directory::tags) const MaskedAreas: TagIdentifier = 0xC68E;

/// Used in IFD 0 of DNG files.
pub(in crate::image_file_directory::tags) const AsShotICCProfile: TagIdentifier = 0xC68F;

/// Used in IFD 0 of DNG files.
pub(in crate::image_file_directory::tags) const AsShotPreProfileMatrix: TagIdentifier = 0xC690;

/// Used in IFD 0 of DNG files.
pub(in crate::image_file_directory::tags) const CurrentICCProfile: TagIdentifier = 0xC691;

/// Used in IFD 0 of DNG files.
pub(in crate::image_file_directory::tags) const CurrentPreProfileMatrix: TagIdentifier = 0xC692;

/// Used in IFD 0 of DNG files.
pub(in crate::image_file_directory::tags) const ColorimetricReference: TagIdentifier = 0xC6BF;

/// This private tag is used in a GEOTIFF standard by DGIWG.
pub(in crate::image_file_directory::tags) const TIFF_RSID: TagIdentifier = 0xC6DC;

/// This private tag is used in a GEOTIFF standard by DGIWG.
pub(in crate::image_file_directory::tags) const GEO_METADATA: TagIdentifier = 0xC6DD;

/// Used in IFD 0 of DNG files.
pub(in crate::image_file_directory::tags) const CameraCalibrationSignature: TagIdentifier = 0xC6F3;

/// Used in IFD 0 or Camera Profile IFD of DNG files.
pub(in crate::image_file_directory::tags) const ProfileCalibrationSignature: TagIdentifier = 0xC6F4;

/// Used in IFD 0 of DNG files.
pub(in crate::image_file_directory::tags) const ExtraCameraProfiles: TagIdentifier = 0xC6F5;

/// Used in IFD 0 of DNG files.
pub(in crate::image_file_directory::tags) const AsShotProfileName: TagIdentifier = 0xC6F6;

/// Used in Raw IFD or Enhanced IFD of DNG files.
pub(in crate::image_file_directory::tags) const NoiseReductionApplied: TagIdentifier = 0xC6F7;

/// Used in IFD 0 or Camera Profile IFD of DNG files.
pub(in crate::image_file_directory::tags) const ProfileName: TagIdentifier = 0xC6F8;

/// Used in IFD 0 or Camera Profile IFD of DNG files.
pub(in crate::image_file_directory::tags) const ProfileHueSatMapDims: TagIdentifier = 0xC6F9;

/// Used in IFD 0 or Camera Profile IFD of DNG files.
pub(in crate::image_file_directory::tags) const ProfileHueSatMapData1: TagIdentifier = 0xC6FA;

/// Used in IFD 0 or Camera Profile IFD of DNG files.
pub(in crate::image_file_directory::tags) const ProfileHueSatMapData2: TagIdentifier = 0xC6FB;

/// Used in IFD 0 or Camera Profile IFD of DNG files.
pub(in crate::image_file_directory::tags) const ProfileToneCurve: TagIdentifier = 0xC6FC;

/// Used in IFD 0 or Camera Profile IFD of DNG files.
pub(in crate::image_file_directory::tags) const ProfileEmbedPolicy: TagIdentifier = 0xC6FD;

/// Used in IFD 0 or Camera Profile IFD of DNG files.
pub(in crate::image_file_directory::tags) const ProfileCopyright: TagIdentifier = 0xC6FE;

/// Used in IFD 0 or Camera Profile IFD of DNG files.
pub(in crate::image_file_directory::tags) const ForwardMatrix1: TagIdentifier = 0xC714;

/// Used in IFD 0 or Camera Profile IFD of DNG files.
pub(in crate::image_file_directory::tags) const ForwardMatrix2: TagIdentifier = 0xC715;

/// Used in Preview IFD of DNG files.
pub(in crate::image_file_directory::tags) const PreviewApplicationName: TagIdentifier = 0xC716;

/// Used in Preview IFD of DNG files.
pub(in crate::image_file_directory::tags) const PreviewApplicationVersion: TagIdentifier = 0xC717;

/// Used in Preview IFD of DNG files.
pub(in crate::image_file_directory::tags) const PreviewSettingsName: TagIdentifier = 0xC718;

/// Used in Preview IFD of DNG files.
pub(in crate::image_file_directory::tags) const PreviewSettingsDigest: TagIdentifier = 0xC719;

/// Used in Preview IFD of DNG files.
pub(in crate::image_file_directory::tags) const PreviewColorSpace: TagIdentifier = 0xC71A;

/// Used in Preview IFD of DNG files.
pub(in crate::image_file_directory::tags) const PreviewDateTime: TagIdentifier = 0xC71B;

/// Used in IFD 0 of DNG files.
pub(in crate::image_file_directory::tags) const RawImageDigest: TagIdentifier = 0xC71C;

/// Used in IFD 0 of DNG files.
pub(in crate::image_file_directory::tags) const OriginalRawFileDigest: TagIdentifier = 0xC71D;

/// Used in Raw IFD of DNG files.
pub(in crate::image_file_directory::tags) const SubTileBlockSize: TagIdentifier = 0xC71E;

/// Used in Raw IFD of DNG files.
pub(in crate::image_file_directory::tags) const RowInterleaveFactor: TagIdentifier = 0xC71F;

/// Used in IFD 0 or Camera Profile IFD of DNG files.
pub(in crate::image_file_directory::tags) const ProfileLookTableDims: TagIdentifier = 0xC725;

/// Used in IFD 0 or Camera Profile IFD of DNG files.
pub(in crate::image_file_directory::tags) const ProfileLookTableData: TagIdentifier = 0xC726;

/// Used in Raw IFD of DNG files.
pub(in crate::image_file_directory::tags) const OpcodeList1: TagIdentifier = 0xC740;

/// Used in Raw IFD of DNG files.
pub(in crate::image_file_directory::tags) const OpcodeList2: TagIdentifier = 0xC741;

/// Used in Raw IFD of DNG files.
pub(in crate::image_file_directory::tags) const OpcodeList3: TagIdentifier = 0xC74E;

/// Used in Raw IFD or Enhanced IFD of DNG files.
pub(in crate::image_file_directory::tags) const NoiseProfile: TagIdentifier = 0xC761;

/// Used in IFD 0 of DNG files.
pub(in crate::image_file_directory::tags) const OriginalDefaultFinalSize: TagIdentifier = 0xC791;

/// Used in IFD 0 of DNG files.
pub(in crate::image_file_directory::tags) const OriginalBestQualityFinalSize: TagIdentifier = 0xC792;

/// Used in IFD 0 of DNG files.
pub(in crate::image_file_directory::tags) const OriginalDefaultCropSize: TagIdentifier = 0xC793;

/// Used in IFD 0 or Camera Profile IFD of DNG files.
pub(in crate::image_file_directory::tags) const ProfileHueSatMapEncoding: TagIdentifier = 0xC7A3;

/// Used in IFD 0 or Camera Profile IFD of DNG files.
pub(in crate::image_file_directory::tags) const ProfileLookTableEncoding: TagIdentifier = 0xC7A4;

/// Used in IFD 0 or Camera Profile IFD of DNG files.
pub(in crate::image_file_directory::tags) const BaselineExposureOffset: TagIdentifier = 0xC7A5;

/// Used in IFD 0 or Camera Profile IFD of DNG files.
pub(in crate::image_file_directory::tags) const DefaultBlackRender: TagIdentifier = 0xC7A6;

/// Used in IFD 0 of DNG files.
pub(in crate::image_file_directory::tags) const NewRawImageDigest: TagIdentifier = 0xC7A7;

/// Used in Preview IFD of DNG files.
pub(in crate::image_file_directory::tags) const RawToPreviewGain: TagIdentifier = 0xC7A8;

/// Used in Raw IFD of DNG files.
pub(in crate::image_file_directory::tags) const DefaultUserCrop: TagIdentifier = 0xC7B5;

/// Used in IFD 0 of DNG files.
pub(in crate::image_file_directory::tags) const DepthFormat: TagIdentifier = 0xC7E9;

/// Used in IFD 0 of DNG files.
pub(in crate::image_file_directory::tags) const DepthNear: TagIdentifier = 0xC7EA;

/// Used in IFD 0 of DNG files.
pub(in crate::image_file_directory::tags) const DepthFar: TagIdentifier = 0xC7EB;

/// Used in IFD 0 of DNG files.
pub(in crate::image_file_directory::tags) const DepthUnits: TagIdentifier = 0xC7EC;

/// Used in IFD 0 of DNG files.
pub(in crate::image_file_directory::tags) const DepthMeasureType: TagIdentifier = 0xC7ED;

/// Used in Enhanced IFD of DNG files.
pub(in crate::image_file_directory::tags) const EnhanceParams: TagIdentifier = 0xC7EE;
