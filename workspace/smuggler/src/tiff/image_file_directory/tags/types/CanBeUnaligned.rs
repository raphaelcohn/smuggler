// This file is part of smuggler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT. No part of smuggler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of smuggler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/smuggler/master/COPYRIGHT.


/// Can be unaligned.
pub trait CanBeUnaligned: Default + Debug + Copy + PartialEq + PartialOrd
{
	#[inline(always)]
	fn dereference(may_be_unaligned: *const Self, byte_order: ByteOrder) -> Self;
}

impl CanBeUnaligned for u16
{
	#[inline(always)]
	fn dereference(may_be_unaligned: *const Self, byte_order: ByteOrder) -> Self
	{
		byte_order.read_unaligned_u16(may_be_unaligned)
	}
}

impl CanBeUnaligned for u32
{
	#[inline(always)]
	fn dereference(may_be_unaligned: *const Self, byte_order: ByteOrder) -> Self
	{
		byte_order.read_unaligned_u32(may_be_unaligned)
	}
}

impl CanBeUnaligned for u64
{
	#[inline(always)]
	fn dereference(may_be_unaligned: *const Self, byte_order: ByteOrder) -> Self
	{
		byte_order.read_unaligned_u64(may_be_unaligned)
	}
}

impl CanBeUnaligned for i16
{
	#[inline(always)]
	fn dereference(may_be_unaligned: *const Self, byte_order: ByteOrder) -> Self
	{
		byte_order.read_unaligned_i16(may_be_unaligned)
	}
}

impl CanBeUnaligned for i32
{
	#[inline(always)]
	fn dereference(may_be_unaligned: *const Self, byte_order: ByteOrder) -> Self
	{
		byte_order.read_unaligned_i32(may_be_unaligned)
	}
}

impl CanBeUnaligned for i64
{
	#[inline(always)]
	fn dereference(may_be_unaligned: *const Self, byte_order: ByteOrder) -> Self
	{
		byte_order.read_unaligned_i64(may_be_unaligned)
	}
}

impl CanBeUnaligned for f32
{
	#[inline(always)]
	fn dereference(may_be_unaligned: *const Self, byte_order: ByteOrder) -> Self
	{
		byte_order.read_unaligned_f32(may_be_unaligned)
	}
}

impl CanBeUnaligned for f64
{
	#[inline(always)]
	fn dereference(may_be_unaligned: *const Self, byte_order: ByteOrder) -> Self
	{
		byte_order.read_unaligned_f64(may_be_unaligned)
	}
}
