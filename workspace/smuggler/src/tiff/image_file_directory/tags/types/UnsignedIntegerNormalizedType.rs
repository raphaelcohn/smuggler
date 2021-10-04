// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


/// Where a tag supports more than one size (eg ImageWidth can be SHORT or LONG, or, in BigTIFF, presumably LONG8), this specifies the largest defined possible type.
/// It is then the responsibility of a normalizing writer to reduce the size if possible to fit the smallest possible type.
pub trait UnsignedIntegerNormalizedType: IntegerNormalizedType
{
}

impl UnsignedIntegerNormalizedType for u8
{
}

impl UnsignedIntegerNormalizedType for u16
{
}

impl UnsignedIntegerNormalizedType for u32
{
}

impl UnsignedIntegerNormalizedType for u64
{
}
