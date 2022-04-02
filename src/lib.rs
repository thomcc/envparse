//! A crate which allows parsing environment variables defined at compile time
//! into constants.
//!
//! # Motivation
//!
//! [`env!`](macro@env) and [`option_env!`](macro@option_env) macros are useful
//! for allowing certain forms of compile-time customization for libraries and
//! programs, however they're unfortunately limited -- they always produce a
//! `&str` or `Option<&str>` as a result.
//!
//! In many cases, it's desirable to allow something like an array size to be
//! configured in this manner, but unfortunately the Rust stdlib's parsing
//! functionality does not support this, as .
//!
//! This pattern is fairly common in C and C++ code, where it's
//! handled by allowing the user to tune these values for their use, possibly by
//! providing something like `-DFOOBAR_SIZE=32`[^1] via an environment variable
//! like `CFLAGS`.
//!
//! [^1]: Or `/D` with MSVC — you get the idea.
//!
//! # Supported types
//!
//! Currently, the following types are supported:
//! - Primitive integers: `i8`, `u8`, `i16`, `u16`, `i32`, `u32, `i64`, `u64`,
//!   `i128`, `u128`, `isize` and `usize`. The syntax attempts to match the
//!   syntax accepted for Rust integer literals. Concretely:
//!     - `
//!     - Hexadecimal `0x1234_abcd` is suppo
//! - Booleans,
//!
//!
//! # Usage
//!
//! Parsing a number from a required environment variable.
//!
//! ```compile_fail
//! const MAX_LEN: usize = envparse::parse_env!("MUST_BE_USER_PROVIDED" as usize);
//! struct Thing {
//!     len: [u8; MAX_LEN],
//! }
//! ```
//!
//! That was a "`compile_fail`"" example because `MUST_BE_USER_PROVIDED` will
//! (hopefully) not be defined in the environment when running rustdoc. In real
//! code, it can be beneficial to handle the missing value. This can be done in
//! two ways.
//!
//! First, a default value can beprovide
//!
//! ```
//! const MAX_LEN: usize = envparse::parse_env!("MYCRATE_MAX_THING_LEN" as usize else 32);
//!
//! struct Thing {
//!     len: [u8; MAX_LEN],
//! }
//! ```
//!
//! Here's one that deliberately fails
//!
//!
//!
//! This is essentially
//!
//! No proc macros are used to perform this operation, and this crate has no
//! dependencies aside from libcore.
//!

// - hexadecimal integers roughly match the pattern `[-+]?0[xX][a-fA-F0-9_]+`,
//   with the exception that `0x___` is not supported (you need at least one
//   digit).
//
// - binary and octal integers are similar, but (with `0b` as the prefix and
//   `[01]` as the set of allowed digits), as well as octal (with `0o` as a
//   prefix, and `[0-7]` as the digits). As an extension, decimal may also be
//   explicitly specified, with a prefix of `0d`. All of these are case
//   insensitive, so `0D99`/`0B11`/`0XFF`/`0O77` are all valid numbers.
//
// - If a prefix is not provided on the number, decimal is assumed as a default.
//   There is no way to override this default in the macro, but perhaps it could
//   be added in the future.
//
// - When parsing a signed integer, a `-` may be present before the number to
//   negate it. A leading `+` may be present for signed or unsigned numbers,
//
// - Currently, whitespace is not allowed in the value. In the future we may
//   relax this to allow leading and trailing whitespace, and possibly
//   whitespace

#![no_std]

mod parse;

/// Not part of the public API. Please do not use.
#[doc(hidden)]
pub mod __priv {
    // Export stuff we need from the macro.
    pub use core;
    pub use core::option::Option::{self, None, Some};
    pub mod parse_dispatch {
        use crate::parse::{parse_signed, parse_unsigned, ParseError::Empty};

        // unsigned
        pub const fn usize(s: &[u8], default: Option<usize>) -> Option<usize> {
            match parse_unsigned(s, None, Some(usize::MAX as u128)) {
                Ok(v) => Some(v as usize),
                Err(Empty) => default,
                _ => None,
            }
        }
        pub const fn u8(s: &[u8], default: Option<u8>) -> Option<u8> {
            match parse_unsigned(s, None, Some(u8::MAX as u128)) {
                Ok(v) => Some(v as u8),
                Err(Empty) => default,
                _ => None,
            }
        }
        pub const fn u16(s: &[u8], default: Option<u16>) -> Option<u16> {
            match parse_unsigned(s, None, Some(u16::MAX as u128)) {
                Ok(v) => Some(v as u16),
                Err(Empty) => default,
                _ => None,
            }
        }
        pub const fn u32(s: &[u8], default: Option<u32>) -> Option<u32> {
            match parse_unsigned(s, None, Some(u32::MAX as u128)) {
                Ok(v) => Some(v as u32),
                Err(Empty) => default,
                _ => None,
            }
        }
        pub const fn u64(s: &[u8], default: Option<u64>) -> Option<u64> {
            match parse_unsigned(s, None, Some(u64::MAX as u128)) {
                Ok(v) => Some(v as u64),
                Err(Empty) => default,
                _ => None,
            }
        }
        pub const fn u128(s: &[u8], default: Option<u128>) -> Option<u128> {
            match parse_unsigned(s, None, None) {
                Ok(v) => Some(v),
                Err(Empty) => default,
                _ => None,
            }
        }

