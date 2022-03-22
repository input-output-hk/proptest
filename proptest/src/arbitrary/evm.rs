//-
// Copyright 2022 The proptest developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use primitive_types::{H128, H160, H256, H512, U128, U256, U512};

use crate::strategy::Map;

use super::Strategy;
use super::{any, Arbitrary, StrategyFor};

macro_rules! hash_impl {
    ($t:ty, $bytes:literal) => {
        impl Arbitrary for $t {
            type Parameters = ();
            type Strategy =
                Map<StrategyFor<[u8; $bytes]>, fn([u8; $bytes]) -> Self>;

            fn arbitrary_with((): Self::Parameters) -> Self::Strategy {
                any::<[u8; $bytes]>().prop_map(|bytes| <$t>::from_slice(&bytes))
            }
        }
    };
}

macro_rules! prim_impl {
    ($t:ty, $u64s:literal) => {
        impl Arbitrary for $t {
            type Parameters = ();
            type Strategy =
                Map<StrategyFor<[u64; $u64s]>, fn([u64; $u64s]) -> Self>;

            fn arbitrary_with((): Self::Parameters) -> Self::Strategy {
                any::<[u64; $u64s]>().prop_map(Self)
            }
        }
    };
}

hash_impl!(H128, 16);
hash_impl!(H160, 20);
hash_impl!(H256, 32);
hash_impl!(H512, 64);

prim_impl!(U128, 2);
prim_impl!(U256, 4);
prim_impl!(U512, 8);

