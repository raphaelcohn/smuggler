// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


use crate::collections::ByteOrder;
use crate::collections::Index;
use crate::collections::TiffBytes;
use std::alloc::Allocator;
use std::cmp::Ordering;
use std::mem::transmute;
use super::EnumRepresentationU16;
use super::RawTagKey;
use super::Tag;
use super::TagIdentifier;
use super::TagKey;
use super::UnrecognizedRepresentationValue;
use super::UnrecognizedTagValue;
use super::parsers::TagParseError;
use super::parsers::TagParser;
use super::parsers::Version6OrBigTiffUnit;
use super::tag_identifiers::public::baseline::Artist;
use super::tag_identifiers::public::baseline::BitsPerSample;
use super::tag_identifiers::public::baseline::CellLength;
use super::tag_identifiers::public::baseline::CellWidth;
use super::tag_identifiers::public::baseline::ColorMap;
use super::tag_identifiers::public::baseline::Compression;
use super::tag_identifiers::public::baseline::Copyright;
use super::tag_identifiers::public::baseline::DateTime;
use super::tag_identifiers::public::baseline::ExtraSamples;
use super::tag_identifiers::public::baseline::FillOrder;
use super::tag_identifiers::public::baseline::FreeByteCounts;
use super::tag_identifiers::public::baseline::FreeOffsets;
use super::tag_identifiers::public::baseline::GrayResponseCurve;
use super::tag_identifiers::public::baseline::GrayResponseUnit;
use super::tag_identifiers::public::baseline::HostComputer;
use super::tag_identifiers::public::baseline::ImageDescription;
use super::tag_identifiers::public::baseline::ImageLength;
use super::tag_identifiers::public::baseline::ImageWidth;
use super::tag_identifiers::public::baseline::Make;
use super::tag_identifiers::public::baseline::MaxSampleValue;
use super::tag_identifiers::public::baseline::MinSampleValue;
use super::tag_identifiers::public::baseline::Model;
use super::tag_identifiers::public::baseline::NewSubfileType;
use super::tag_identifiers::public::baseline::Orientation;
use super::tag_identifiers::public::baseline::PhotometricInterpretation;
use super::tag_identifiers::public::baseline::PlanarConfiguration;
use super::tag_identifiers::public::baseline::ResolutionUnit;
use super::tag_identifiers::public::baseline::RowsPerStrip;
use super::tag_identifiers::public::baseline::SamplesPerPixel;
use super::tag_identifiers::public::baseline::Software;
use super::tag_identifiers::public::baseline::StripByteCounts;
use super::tag_identifiers::public::baseline::StripOffsets;
use super::tag_identifiers::public::baseline::SubfileType;
use super::tag_identifiers::public::baseline::Threshholding;
use super::tag_identifiers::public::baseline::XResolution;
use super::tag_identifiers::public::baseline::YResolution;
use super::tag_identifiers::public::extension::BadFaxLines;
use super::tag_identifiers::public::extension::CleanFaxData;
use super::tag_identifiers::public::extension::ClipPath;
use super::tag_identifiers::public::extension::CodingMethods;
use super::tag_identifiers::public::extension::ConsecutiveBadFaxLines;
use super::tag_identifiers::public::extension::Decode;
use super::tag_identifiers::public::extension::DefaultImageColor;
use super::tag_identifiers::public::extension::DocumentName;
use super::tag_identifiers::public::extension::DotRange;
use super::tag_identifiers::public::extension::FaxProfile;
use super::tag_identifiers::public::extension::GlobalParametersIFD;
use super::tag_identifiers::public::extension::HalftoneHints;
use super::tag_identifiers::public::extension::ImageID;
use super::tag_identifiers::public::extension::ImageLayer;
use super::tag_identifiers::public::extension::Indexed;
use super::tag_identifiers::public::extension::InkNames;
use super::tag_identifiers::public::extension::InkSet;
use super::tag_identifiers::public::extension::JPEGACTables;
use super::tag_identifiers::public::extension::JPEGDCTables;
use super::tag_identifiers::public::extension::JPEGInterchangeFormat;
use super::tag_identifiers::public::extension::JPEGInterchangeFormatLength;
use super::tag_identifiers::public::extension::JPEGLosslessPredictors;
use super::tag_identifiers::public::extension::JPEGPointTransforms;
use super::tag_identifiers::public::extension::JPEGProc;
use super::tag_identifiers::public::extension::JPEGQTables;
use super::tag_identifiers::public::extension::JPEGRestartInterval;
use super::tag_identifiers::public::extension::JPEGTables;
use super::tag_identifiers::public::extension::ModeNumber;
use super::tag_identifiers::public::extension::NumberOfInks;
use super::tag_identifiers::public::extension::OPIProxy;
use super::tag_identifiers::public::extension::PageName;
use super::tag_identifiers::public::extension::PageNumber;
use super::tag_identifiers::public::extension::Predictor;
use super::tag_identifiers::public::extension::PrimaryChromaticities;
use super::tag_identifiers::public::extension::ProfileType;
use super::tag_identifiers::public::extension::ReferenceBlackWhite;
use super::tag_identifiers::public::extension::SMaxSampleValue;
use super::tag_identifiers::public::extension::SMinSampleValue;
use super::tag_identifiers::public::extension::SampleFormat;
use super::tag_identifiers::public::extension::StripRowCounts;
use super::tag_identifiers::public::extension::SubIFDs;
use super::tag_identifiers::public::extension::T4Options;
use super::tag_identifiers::public::extension::T6Options;
use super::tag_identifiers::public::extension::TargetPrinter;
use super::tag_identifiers::public::extension::TileByteCounts;
use super::tag_identifiers::public::extension::TileLength;
use super::tag_identifiers::public::extension::TileOffsets;
use super::tag_identifiers::public::extension::TileWidth;
use super::tag_identifiers::public::extension::TransferFunction;
use super::tag_identifiers::public::extension::TransferRange;
use super::tag_identifiers::public::extension::VersionYear;
use super::tag_identifiers::public::extension::WhitePoint;
use super::tag_identifiers::public::extension::XClipPathUnits;
use super::tag_identifiers::public::extension::XMP;
use super::tag_identifiers::public::extension::XPosition;
use super::tag_identifiers::public::extension::YCbCrCoefficients;
use super::tag_identifiers::public::extension::YCbCrPositioning;
use super::tag_identifiers::public::extension::YCbCrSubSampling;
use super::tag_identifiers::public::extension::YClipPathUnits;
use super::tag_identifiers::public::extension::YPosition;
use super::tag_identifiers::public::private::ActiveArea;
use super::tag_identifiers::public::private::AliasLayerMetadata;
use super::tag_identifiers::public::private::AnalogBalance;
use super::tag_identifiers::public::private::AntiAliasStrength;
use super::tag_identifiers::public::private::AsShotICCProfile;
use super::tag_identifiers::public::private::AsShotNeutral;
use super::tag_identifiers::public::private::AsShotPreProfileMatrix;
use super::tag_identifiers::public::private::AsShotProfileName;
use super::tag_identifiers::public::private::AsShotWhiteXY;
use super::tag_identifiers::public::private::BaselineExposure;
use super::tag_identifiers::public::private::BaselineExposureOffset;
use super::tag_identifiers::public::private::BaselineNoise;
use super::tag_identifiers::public::private::BaselineSharpness;
use super::tag_identifiers::public::private::BayerGreenSplit;
use super::tag_identifiers::public::private::BestQualityScale;
use super::tag_identifiers::public::private::BlackLevel;
use super::tag_identifiers::public::private::BlackLevelDeltaH;
use super::tag_identifiers::public::private::BlackLevelDeltaV;
use super::tag_identifiers::public::private::BlackLevelRepeatDim;
use super::tag_identifiers::public::private::CFALayout;
use super::tag_identifiers::public::private::CFAPlaneColor;
use super::tag_identifiers::public::private::CalibrationIlluminant1;
use super::tag_identifiers::public::private::CalibrationIlluminant2;
use super::tag_identifiers::public::private::CameraCalibration1;
use super::tag_identifiers::public::private::CameraCalibration2;
use super::tag_identifiers::public::private::CameraCalibrationSignature;
use super::tag_identifiers::public::private::CameraSerialNumber;
use super::tag_identifiers::public::private::ChromaBlurRadius;
use super::tag_identifiers::public::private::ColorMatrix1;
use super::tag_identifiers::public::private::ColorMatrix2;
use super::tag_identifiers::public::private::ColorimetricReference;
use super::tag_identifiers::public::private::CurrentICCProfile;
use super::tag_identifiers::public::private::CurrentPreProfileMatrix;
use super::tag_identifiers::public::private::DNGBackwardVersion;
use super::tag_identifiers::public::private::DNGPrivateData;
use super::tag_identifiers::public::private::DNGVersion;
use super::tag_identifiers::public::private::DefaultBlackRender;
use super::tag_identifiers::public::private::DefaultCropOrigin;
use super::tag_identifiers::public::private::DefaultCropSize;
use super::tag_identifiers::public::private::DefaultScale;
use super::tag_identifiers::public::private::DefaultUserCrop;
use super::tag_identifiers::public::private::DepthFar;
use super::tag_identifiers::public::private::DepthFormat;
use super::tag_identifiers::public::private::DepthMeasureType;
use super::tag_identifiers::public::private::DepthNear;
use super::tag_identifiers::public::private::DepthUnits;
use super::tag_identifiers::public::private::EnhanceParams;
use super::tag_identifiers::public::private::ExifIFD;
use super::tag_identifiers::public::private::ExtraCameraProfiles;
use super::tag_identifiers::public::private::ForwardMatrix1;
use super::tag_identifiers::public::private::ForwardMatrix2;
use super::tag_identifiers::public::private::GDAL_METADATA;
use super::tag_identifiers::public::private::GDAL_NODATA;
use super::tag_identifiers::public::private::GEO_METADATA;
use super::tag_identifiers::public::private::GPSIFD;
use super::tag_identifiers::public::private::GeoAsciiParamsTag;
use super::tag_identifiers::public::private::GeoDoubleParamsTag;
use super::tag_identifiers::public::private::GeoKeyDirectoryTag;
use super::tag_identifiers::public::private::HylaFAXFaxRecvParams;
use super::tag_identifiers::public::private::HylaFAXFaxRecvTime;
use super::tag_identifiers::public::private::HylaFAXFaxSubAddress;
use super::tag_identifiers::public::private::ICCProfile;
use super::tag_identifiers::public::private::INGRFlagRegisters;
use super::tag_identifiers::public::private::INGRPacketDataTag;
use super::tag_identifiers::public::private::IPTC;
use super::tag_identifiers::public::private::ImageSourceData;
use super::tag_identifiers::public::private::InteroperabilityIFD;
use super::tag_identifiers::public::private::IrasBTransformationMatrix;
use super::tag_identifiers::public::private::LensInfo;
use super::tag_identifiers::public::private::LinearResponseLimit;
use super::tag_identifiers::public::private::LinearizationTable;
use super::tag_identifiers::public::private::LocalizedCameraModel;
use super::tag_identifiers::public::private::MDColorTable;
use super::tag_identifiers::public::private::MDFileTag;
use super::tag_identifiers::public::private::MDFileUnits;
use super::tag_identifiers::public::private::MDLabName;
use super::tag_identifiers::public::private::MDPrepDate;
use super::tag_identifiers::public::private::MDPrepTime;
use super::tag_identifiers::public::private::MDSampleInfo;
use super::tag_identifiers::public::private::MDScalePixel;
use super::tag_identifiers::public::private::MakerNoteSafety;
use super::tag_identifiers::public::private::MaskedAreas;
use super::tag_identifiers::public::private::ModelPixelScaleTag;
use super::tag_identifiers::public::private::ModelTiepointTag;
use super::tag_identifiers::public::private::ModelTransformationTag;
use super::tag_identifiers::public::private::NewRawImageDigest;
use super::tag_identifiers::public::private::NoiseProfile;
use super::tag_identifiers::public::private::NoiseReductionApplied;
use super::tag_identifiers::public::private::OceApplicationSelector;
use super::tag_identifiers::public::private::OceIdentificationNumber;
use super::tag_identifiers::public::private::OceImageLogicCharacteristics;
use super::tag_identifiers::public::private::OceScanJobDescription;
use super::tag_identifiers::public::private::OpcodeList1;
use super::tag_identifiers::public::private::OpcodeList2;
use super::tag_identifiers::public::private::OpcodeList3;
use super::tag_identifiers::public::private::OriginalBestQualityFinalSize;
use super::tag_identifiers::public::private::OriginalDefaultCropSize;
use super::tag_identifiers::public::private::OriginalDefaultFinalSize;
use super::tag_identifiers::public::private::OriginalRawFileData;
use super::tag_identifiers::public::private::OriginalRawFileDigest;
use super::tag_identifiers::public::private::OriginalRawFileName;
use super::tag_identifiers::public::private::Photoshop;
use super::tag_identifiers::public::private::PreviewApplicationName;
use super::tag_identifiers::public::private::PreviewApplicationVersion;
use super::tag_identifiers::public::private::PreviewColorSpace;
use super::tag_identifiers::public::private::PreviewDateTime;
use super::tag_identifiers::public::private::PreviewSettingsDigest;
use super::tag_identifiers::public::private::PreviewSettingsName;
use super::tag_identifiers::public::private::ProfileCalibrationSignature;
use super::tag_identifiers::public::private::ProfileCopyright;
use super::tag_identifiers::public::private::ProfileEmbedPolicy;
use super::tag_identifiers::public::private::ProfileHueSatMapData1;
use super::tag_identifiers::public::private::ProfileHueSatMapData2;
use super::tag_identifiers::public::private::ProfileHueSatMapDims;
use super::tag_identifiers::public::private::ProfileHueSatMapEncoding;
use super::tag_identifiers::public::private::ProfileLookTableData;
use super::tag_identifiers::public::private::ProfileLookTableDims;
use super::tag_identifiers::public::private::ProfileLookTableEncoding;
use super::tag_identifiers::public::private::ProfileName;
use super::tag_identifiers::public::private::ProfileToneCurve;
use super::tag_identifiers::public::private::RawDataUniqueID;
use super::tag_identifiers::public::private::RawImageDigest;
use super::tag_identifiers::public::private::RawToPreviewGain;
use super::tag_identifiers::public::private::ReductionMatrix1;
use super::tag_identifiers::public::private::ReductionMatrix2;
use super::tag_identifiers::public::private::RowInterleaveFactor;
use super::tag_identifiers::public::private::ShadowScale;
use super::tag_identifiers::public::private::SubTileBlockSize;
use super::tag_identifiers::public::private::TIFF_RSID;
use super::tag_identifiers::public::private::UniqueCameraModel;
use super::tag_identifiers::public::private::WangAnnotation;
use super::tag_identifiers::public::private::WhiteLevel;
use super::types::UnsignedIntegerTagType;
use super::types::AsciiCharacter;
use super::types::BitField;
use super::types::BitFieldInteger;
use std::marker::PhantomData;
use std::hash::Hash;
use std::fmt::Debug;
use strum::EnumCount;
use strum::EnumIter;
use strum::EnumString;
use strum::IntoEnumIterator;
use strum::IntoStaticStr;
use strum::ToString;
use super::types::TagType;
use super::types::UnsignedIntegerValue;
use super::types::UnsignedInteger;
use super::types::EnumUnsignedInteger;
use super::types::UnsignedEnum;
use super::types::Unaligned;
use swiss_army_knife::byte_swap::ByteSwapUnalignedMemory;
use swiss_army_knife::byte_swap::Unaligned16;
use swiss_army_knife::byte_swap::Unaligned32;
use swiss_army_knife::byte_swap::Unaligned64;
use crate::tiff::image_file_directory::tags::parsers::SpecificTagParseError;
use crate::tiff::image_file_directory::tags::UnrecognizedTag;


include!("NewSubfileTypeBitField.rs");
include!("NewSubfileTypeBitFieldBit.rs");
include!("PublicTag.rs");
include!("PublicTagParser.rs");
include!("PublicTagKey.rs");
include!("SubfileTypeEnum.rs");
