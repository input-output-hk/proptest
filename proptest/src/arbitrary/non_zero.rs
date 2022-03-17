//-
// Copyright 2022, The proptest developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use core::num::{
    NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize,
    NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
};

use crate::{
    arbitrary::{any, Arbitrary},
    strategy::{BoxedStrategy, Filter},
};

use super::StrategyFor;

macro_rules! non_zero_impl {
    ($base:ty, $non_zero:ty) => {
        impl Arbitrary for $non_zero {
            type Parameters = ();
            type Strategy = BoxedStrategy<Self>;

            fn arbitrary_with((): Self::Parameters) -> Self::Strategy {
                any::<$base>()
                    .prop_filter("must be non-zero", |i| i != 0)
                    .prop_map(|i| i.try_into().unwrap())
                    .boxed()
            }
        }
    };
}

non_zero_impl!(u8, NonZeroU8);
non_zero_impl!(u16, NonZeroU16);
non_zero_impl!(u32, NonZeroU32);
non_zero_impl!(u64, NonZeroU64);
non_zero_impl!(u128, NonZeroU128);
non_zero_impl!(usize, NonZeroUsize);
non_zero_impl!(i8, NonZeroI8);
non_zero_impl!(i16, NonZeroI16);
non_zero_impl!(i32, NonZeroI32);
non_zero_impl!(i64, NonZeroI64);
non_zero_impl!(i128, NonZeroI128);
non_zero_impl!(isize, NonZeroIsize);