        // signed
        pub const fn isize(s: &[u8], default: Option<isize>) -> Option<isize> {
            match parse_signed(s, Some(isize::MIN as i128), Some(isize::MAX as i128)) {
                Ok(v) => Some(v as isize),
                Err(Empty) => default,
                _ => None,
            }
        }
        pub const fn i8(s: &[u8], default: Option<i8>) -> Option<i8> {
            match parse_signed(s, Some(i8::MIN as i128), Some(i8::MAX as i128)) {
                Ok(v) => Some(v as i8),
                Err(Empty) => default,
                _ => None,
            }
        }
        pub const fn i16(s: &[u8], default: Option<i16>) -> Option<i16> {
            match parse_signed(s, Some(i16::MIN as i128), Some(i16::MAX as i128)) {
                Ok(v) => Some(v as i16),
                Err(Empty) => default,
                _ => None,
            }
        }
        pub const fn i32(s: &[u8], default: Option<i32>) -> Option<i32> {
            match parse_signed(s, Some(i32::MIN as i128), Some(i32::MAX as i128)) {
                Ok(v) => Some(v as i32),
                Err(Empty) => default,
                _ => None,
            }
        }
        pub const fn i64(s: &[u8], default: Option<i64>) -> Option<i64> {
            match parse_signed(s, Some(i64::MIN as i128), Some(i64::MAX as i128)) {
                Ok(v) => Some(v as i64),
                Err(Empty) => default,
                _ => None,
            }
        }
        pub const fn i128(s: &[u8], default: Option<i128>) -> Option<i128> {
            match parse_signed(s, None, None) {
                Ok(v) => Some(v),
                Err(Empty) => default,
                _ => None,
            }
        }
        // Other things
        pub const fn bool(s: &[u8], default: Option<bool>) -> Option<bool> {
            match crate::parse::parse_bool(s) {
                Ok(v) => Some(v),
                Err(Empty) => default,
                _ => None,
            }
        }
        //
        // pub const fn char(s: &[u8]) -> Option<char> {
        //     crate::parse::parse_char(s)
        // }
    }
}

/// Here's an example
/// ```
/// use envparse::parse_env;
/// const MAX_LEN: usize = parse_env!("MYCRATE_MAX_THING_LEN" as usize else 64);
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
#[macro_export]
macro_rules! parse_env {
    ($var_name:literal as $typ:ident) => {{
        // The way we make this work without traits is we just look inside
        // `__priv::parse_dispatch` for a function with the same name as the
        // type they provided to the macro. Not very extensible, but doesn't
        // require const traits (which feel like they're a jillion years away).
        //
        // Also note that items like `const` don't have hygene, so we pick a
        // name for this intermediate constant that's unlikely to collide with
        // identifiers in the user's program (even if they're wrapping our macro
        // in another macro, they shouldn't use the `__ENVPARSE_` prefix. And if
        // they do, I don't care about breaking them).
        const __ENVPARSE_VALUE: $typ = match $crate::__priv::parse_dispatch::$typ(
            $crate::__priv::core::env!($var_name).as_bytes(),
            $crate::__priv::None,
        ) {
            $crate::__priv::Some(v) => v,
            $crate::__priv::None => {
                $crate::__priv::core::panic!($crate::__priv::core::concat!(
                    "error: the value in ",
                    $crate::__priv::core::stringify!($s),
                    " doesn't parse as a number, or is out of range for `",
                    $crate::__priv::core::stringify!($typ),
                    "`.",
                ));
            }
        };
        __ENVPARSE_VALUE
    }};

    ($var_name:literal as $typ:ident else $default:expr) => {{
        const __ENVPARSE_VALUE: $typ = {
            const __ENVPARSE_DEFAULT: $typ = $default;
            match $crate::__priv::core::option_env!($var_name) {
                $crate::__priv::None => __ENVPARSE_DEFAULT,
                $crate::__priv::Some(s) => {
                    match $crate::__priv::parse_dispatch::$typ(
                        s.as_bytes(),
                        $crate::__priv::Some(__ENVPARSE_DEFAULT),
                    ) {
                        $crate::__priv::Some(v) => v,
                        $crate::__priv::None => {
                            $crate::__priv::core::panic!($crate::__priv::core::concat!(
                                "error: the value in ",
                                $crate::__priv::core::stringify!($s),
                                " doesn't parse as a number, or is out of range for `",
                                $crate::__priv::core::stringify!($typ),
                                "`."
                            ));
                        }
                    }
                }
            }
        };
        __ENVPARSE_VALUE
    }};

    (try $var_name:literal as $typ:ident) => {{
        const __OPTION: $crate::__priv::Option<$typ> = {
            match $crate::__priv::core::option_env!($var_name) {
                $crate::__priv::None => $crate::__priv::None,
                $crate::__priv::Some(s) if s.is_empty() => $crate::__priv::None,
                $crate::__priv::Some(s) => {
                    match $crate::__priv::parse_dispatch::$typ(s.as_bytes(), $crate::__priv::None) {
                        $crate::__priv::Some(v) => v,
                        $crate::__priv::None => {
                            $crate::__priv::core::panic!($crate::__priv::core::concat!(
                                "error: the value in ",
                                $crate::__priv::core::stringify!($s),
                                " doesn't parse as a number, or is out of range for `",
                                $crate::__priv::core::stringify!($typ),
                                "`."
                            ));
                        }
                    }
                }
            }
        };
        __OPTION
    }};
}
