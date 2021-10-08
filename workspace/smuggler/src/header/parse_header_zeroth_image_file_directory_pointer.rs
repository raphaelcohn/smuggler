// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


#[inline(always)]
pub(super) fn parse_header_zeroth_image_file_directory_pointer<TB: TiffBytes, Unit: Version6OrBigTiffUnit>(tiff_bytes_with_order: &TiffBytesWithOrder<TB>) -> Result<ImageFileDirectoryPointer, HeaderParseError>
{
	Unit::parse_header_constants(tiff_bytes_with_order)?;
	
	let zeroth_image_file_directory_pointer =
	{
		use ZerothImageFileDirectoryPointerParseError::*;
		
		let offset = tiff_bytes_with_order.offset::<Unit>(Unit::IndexOfZerothImageFileDirectory);
		let pointer = ImageFileDirectoryPointer::new_unchecked(offset).map_err(PointerToZerothImageFileDirectory)?;
		pointer.ok_or(ThereMustBeAtLeastOneImageFileDirectory)?
	};
	Ok(zeroth_image_file_directory_pointer)
}
