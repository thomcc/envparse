//! A crate which allows parsing environment variables defined at compile time
//! into constants using `const fn` (rather than proc macros).
//!
//! See [`parse_env`](macro@parse_env) for the main entry point into the
//! library.
//!
//! # Motivation
//!
//! [`env!`](macro@env) and [`option_env!`](macro@option_env) macros are useful
//! for allowing certain forms of compile-time customization for libraries and
//! programs, however they're unfortunately limited: they always produce a
//! `&str` or `Option<&str>` as a result. This works so long as you need a
//! string, and not something like a number.
//!
//! In many cases, it's desirable to allow something like an array size to be
//! configured at compile time. This pattern is fairly common in C and C++ code,
//! where it's handled by allowing the user to tune these values for their use,
//! possibly by providing something like `-DFOOBAR_SIZE=32`[^1] via an
//! environment variable like `CFLAGS`.
//!
//! Unfortunately, using one of these strings as an array length requires it be
//! parsed at compile time, either in a proc macro, or a `const fn`. Both of
//! these have downsides: proc macros are extremely slow to compile, and
//!
//! Unfortunately, `const fn` is very limited in Rust, so parsing this is a
//! pain. That's what this library is for.
//!
//! [^1]: Or `/D` with MSVC — you get the idea.
//!
//! ```
//! use envparse::parse_env;
//! // parse `MYCRATE_MAX_THING_LEN` from the environment,
//! // defaulting to 4 if not provided.
//! const MAX_THING_LEN: usize = parse_env!("MYCRATE_MAX_THING_LEN" as usize else 4);
//! struct Thing {
//!     len: [u8; MAX_THING_LEN],
//! }
//! ```
//!
//! # Supported types
//!
//! Currently, the following types are supported:
//!
//! ## Primitive integers
//!
//! The primitive integer types are all supported: `i8`, `u8`, `i16`, `u16`,
//! `i32`, `u32, `i64`, `u64`, `i128`, `u128`, `isize` and `usize`.
//!
//! These mostly follow a (slight superset of) Rust's syntax, with the exception
//! that a trailing type indicator is not allowed.
//!
//! ## Booleans
//!
//! Booleans are supported, following some mostly ad-hoc conventions described
//! by the table. As with integers, the parsing is not case-sensitive and
//! ignores leading and trailing whitespace
//!
//! Note that the empty string is not considered a valid bool, so `FOOBAR=""`
//! neither works to enable or disable something.
//!
//! | `bool` value | accepted strings (case-insensitive, trimmed) |
//! | :--          | :--                                          |
//! | `false`      | `0`, `false`, `f`, `off`, `no` or `n`        |
//! | `true`       | `1`, `true`, `t`, `on`, `yes` or `y`         |
//!
//! # Syntax
//!
//! ## Integers
//!
//! Integers are parsed as follows with a couple notes:
//!
//! 1. Whitespace is ignored at the start or end of the input.
//! 2. Input is not case-sensitive. `0XABC` is equivalent to `0xabc`.
//! 3. `+` is allowed as a sign prefix, unlike in Rust's syntax.
//! 4. Unsigned integers reject a leading `-` sign early, but for the most part
//!    bounds/ranges are not checked until after parsing.
//!
//! ```txt
//! integer: ('+' | '-')? (dec_int | oct_int | bin_int | hex_int)
//!
//! dec_int: digit_dec (digit_dec | '_')*
//! hex_int: '0x' (digit_hex | '_')* digit_hex (digit_hex | '_')*
//! oct_int: '0o' (digit_oct | '_')* digit_oct (digit_oct | '_')*
//! bin_int: '0b' (digit_bin | '_')* digit_bin (digit_bin | '_')*
//! digit_bin: [0-1]
//! digit_oct: [0-7]
//! digit_dec: [0-9]
//! digit_hex: [0-9a-fA-F]
//! ```
//!
//! ## Booleans
//!
//! This is entirely case-insensitive, and any whitespace is trimmed from either
//! end.
//!
//! We're fairly forgiving here (perhaps more-so than we should be), in order to
//! be compatible with some other ways of configuration (rustc's command line
//! arguments, for example).
//!
//! ```txt
//! boolean: (true_str | false_str)
//! false_str: ( '0' | 'false' | 'f' | 'off' | 'no'  | 'n' )
//! true_str:  ( '1' | 'true'  | 't' | 'on'  | 'yes' | 'y' )
//! ```
#![no_std]

