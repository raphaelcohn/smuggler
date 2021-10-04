// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


/*

see https://www.loc.gov/preservation/digital/formats/content/tiff_tags.shtml

33421	828D	CFARepeatPatternDim	For camera raw files from sensors with CFA overlay.
33422	828E	CFAPattern	For camera raw files from sensors with CFA overlay.
33423	828F	BatteryLevel	Encodes camera battery level at time of image capture.
33434	829A	ExposureTime	Exposure time, given in seconds.
33437	829D	FNumber	The F number.


34675	8773	InterColorProfile	ICC profile data.

TIFF/EP spec, p. 47

Exif private IFD
	Also called ICC Profile.

Also used by HD Photo

34850	8822	ExposureProgram	The class of the program used by the camera to set exposure when the picture is taken.	Exif Private IFD

TIFF/EP spec, p. 41
34852	8824	SpectralSensitivity	Indicates the spectral sensitivity of each channel of the camera used.	Exif Private IFD

TIFF/EP spec, p. 48
34853	8825	GPSInfo	A pointer to the Exif-related GPS Info IFD.

TIFF/EP spec, p. 34

Exif private IFD
	Also called GPS IFD.
34855	8827	ISOSpeedRatings	Indicates the ISO Speed and ISO Latitude of the camera or input device as specified in ISO 12232.	Exif Private IFD

TIFF/EP spec, p. 47
34856	8828	OECF	Indicates the Opto-Electric Conversion Function (OECF) specified in ISO 14524.	Exif Private IFD

TIFF/EP spec, p. 48
34857	8829	Interlace	Indicates the field number of multifield images.

TIFF/EP spec, p. 22

Exif private IFD

34858	882A	TimeZoneOffset	Encodes time zone of camera clock relative to GMT.

TIFF/EP spec, p. 38

Exif private IFD

34859	882B	SelfTimeMode	Number of seconds image capture was delayed from button press.

TIFF/EP spec, p. 45

Exif private IFD

AND MUCH MORE


 */
