/// The way we make this work without traits is we just look inside
/// `__priv::parsers` for a function with the same name as the type they provided
/// to the macro. Not very extensible, but doesn't require const traits (which
/// feel like they're a jillion years away).

macro_rules! unwrap_or {
    ($o:expr, $or:expr) => {
        match $o {
            Some(inner) => inner,
            None => $or,
        }
    };
}

mod parse_bounded {
    use crate::parse::{parse_signed, parse_unsigned, ParseError::Empty};

    // unsigned
    pub const fn usize(
        s: &[u8],
        default: Option<usize>,
        min: Option<usize>,
        max: Option<usize>,
        clamp: bool,
    ) -> Option<usize> {
        match parse_unsigned(s, unwrap_or!(min, 0) as u128, unwrap_or!(max, usize::MAX) as u128, clamp) {
            Ok(v) => Some(v as usize),
            Err(Empty) => default,
            _ => None,
        }
    }

    pub const fn u8(s: &[u8], default: Option<u8>, min: Option<u8>, max: Option<u8>, clamp: bool) -> Option<u8> {
        match parse_unsigned(s, unwrap_or!(min, 0) as u128, unwrap_or!(max, u8::MAX) as u128, clamp) {
            Ok(v) => Some(v as u8),
            Err(Empty) => default,
            _ => None,
        }
    }

    pub const fn u16(s: &[u8], default: Option<u16>, min: Option<u16>, max: Option<u16>, clamp: bool) -> Option<u16> {
        match parse_unsigned(s, unwrap_or!(min, 0) as u128, unwrap_or!(max, u16::MAX) as u128, clamp) {
            Ok(v) => Some(v as u16),
            Err(Empty) => default,
            _ => None,
        }
    }

    pub const fn u32(s: &[u8], default: Option<u32>, min: Option<u32>, max: Option<u32>, clamp: bool) -> Option<u32> {
        match parse_unsigned(s, unwrap_or!(min, 0) as u128, unwrap_or!(max, u32::MAX) as u128, clamp) {
            Ok(v) => Some(v as u32),
            Err(Empty) => default,
            _ => None,
        }
    }

    pub const fn u64(s: &[u8], default: Option<u64>, min: Option<u64>, max: Option<u64>, clamp: bool) -> Option<u64> {
        match parse_unsigned(s, unwrap_or!(min, 0) as u128, unwrap_or!(max, u64::MAX) as u128, clamp) {
            Ok(v) => Some(v as u64),
            Err(Empty) => default,
            _ => None,
        }
    }

    pub const fn u128(
        s: &[u8],
        default: Option<u128>,
        min: Option<u128>,
        max: Option<u128>,
        clamp: bool,
    ) -> Option<u128> {
        match parse_unsigned(s, unwrap_or!(min, 0), unwrap_or!(max, u128::MAX), clamp) {
            Ok(v) => Some(v),
            Err(Empty) => default,
            _ => None,
        }
    }

    // signed
    pub const fn isize(
        s: &[u8],
        default: Option<isize>,
        min: Option<isize>,
        max: Option<isize>,
        clamp: bool,
    ) -> Option<isize> {
        match parse_signed(s, unwrap_or!(min, isize::MIN) as i128, unwrap_or!(max, isize::MAX) as i128, clamp) {
            Ok(v) => Some(v as isize),
            Err(Empty) => default,
            _ => None,
        }
    }

    pub const fn i8(s: &[u8], default: Option<i8>, min: Option<i8>, max: Option<i8>, clamp: bool) -> Option<i8> {
        match parse_signed(s, unwrap_or!(min, i8::MIN) as i128, unwrap_or!(max, i8::MAX) as i128, clamp) {
            Ok(v) => Some(v as i8),
            Err(Empty) => default,
            _ => None,
        }
    }

    pub const fn i16(s: &[u8], default: Option<i16>, min: Option<i16>, max: Option<i16>, clamp: bool) -> Option<i16> {
        match parse_signed(s, unwrap_or!(min, i16::MIN) as i128, unwrap_or!(max, i16::MAX) as i128, clamp) {
            Ok(v) => Some(v as i16),
            Err(Empty) => default,
            _ => None,
        }
    }

    pub const fn i32(s: &[u8], default: Option<i32>, min: Option<i32>, max: Option<i32>, clamp: bool) -> Option<i32> {
        match parse_signed(s, unwrap_or!(min, i32::MIN) as i128, unwrap_or!(max, i32::MAX) as i128, clamp) {
            Ok(v) => Some(v as i32),
            Err(Empty) => default,
            _ => None,
        }
    }

    pub const fn i64(s: &[u8], default: Option<i64>, min: Option<i64>, max: Option<i64>, clamp: bool) -> Option<i64> {
        match parse_signed(s, unwrap_or!(min, i64::MIN) as i128, unwrap_or!(max, i64::MAX) as i128, clamp) {
            Ok(v) => Some(v as i64),
            Err(Empty) => default,
            _ => None,
        }
    }

    pub const fn i128(
        s: &[u8],
        default: Option<i128>,
        min: Option<i128>,
        max: Option<i128>,
        clamp: bool,
    ) -> Option<i128> {
        match parse_signed(s, unwrap_or!(min, i128::MIN) as i128, unwrap_or!(max, i128::MAX) as i128, clamp) {
            Ok(v) => Some(v as i128),
            Err(Empty) => default,
            _ => None,
        }
    }
}

pub mod parsers {
    use crate::parse::ParseError::Empty;

    // unsigned
    pub const fn usize(s: &[u8], default: Option<usize>) -> Option<usize> {
        super::parse_bounded::usize(s, default, None, None, false)
    }
    pub const fn u8(s: &[u8], default: Option<u8>) -> Option<u8> {
        super::parse_bounded::u8(s, default, None, None, false)
    }
    pub const fn u16(s: &[u8], default: Option<u16>) -> Option<u16> {
        super::parse_bounded::u16(s, default, None, None, false)
    }
    pub const fn u32(s: &[u8], default: Option<u32>) -> Option<u32> {
        super::parse_bounded::u32(s, default, None, None, false)
    }
    pub const fn u64(s: &[u8], default: Option<u64>) -> Option<u64> {
        super::parse_bounded::u64(s, default, None, None, false)
    }
    pub const fn u128(s: &[u8], default: Option<u128>) -> Option<u128> {
        super::parse_bounded::u128(s, default, None, None, false)
    }

    // Signed
    pub const fn isize(s: &[u8], default: Option<isize>) -> Option<isize> {
        super::parse_bounded::isize(s, default, None, None, false)
    }
    pub const fn i8(s: &[u8], default: Option<i8>) -> Option<i8> {
        super::parse_bounded::i8(s, default, None, None, false)
    }
    pub const fn i16(s: &[u8], default: Option<i16>) -> Option<i16> {
        super::parse_bounded::i16(s, default, None, None, false)
    }
    pub const fn i32(s: &[u8], default: Option<i32>) -> Option<i32> {
        super::parse_bounded::i32(s, default, None, None, false)
    }
    pub const fn i64(s: &[u8], default: Option<i64>) -> Option<i64> {
        super::parse_bounded::i64(s, default, None, None, false)
    }
    pub const fn i128(s: &[u8], default: Option<i128>) -> Option<i128> {
        super::parse_bounded::i128(s, default, None, None, false)
    }

    // Other things
    pub const fn bool(s: &[u8], default: Option<bool>) -> Option<bool> {
        match crate::parse::parse_bool(s) {
            Ok(v) => Some(v),
            Err(Empty) => default,
            _ => None,
        }
    }
}
