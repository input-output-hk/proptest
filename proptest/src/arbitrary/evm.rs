use primitive_types::{H128, H160, H256, H512, U128, U256, U512};

use crate::strategy::Map;

use super::Strategy;
use super::{any, Arbitrary, StrategyFor};

macro_rules! evm_impl {
    ($t:ty, $bytes:literal) => {
        impl Arbitrary for &t {
            type Parameters = ();
            type Strategy =
                Map<StrategyFor<[u8; $bytes]>, fn([u8; $bytes]) -> Self>;

            fn arbitrary_with((): Self::Parameters) -> Self::Strategy {
                any::<[u8; $bytes]>().prop_map(|bytes| $t::from_slice(&bytes))
            }
        }
    };
}

evm_impl!(H128, 16);
evm_impl!(H160, 20);
evm_impl!(H256, 32);
evm_impl!(H512, 64);

evm_impl!(U128, 16);
evm_impl!(U256, 32);
evm_impl!(U512, 64);

