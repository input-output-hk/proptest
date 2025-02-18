//-
// Copyright 2017 Jason Lingle
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Support for strategies producing fixed-length arrays.
//!
//! An array of strategies (but only length 1 to 32 for now) is itself a
//! strategy which generates arrays of that size drawing elements from the
//! corresponding input strategies.
//!
//! See also [`UniformArrayStrategy`](struct.UniformArrayStrategy.html) for
//! easily making a strategy for an array drawn from one strategy.
//!
//! General implementations are available for sizes 1 through 32.

use crate::std_facade::vec::Vec;

use core::convert::TryInto;
use core::iter;
use core::marker::PhantomData;

use crate::strategy::*;
use crate::test_runner::*;

/// A `Strategy` which generates fixed-size arrays containing values drawn from
/// an inner strategy.
///
/// `T` must be an array type of length 1 to 32 whose values are produced by
/// strategy `S`. Instances of this type are normally created by the various
/// `uniformXX` functions in this module.
///
/// This is mainly useful when the inner strategy is not `Copy`, precluding
/// expressing the strategy as `[myStrategy; 32]`, for example.
///
/// ## Example
///
/// ```
/// use proptest::prelude::*;
///
/// proptest! {
///   #[test]
///   fn test_something(a in prop::array::uniform32(1u32..)) {
///     let unexpected = [0u32;32];
///     // `a` is also a [u32;32], so we can compare them directly
///     assert_ne!(unexpected, a);
///   }
/// }
/// # fn main() { }
/// ```
#[must_use = "strategies do nothing unless used"]
#[derive(Clone, Copy, Debug)]
pub struct UniformArrayStrategy<S, T> {
    strategy: S,
    _marker: PhantomData<T>,
}

impl<S, T> UniformArrayStrategy<S, T> {
    /// Directly create a `UniformArrayStrategy`.
    ///
    /// This is only intended for advanced use, since the only way to specify
    /// the array size is with the turbofish operator and explicitly naming the
    /// type of the values in the array and the strategy itself.
    ///
    /// Prefer the `uniformXX` functions at module-level unless something
    /// precludes their use.
    pub fn new(strategy: S) -> Self {
        UniformArrayStrategy {
            strategy,
            _marker: PhantomData,
        }
    }
}

/// A `ValueTree` operating over a fixed-size array.
#[derive(Clone, Copy, Debug)]
pub struct ArrayValueTree<T> {
    tree: T,
    shrinker: usize,
    last_shrinker: Option<usize>,
}

/// Create a strategy to generate fixed-length arrays.
///
/// All values within the new strategy are generated using the given
/// strategy. The length of the array corresponds to the suffix of the
/// name of this function.
///
/// See [`UniformArrayStrategy`](struct.UniformArrayStrategy.html) for
/// example usage.
pub fn uniform<S: Strategy, const N: usize>(
    strategy: S,
) -> UniformArrayStrategy<S, [S::Value; N]> {
    UniformArrayStrategy {
        strategy,
        _marker: PhantomData,
    }
}

impl<S: Strategy, const N: usize> Strategy for [S; N] {
    type Tree = ArrayValueTree<[S::Tree; N]>;
    type Value = [S::Value; N];

    fn new_tree(&self, runner: &mut TestRunner) -> NewTree<Self> {
        let tree_vec = self
            .iter()
            .map(|strategy| strategy.new_tree(runner))
            .collect::<Result<Vec<_>, _>>()?;

        let tree = match tree_vec.try_into() {
            Ok(tree) => tree,
            Err(_) => panic!("Incorrect tree length"),
        };

        Ok(ArrayValueTree {
            tree,
            shrinker: 0,
            last_shrinker: None,
        })
    }
}

impl<S: Strategy, const N: usize> Strategy
    for UniformArrayStrategy<S, [S::Value; N]>
{
    type Tree = ArrayValueTree<[S::Tree; N]>;
    type Value = [S::Value; N];

    fn new_tree(&self, runner: &mut TestRunner) -> NewTree<Self> {
        let tree_vec = iter::repeat_with(|| self.strategy.new_tree(runner))
            .take(N)
            .collect::<Result<Vec<_>, _>>()?;

        let tree = match tree_vec.try_into() {
            Ok(tree) => tree,
            Err(_) => unreachable!("Incorrect tree length"),
        };

        Ok(ArrayValueTree {
            tree,
            shrinker: 0,
            last_shrinker: None,
        })
    }
}

impl<T: ValueTree, const N: usize> ValueTree for ArrayValueTree<[T; N]> {
    type Value = [T::Value; N];

    fn current(&self) -> [T::Value; N] {
        self.tree
            .iter()
            .map(|tree| tree.current())
            .collect::<Vec<_>>()
            .try_into()
            .expect("Incorrect tree size")
    }

    fn simplify(&mut self) -> bool {
        while self.shrinker < N {
            if self.tree[self.shrinker].simplify() {
                self.last_shrinker = Some(self.shrinker);
                return true;
            } else {
                self.shrinker += 1;
            }
        }

        false
    }

    fn complicate(&mut self) -> bool {
        if let Some(shrinker) = self.last_shrinker {
            self.shrinker = shrinker;
            if self.tree[shrinker].complicate() {
                true
            } else {
                self.last_shrinker = None;
                false
            }
        } else {
            false
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn shrinks_fully_ltr() {
        fn pass(a: [i32; 2]) -> bool {
            a[0] * a[1] <= 9
        }

        let input = [0..32, 0..32];
        let mut runner = TestRunner::deterministic();

        let mut cases_tested = 0;
        for _ in 0..256 {
            // Find a failing test case
            let mut case = input.new_tree(&mut runner).unwrap();
            if pass(case.current()) {
                continue;
            }

            loop {
                if pass(case.current()) {
                    if !case.complicate() {
                        break;
                    }
                } else {
                    if !case.simplify() {
                        break;
                    }
                }
            }

            let last = case.current();
            assert!(!pass(last));
            // Maximally shrunken
            assert!(pass([last[0] - 1, last[1]]));
            assert!(pass([last[0], last[1] - 1]));

            cases_tested += 1;
        }

        assert!(cases_tested > 32, "Didn't find enough test cases");
    }

    #[test]
    fn test_sanity() {
        check_strategy_sanity([(0i32..1000), (1i32..1000)], None);
    }
}