/// Not part of the public API. Please do not use.
mod privat;

#[doc(hidden)]
pub mod __priv {
    // Export stuff we need from the macro.
    pub use core;
    pub use core::option::Option::{self, None, Some};

    pub use crate::privat::{parse_bounded, parsers, RangeWrap};
}

/// Parse an environment variable into some value. The main entry-point of this
/// library.
///
/// Here's an example
/// ```
/// const MAX_LEN: usize = envparse::parse_env!("MYCRATE_MAX_THING_LEN" as usize else 64);
/// struct Thing {
///     len: [u8; MAX_LEN],
/// }
/// ```
///
/// Here's one that deliberately fails
///
/// ```compile_fail
/// const MAX_LEN: usize = envparse::parse_env!("MUST_BE_USER_PROVIDED" as usize);
/// struct Thing {
///     len: [u8; MAX_LEN],
/// }
/// ```
///
/// You can bound by ranges too. This one will fail because the
/// `MUST_BE_USER_PROVIDED` var isn't provided.
///
/// ```compile_fail
/// const MAX_LEN_LOG2: u32 = envparse::parse_env!("MUST_BE_USER_PROVIDED" as u32 in 1..32);
/// const MAX_LEN: usize = 1 << MAX_LEN_LOG2;
/// struct Thing {
///     len: [u8; MAX_LEN],
/// }
/// ```
///
/// If it's optional and you want an `Option` out of it, you can use `try`:
///
/// ```
/// const MAX_LEN: usize = match envparse::parse_env!(try "OPTIONAL_MAX_LEN_LOG2" as u32 in 1..32) {
///     Some(v) => 1 << v,
///     None => 0x80,
/// };
/// struct Thing {
///     len: [u8; MAX_LEN],
/// }
/// ```
#[macro_export]
macro_rules! parse_env {
    ($var_name:literal as $typ:ident) => {{
        const {
            match $crate::__priv::parsers::$typ($crate::__priv::core::env!($var_name).as_bytes(), $crate::__priv::None)
            {
                $crate::__priv::Some(v) => v,
                $crate::__priv::None => {
                    $crate::__priv::core::panic!($crate::__priv::core::concat!(
                        "error: the value in `",
                        $crate::__priv::core::stringify!($s),
                        "` doesn't parse as a `",
                        $crate::__priv::core::stringify!($typ),
                        "`, or is out of range.",
                    ));
                }
            }
        }
    }};

    ($var_name:literal as $typ:ident else $default:expr) => {{
        const {
            const __ENVPARSE_DEFAULT: $typ = $default;
            match $crate::__priv::core::option_env!($var_name) {
                $crate::__priv::None => __ENVPARSE_DEFAULT,
                $crate::__priv::Some(s) => {
                    match $crate::__priv::parsers::$typ(s.as_bytes(), $crate::__priv::Some(__ENVPARSE_DEFAULT)) {
                        $crate::__priv::Some(v) => v,
                        $crate::__priv::None => {
                            $crate::__priv::core::panic!($crate::__priv::core::concat!(
                                "error: the value in `",
                                $crate::__priv::core::stringify!($s),
                                "` doesn't parse as a `",
                                $crate::__priv::core::stringify!($typ),
                                "`, or is out of range.",
                            ));
                        }
                    }
                }
            }
        }
    }};

    ($var_name:literal as $typ:ident in $range:expr) => {{
        const {
            match $crate::__priv::parse_bounded::$typ(
                $crate::__priv::core::env!($var_name).as_bytes(),
                $crate::__priv::None,
                $crate::__priv::Some(
                    $crate::__priv::RangeWrap($range, $crate::__priv::core::marker::PhantomData::<$typ>).start(),
                ),
                $crate::__priv::Some(
                    $crate::__priv::RangeWrap($range, $crate::__priv::core::marker::PhantomData::<$typ>).end_incl(),
                ),
                false, // clamp
            ) {
                $crate::__priv::Some(v) => v,
                $crate::__priv::None => {
                    $crate::__priv::core::panic!($crate::__priv::core::concat!(
                        "error: the value in ",
                        $crate::__priv::core::stringify!($s),
                        " doesn't parse as a `",
                        $crate::__priv::core::stringify!($typ),
                        "`, or is outside of the range `",
                        $crate::__priv::core::stringify!($range),
                        "`."
                    ));
                }
            }
        }
    }};

    ($var_name:literal as $typ:ident (in $range:expr) else $default:expr) => {{
        const {
            const __ENVPARSE_DEFAULT: $typ = $default;
            match $crate::__priv::core::option_env!($var_name) {
                $crate::__priv::None => __ENVPARSE_DEFAULT,
                $crate::__priv::Some(s) => {
                    match $crate::__priv::parse_bounded::$typ(
                        s.as_bytes(),
                        $crate::__priv::Some(__ENVPARSE_DEFAULT),
                        $crate::__priv::Some(
                            $crate::__priv::RangeWrap($range, $crate::__priv::core::marker::PhantomData::<$typ>)
                                .start(),
                        ),
                        $crate::__priv::Some(
                            $crate::__priv::RangeWrap($range, $crate::__priv::core::marker::PhantomData::<$typ>)
                                .end_incl(),
                        ),
                        false, // clamp
                    ) {
                        $crate::__priv::Some(v) => v,
                        $crate::__priv::None => {
                            $crate::__priv::core::panic!($crate::__priv::core::concat!(
                                "error: the value in ",
                                $crate::__priv::core::stringify!($s),
                                " doesn't parse as a `",
                                $crate::__priv::core::stringify!($typ),
                                "`, or is outside of the range`",
                                $crate::__priv::core::stringify!($range),
                                "`."
                            ));
                        }
                    }
                }
            }
        }
    }};

    (try $var_name:literal as $typ:ident) => {{
        const {
            match $crate::__priv::core::option_env!($var_name) {
                $crate::__priv::None => $crate::__priv::None,
                $crate::__priv::Some(s) if s.is_empty() => $crate::__priv::None,
                $crate::__priv::Some(s) => match $crate::__priv::parsers::$typ(s.as_bytes(), $crate::__priv::None) {
                    $crate::__priv::None => {
                        $crate::__priv::core::panic!($crate::__priv::core::concat!(
                            "error: the value in ",
                            $crate::__priv::core::stringify!($s),
                            " doesn't parse as a `",
                            $crate::__priv::core::stringify!($typ),
                            "`, or is out of range.",
                        ));
                    }
                    opt => opt,
                },
            }
        }
    }};

    (try $var_name:literal as $typ:ident in $range:expr) => {{
        const {
            match ::core::option_env!($var_name) {
                $crate::__priv::None => $crate::__priv::None,
                $crate::__priv::Some(s) if s.is_empty() => $crate::__priv::None,
                $crate::__priv::Some(s) => match $crate::__priv::parse_bounded::$typ(
                    s.as_bytes(),
                    $crate::__priv::None,
                    $crate::__priv::Some(
                        $crate::__priv::RangeWrap($range, $crate::__priv::core::marker::PhantomData::<$typ>).start(),
                    ),
                    $crate::__priv::Some(
                        $crate::__priv::RangeWrap($range, $crate::__priv::core::marker::PhantomData::<$typ>).end_incl(),
                    ),
                    false, // clamp
                ) {
                    $crate::__priv::None => {
                        ::core::panic!(::core::concat!(
                            "error: the value in ",
                            ::core::stringify!($s),
                            " doesn't parse as a `",
                            ::core::stringify!($typ),
                            "`, or is outside of the range `",
                            ::core::stringify!($range),
                            "`.",
                        ));
                    }
                    opt => opt,
                },
            }
        }
    }};
}

pub mod parse;
