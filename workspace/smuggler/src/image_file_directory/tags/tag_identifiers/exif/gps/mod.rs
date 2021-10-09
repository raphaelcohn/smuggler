// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


use super::super::TagIdentifier;


/// Indicates the version of GPSInfoIFD.
pub(in crate::image_file_directory::tags) const GPSVersionID: TagIdentifier = 0x0000;

/// Indicates whether the latitude is north or south latitude.
pub(in crate::image_file_directory::tags) const GPSLatitudeRef: TagIdentifier = 0x0001;

/// Indicates the latitude.
pub(in crate::image_file_directory::tags) const GPSLatitude: TagIdentifier = 0x0002;

/// Indicates whether the longitude is east or west longitude.
pub(in crate::image_file_directory::tags) const GPSLongitudeRef: TagIdentifier = 0x0003;

/// Indicates the longitude.
pub(in crate::image_file_directory::tags) const GPSLongitude: TagIdentifier = 0x0004;

/// Indicates the altitude used as the reference altitude.
pub(in crate::image_file_directory::tags) const GPSAltitudeRef: TagIdentifier = 0x0005;

/// Indicates the altitude based on the reference in GPSAltitudeRef.
pub(in crate::image_file_directory::tags) const GPSAltitude: TagIdentifier = 0x0006;

/// Indicates the time as UTC (Coordinated Universal Time).
pub(in crate::image_file_directory::tags) const GPSTimeStamp: TagIdentifier = 0x0007;

/// Indicates the GPS satellites used for measurements.
pub(in crate::image_file_directory::tags) const GPSSatellites: TagIdentifier = 0x0008;

/// Indicates the status of the GPS receiver when the image is recorded.
pub(in crate::image_file_directory::tags) const GPSStatus: TagIdentifier = 0x0009;

/// Indicates the GPS measurement mode.
pub(in crate::image_file_directory::tags) const GPSMeasureMode: TagIdentifier = 0x000A;

/// Indicates the GPS DOP (data degree of precision).
pub(in crate::image_file_directory::tags) const GPSDOP: TagIdentifier = 0x000B;

/// Indicates the unit used to express the GPS receiver speed of movement.
pub(in crate::image_file_directory::tags) const GPSSpeedRef: TagIdentifier = 0x000C;

/// Indicates the speed of GPS receiver movement.
pub(in crate::image_file_directory::tags) const GPSSpeed: TagIdentifier = 0x000D;

/// Indicates the reference for giving the direction of GPS receiver movement.
pub(in crate::image_file_directory::tags) const GPSTrackRef: TagIdentifier = 0x000E;

/// Indicates the direction of GPS receiver movement.
pub(in crate::image_file_directory::tags) const GPSTrack: TagIdentifier = 0x000F;

/// Indicates the reference for giving the direction of the image when it is captured.
pub(in crate::image_file_directory::tags) const GPSImgDirectionRef : TagIdentifier = 0x0010 ;

/// Indicates the direction of the image when it was captured.
pub(in crate::image_file_directory::tags) const GPSImgDirection: TagIdentifier = 0x0011;

/// Indicates the geodetic survey data used by the GPS receiver.
pub(in crate::image_file_directory::tags) const GPSMapDatum: TagIdentifier = 0x0012;

/// Indicates whether the latitude of the destination point is north or south latitude.
pub(in crate::image_file_directory::tags) const GPSDestLatitudeRef : TagIdentifier = 0x0013 ;

/// Indicates the latitude of the destination point.
pub(in crate::image_file_directory::tags) const GPSDestLatitude: TagIdentifier = 0x0014;

/// Indicates whether the longitude of the destination point is east or west longitude.
pub(in crate::image_file_directory::tags) const GPSDestLongitudeRef : TagIdentifier = 0x0015 ;

/// Indicates the longitude of the destination point.
pub(in crate::image_file_directory::tags) const GPSDestLongitude: TagIdentifier = 0x0016;

/// Indicates the reference used for giving the bearing to the destination point.
pub(in crate::image_file_directory::tags) const GPSDestBearingRef : TagIdentifier = 0x0017 ;

/// Indicates the bearing to the destination point.
pub(in crate::image_file_directory::tags) const GPSDestBearing: TagIdentifier = 0x0018;

/// Indicates the unit used to express the distance to the destination point.
pub(in crate::image_file_directory::tags) const GPSDestDistanceRef : TagIdentifier = 0x0019 ;

/// Indicates the distance to the destination point.
pub(in crate::image_file_directory::tags) const GPSDestDistance: TagIdentifier = 0x001A;

/// A character string recording the name of the method used for location finding.
pub(in crate::image_file_directory::tags) const GPSProcessingMethod : TagIdentifier = 0x001B ;

/// A character string recording the name of the GPS area.
pub(in crate::image_file_directory::tags) const GPSAreaInformation: TagIdentifier = 0x001C;

/// A character string recording date and time information relative to UTC (Coordinated Universal Time).
pub(in crate::image_file_directory::tags) const GPSDateStamp : TagIdentifier = 0x001D ;

/// Indicates whether differential correction is applied to the GPS receiver.
pub(in crate::image_file_directory::tags) const GPSDifferential: TagIdentifier = 0x001E;

/// Horizontal positioning error.
///
/// Added after EXIF 2.2 (probably in 2.3; present in EXIF 2.32).
pub(in crate::image_file_directory::tags) const GPSHPositioningError: TagIdentifier = 0x001F;
