//-
// Copyright 2017 Jason Lingle
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// Easily define `proptest` tests.
///
/// Within `proptest!`, define one or more functions without return type
/// normally, except instead of putting `: type` after each parameter, write
/// `in strategy`, where `strategy` is an expression evaluating to some
/// `Strategy`.
///
/// Each function will be wrapped in a function which sets up a `TestRunner`,
/// and then invokes the function body with inputs generated according to the
/// strategies. Note that the inputs are borrowed from the test runner, so if
/// they are not `Copy`, you will need to use `ref` with each parameter name.
///
/// Example:
///
/// ```
/// #[macro_use] extern crate proptest;
///
/// proptest! {
///   # /*
///   #[test]
///   # */
///   fn test_addition(a in 0..10, b in 0..10) {
///     assert!(a + b <= 18);
///   }
///
///   // Note the `ref a` and `ref b` --- `String` is not `Copy`,
///   // so we can't take ownership implicitly.
///   # /*
///   #[test]
///   # */
///   fn test_string_concat(ref a in ".*", ref b in ".*") {
///     let cat = format!("{}{}", a, b);
///     assert_eq!(a.len() + b.len(), cat.len());
///   }
/// }
/// #
/// # fn main() { test_addition(); test_string_concat(); }
/// ```
///
/// To override the default configuration, you can start the `proptest!` block
/// with `#![proptest_config(expr)]`, where `expr` is an expression that
/// evaluates to a `proptest::test_runner::Config` (or a reference to one).
///
/// ```
/// #[macro_use] extern crate proptest;
/// use proptest::test_runner::Config;
///
/// proptest! {
///   #![proptest_config(Config { cases: 99, .. Config::default() })]
///   # /*
///   #[test]
///   # */
///   fn test_addition(a in 0..10, b in 0..10) {
///     assert!(a + b <= 18);
///   }
/// }
/// #
/// # fn main() { test_addition(); }
/// ```
#[macro_export]
macro_rules! proptest {
    (#![proptest_config($config:expr)]
     $(
        $(#[$meta:meta])*
        fn $test_name:ident($($parm:pat in $strategy:expr),+) $body:block
    )*) => {
        $(
            $(#[$meta])*
            fn $test_name() {
                let mut runner = $crate::test_runner::TestRunner::new(
                    $config.clone());
                match runner.run(
                    &proptest!(@_WRAP ($($strategy)*)),
                    |&proptest!(@_WRAPPAT ($($parm),*))| {
                        $body;
                        Ok(())
                    })
                {
                    Ok(_) => (),
                    Err(e) => panic!("{}\n{}", e, runner),
                }
            }
        )*
    };

    (@_WRAP ($item:tt)) => { $item };
    (@_WRAP ($a:tt $($rest:tt)*)) => {
        ($a, proptest!(@_WRAP ($($rest)*)))
    };
    (@_WRAPPAT ($item:pat)) => { $item };
    (@_WRAPPAT ($a:pat, $($rest:pat),*)) => {
        ($a, proptest!(@_WRAPPAT ($($rest),*)))
    };

    ($(
        $(#[$meta:meta])*
        fn $test_name:ident($($parm:pat in $strategy:expr),+) $body:block
    )*) => { proptest! {
        #![proptest_config($crate::test_runner::Config::default())]
        $($(#[$meta])*
          fn $test_name($($parm in $strategy),+) $body)*
    } };
}

/// Rejects the test input if assumptions are not met.
///
/// Used directly within a function defined with `proptest!` or in any function
/// returning `Result<_, TestCaseError>`.
///
/// This is invoked as `prop_assume!(condition, format, args...)`. `condition`
/// is evaluated; if it is false, `Err(TestCaseError::Reject)` is returned. The
/// message includes the point of invocation and the format message. `format`
/// and `args` may be omitted to simply use the condition itself as the
/// message.
#[macro_export]
macro_rules! prop_assume {
    ($expr:expr) => {
        prop_assume!($expr, "{}", stringify!($expr))
    };

    ($expr:expr, $fmt:tt $(, $fmt_arg:expr),*) => {
        if !$expr {
            return Err($crate::test_runner::TestCaseError::Reject(
                format!(concat!("{}:{}:{}: ", $fmt),
                        file!(), line!(), column!()
                        $(, $fmt_arg)*)));
        }
    };
}

#[cfg(test)]
mod test {
    proptest! {
        #[test]
        fn test_something(a in 0u32..42u32, b in 1u32..10u32) {
            prop_assume!(a != 41 || b != 9);
            assert!(a + b < 50);
        }
    }
}