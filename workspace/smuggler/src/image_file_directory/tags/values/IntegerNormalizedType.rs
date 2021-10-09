// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


/// A normalized type.
pub trait IntegerNormalizedType: Default + Debug + Copy + Eq + Ord + Hash
{
}

impl IntegerNormalizedType for u8
{
}

impl IntegerNormalizedType for u16
{
}

impl IntegerNormalizedType for u32
{
}

impl IntegerNormalizedType for u64
{
}

impl IntegerNormalizedType for i8
{
}

impl IntegerNormalizedType for i16
{
}

impl IntegerNormalizedType for i32
{
}

impl IntegerNormalizedType for i64
{
}
