//! A crate which allows parsing environment variables defined at compile time
//! into constants.
//!
//! ```
//! // parse `MYCRATE_MAX_THING_LEN` from the environment, defaulting to 4 if not provided.
//! const MAX_THING_LEN: usize = envparse::parse_env!("MYCRATE_MAX_THING_LEN" as usize = 4);
//! struct Thing {
//!     len: [u8; MAX_THING_LEN],
//! }
//! ```
//!
//! Or:
//! ```ignore
//! use envparse::parse_env;
//! // parse `MYCRATE_MAX_THING_LEN` from the environment, defaulting to 4 if not provided.
//! const MAX_THING_BITS: usize = parse_env!(
//!     "MYCRATE_MAX_THING_LEN" as usize,
//!     options(default = 5, min = 2, max = 30, clamp),
//! );
//! struct Thing {
//!     len: [u8; MAX_LEN],
//! }
//! ```
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
//! Unfortunately, `const fn` is very limited in Rust.
//!
//! [^1]: Or `/D` with MSVC — you get the idea.
//!
//! # Supported types (and syntax)
//!
//! Currently, the following types are supported:
//!
//! ## Primitive integers
//!
//! The primitive integer types are all supported: `i8`, `u8`, `i16`, `u16`,
//! `i32`, `u32, `i64`, `u64`, `i128`, `u128`, `isize` and `usize`.
//!
//! The syntax accepted is a superset of the Rust lexical syntax for numbers:
//!
//! - Optional sign, either
//!
//!
//! - Hexadecimal `0x1234_abcd` is suppo
//!
//! - Booleans, where we're fairly flexible in what we accept, in order to be
//!   compatible with common methods for configuring programs via the
//!   environment.
//!
//!     - The following (case-insensitive) strings are parsed as false: `0`,
//!       `false`, `f`, `off`, `no`, and `n`.
//!     - The following (case-insensitive) strings are parsed as false: `1`,
//!       `true`, `t`, `on`, `yes`, and `y`.
//!
//! -
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
//! First, a default value can be provide
//!
//! ```
//! const MAX_LEN: usize = envparse::parse_env!("MYCRATE_MAX_THING_LEN" as usize else 32);
//!
//! struct Thing {
//!     len: [u8; MAX_LEN],
//! }
//! ```
//!
//! Here's one that deliberately fails, his is essentially
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
mod privat;

#[doc(hidden)]
pub mod __priv {
    // Export stuff we need from the macro.
    pub use core;
    pub use core::option::Option::{self, None, Some};

    // pub use crate::privat::parse_bounded;
    pub use crate::privat::parsers;
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
    // note that identifiers at item scope (like `const`s) don't have hygene, so
    // we pick a name for this intermediate constant that's unlikely to collide
    // with identifiers in the user's program (even if they're wrapping our
    // macro in another macro, they shouldn't use the `__ENVPARSE_` prefix. And
    // if they do, I don't care about breaking them).
    ($var_name:literal as $typ:ident) => {{
        const __ENVPARSE_VALUE: $typ =
            match $crate::__priv::parsers::$typ($crate::__priv::core::env!($var_name).as_bytes(), $crate::__priv::None)
            {
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
                    match $crate::__priv::parsers::$typ(s.as_bytes(), $crate::__priv::Some(__ENVPARSE_DEFAULT)) {
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
                $crate::__priv::Some(s) => match $crate::__priv::parsers::$typ(s.as_bytes(), $crate::__priv::None) {
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
                },
            }
        };
        __OPTION
    }};
}
