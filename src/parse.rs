//! The raw parsers, provided for convenience, since perhaps you need a to use a
//! const-compatible integer (or boolean, I suppose) parser for something other
//! than environment variables.

/// Indicates failure to parse something. Because the parsers are generally
/// parsing integers numbers, that's what these errors focus on. See
/// [`parse_unsigned`] and [`parse_signed`] for more information.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ParseError {
    /// Empty or whitespace-only string.
    Empty,

    /// Got a leading `-` on an unsigned number.
    UnexpectedSign,

    /// Got an invalid digit for the base.
    InvalidDigit,

    /// Got no digits, e.g. a string like `"-"`, `"0x"`, etc.
    NoDigits,

    /// Hit integer overflow when computing.
    IntOverflow,

    /// Got a valid number but it doesn't fit in the range of the type (and
    /// `clamp` wasn't specified as true).
    OutOfRange,

    /// Got something that doesn't seem to indicate a boolean.
    UnknownBoolValue,
}

pub(crate) const fn number_parse(s: &[u8], skip_sign: bool) -> Result<(u128, bool), ParseError> {
    let (mut pos, end) = match trim_ws(s) {
        Some((start, end)) => (start, end),
        None => return Err(ParseError::Empty),
    };
    let neg = match s[pos] {
        b'-' if !skip_sign => return Err(ParseError::UnexpectedSign),
        c @ b'-' | c @ b'+' => {
            pos += 1;
            c == b'-'
        }
        _ => false,
    };
    if pos == end {
        return Err(ParseError::NoDigits);
    }
    let radix = if pos + 2 <= end {
        let (radix, len) = match (s[pos], s[pos + 1]) {
            (b'0', b'x') | (b'0', b'X') => (16, 2),
            // (b'0', b'd') | (b'0', b'D') => (10, 2),
            (b'0', b'o') | (b'0', b'O') => (8, 2),
            (b'0', b'b') | (b'0', b'B') => (2, 2),
            _ => (10, 0),
        };
        pos += len;
        radix
    } else {
        10
    };
    let mut accum = 0u128;
    let mut ever_saw_digits = false;
    while pos < end {
        let d = s[pos];
        pos += 1;
        let value = match (d, radix) {
            (b'0'..=b'1', 2) | (b'0'..=b'7', 8) | (b'0'..=b'9', 10 | 16) => (d - b'0') as u128,
            (b'a'..=b'f', 16) => (d - b'a') as u128 + 10,
            (b'A'..=b'F', 16) => (d - b'A') as u128 + 10,
            (b'_', _) => continue,
            _ => return Err(ParseError::InvalidDigit),
        };
        ever_saw_digits = true;
        match accum.checked_mul(radix) {
            None => return Err(ParseError::IntOverflow),
            Some(shift) => match shift.checked_add(value) {
                None => return Err(ParseError::IntOverflow),
                Some(val) => accum = val,
            },
        }
    }
    if ever_saw_digits {
        Ok((accum, neg))
    } else {
        Err(ParseError::NoDigits)
    }
}

const fn trim_ws(s: &[u8]) -> Option<(usize, usize)> {
    let mut start = 0;
    if s.is_empty() || s.len() <= start {
        return None;
    }
    while start < s.len() && s[start].is_ascii_whitespace() {
        start += 1;
    }
    if start == s.len() {
        return None;
    }
    let mut end = s.len() - 1;
    while end > start && s[end].is_ascii_whitespace() {
        end -= 1;
    }
    end += 1;
    if end <= start {
        None
    } else {
        Some((start, end))
    }
}

/// Parse a `u128` from a byte slice in const.
///
/// Case-insensitive, ignores leading and trailing whitespace, supports internal
/// underscores and prefixed non-base-ten numbers (e.g. `"0xffff_ffff"` and
/// `"0b1111_0011"` will both parse as you would hope).
///
/// See [Syntax](mod@super#syntax) for more info on what strings this function
/// accepts.
pub const fn parse_unsigned(s: &[u8], incl_min: u128, incl_max: u128, clamp: bool) -> Result<u128, ParseError> {
    let val = match number_parse(s, false) {
        Ok((n, _)) => n,
        Err(e) => match e {
            ParseError::IntOverflow if clamp => incl_max,
            ParseError::UnexpectedSign if clamp => incl_min,
            e => return Err(e),
        },
    };
    if val < incl_min {
        return if clamp { Ok(incl_min) } else { Err(ParseError::OutOfRange) };
    }
    if val > incl_max {
        return if clamp { Ok(incl_max) } else { Err(ParseError::OutOfRange) };
    }
    Ok(val)
}

/// Like [`parse_unsigned`] but for signed numbers, returning a `i128`.
///
/// See [Syntax](mod@super#syntax) for information on what strings this
/// function accepts.
pub const fn parse_signed(s: &[u8], incl_min: i128, incl_max: i128, clamp: bool) -> Result<i128, ParseError> {
    const I128_MIN_MAGNITUDE: u128 = (i128::MAX as u128) + 1;
    let val = match number_parse(s, true) {
        Ok((n, true)) if n == I128_MIN_MAGNITUDE => i128::MIN,
        Ok((n, true)) if n <= (i128::MAX as u128) => -(n as i128),
        Ok((_, true)) if clamp => incl_min,
        Ok((n, false)) if n <= (i128::MAX as u128) => n as i128,
        Ok((_, false)) if clamp => incl_max,
        Ok((_, _)) => return Err(ParseError::OutOfRange),
        // Err(ParseError::IntOverflow) =>
        Err(e) => return Err(e),
    };
    if val < incl_min {
        return if clamp { Ok(incl_min) } else { Err(ParseError::OutOfRange) };
    }
    if val > incl_max {
        return if clamp { Ok(incl_max) } else { Err(ParseError::OutOfRange) };
    }
    Ok(val)
}

/// Parses a boolean from a byte slice.
///
/// Case-insensitive, ignores leading and trailing whitespace, and accepts
/// `"0"`, `"f"`, `"n"`, `"no"`, `"off"`, and `"false"` for `false`, and `"1"`,
/// `"t"`, `"y"`, `"on"`, `"yes"`, and `"true"` for `true`.
///
/// See [Syntax](mod@super#syntax) for information on what strings this
/// function accepts.
pub const fn parse_bool(s: &[u8]) -> Result<bool, ParseError> {
    let (i, e) = match trim_ws(s) {
        Some(tup) => tup,
        None => return Err(ParseError::Empty),
    };
    let len = e.saturating_sub(i);
    match len {
        0 => Err(ParseError::Empty),
        // All these are case insensitive.
        //
        // The bool syntax accepted is similar to what `rustc` accepts for `-C`
        // and `-Z` flags, although a few single-char values are allowed ("1" |
        // "t" | "y" for true, and "0" | "n" | "f" for false)
        1 => match s[i] {
            // "1"/"0" | "t"/"f" | "y/n"
            b'1' | b't' | b'T' | b'y' | b'Y' => Ok(true),
            b'0' | b'f' | b'F' | b'n' | b'N' => Ok(false),
            _ => Err(ParseError::UnknownBoolValue),
        },
        2 => match (s[i], s[i + 1]) {
            // "no"
            (b'n' | b'N', b'o' | b'O') => Ok(false),
            // "on"
            (b'o' | b'O', b'n' | b'N') => Ok(true),
            _ => Err(ParseError::UnknownBoolValue),
        },
        3 => match (s[i], s[i + 1], s[i + 2]) {
            // "off"
            (b'o' | b'O', b'f' | b'F', b'f' | b'F') => Ok(false),
            // "yes"
            (b'y' | b'Y', b'e' | b'E', b's' | b'S') => Ok(true),
            _ => Err(ParseError::UnknownBoolValue),
        },
        4 => match (s[i], s[i + 1], s[i + 2], s[i + 3]) {
            // "true"
            (b't' | b'T', b'r' | b'R', b'u' | b'U', b'e' | b'E') => Ok(true),
            _ => Err(ParseError::UnknownBoolValue),
        },
        5 => match (s[i], s[i + 1], s[i + 2], s[i + 3], s[i + 4]) {
            // "false"
            (b'f' | b'F', b'a' | b'A', b'l' | b'L', b's' | b'S', b'e' | b'E') => Ok(false),
            _ => Err(ParseError::UnknownBoolValue),
        },
        _ => Err(ParseError::UnknownBoolValue),
    }
}

#[cfg(test)]
mod test {
    extern crate alloc;
    use super::*;
    use ParseError::*;

    #[test]
    fn test_trim_empty() {
        assert_eq!(trim_ws(b""), None);
        assert_eq!(trim_ws(b" \t\n\r"), None);
        assert_eq!(trim_ws(b" \t\n\r"), None);
        for i in 0..15 {
            for c in [" ", "\t", "\n", "\r"] {
                let s = c.repeat(i);
                assert_eq!(trim_ws(s.as_bytes()), None, "string of {} spaces (type = {:?}): {:?}", s.len(), c, s,);
                for c2 in [" ", "\t", "\n", "\r"] {
                    let cc = alloc::format!("{}{}", c, c2);
                    let s2 = cc.repeat(i);
                    assert_eq!(
                        trim_ws(s.as_bytes()),
                        None,
                        "string of {} spaces (type = {:?}): {:?}",
                        s2.len(),
                        cc,
                        s2,
                    );
                }
            }
        }
    }

    #[test]
    fn test_trim() {
        fn check(s: &str, r: core::ops::Range<usize>) {
            assert_eq!(trim_ws(s.as_bytes()), Some((r.start, r.end)), "trim {:?}", (s, r));
            let (sr, se) = trim_ws(s.as_bytes()).unwrap();
            assert_eq!(s.get(sr..se), Some(s.trim()), "trim smoke {:?}", (s, r, sr..se),);
        }

        for i in 1..10 {
            let s = "a".repeat(i);
            let r = 0..s.len();
            check(&s, r.clone());
            for i in 0..15 {
                for p1 in [" ", "\t", "\r", "\n"] {
                    for p2 in [" ", "\t", "\r", "\n"] {
                        let pad = alloc::format!("{}{}", p1, p2).repeat(i);
                        check(&alloc::format!("{}{}", s, pad), r.clone());
                        let padded_r = r.start + pad.len()..r.end + pad.len();
                        check(&alloc::format!("{}{}", pad, s), padded_r.clone());
                        check(&alloc::format!("{}{}{}", pad, s, pad), padded_r.clone());
                    }
                }
            }
        }
    }

    fn mixcase(s: &str, b: bool) -> alloc::string::String {
        s.chars()
            .enumerate()
            .map(|(i, c)| {
                if (i % 2 == 0) ^ b {
                    c.to_uppercase().collect::<alloc::string::String>()
                } else {
                    c.to_lowercase().collect::<alloc::string::String>()
                }
            })
            .collect()
    }

    #[test]
    fn test_parse_unsigned() {
        #[track_caller]
        fn check(s: &str, res: Result<u128, ParseError>) {
            assert_eq!(parse_unsigned(s.as_ref(), 0, u128::MAX, false), res, "input: {:?}", (s, res),);
        }

        #[track_caller]
        fn ok1(s: &str, v: u128) {
            check(s, Ok(v));
        }

        #[track_caller]
        fn err1(s: &str, e: ParseError) {
            check(s, Err(e))
        }

        #[track_caller]
        fn ok(s: &str, v: u128) {
            ok1(s, v);
            ok1(&s.to_uppercase(), v);
            ok1(&s.to_lowercase(), v);
            ok1(&mixcase(s, true), v);
            ok1(&mixcase(s, false), v);
            ok1(&alloc::format!(" {}", s), v);
            ok1(&alloc::format!("{} ", s), v);
            ok1(&alloc::format!(" {} ", s), v);
            ok1(&alloc::format!("{}    ", s), v);
            ok1(&alloc::format!("    {}", s), v);
            ok1(&alloc::format!("    {}    ", s), v);
            if !s.contains("+") {
                ok1(&alloc::format!("+{}", s), v);
                ok1(&alloc::format!("+{} ", s), v);
                ok1(&alloc::format!(" +{}", s), v);
                ok1(&alloc::format!(" +{} ", s), v);
                ok1(&alloc::format!("+{}    ", s), v);
                ok1(&alloc::format!("    +{}", s), v);
                ok1(&alloc::format!("    +{}    ", s), v);
            }
        }

        #[track_caller]
        fn err(s: &str, e: ParseError) {
            err1(s, e);
            err1(&s.to_uppercase(), e);
            err1(&s.to_lowercase(), e);
            err1(&alloc::format!("{} ", s), e);
            err1(&alloc::format!(" {}", s), e);
            err1(&alloc::format!(" {} ", s), e);
            err1(&alloc::format!("{}    ", s), e);
            err1(&alloc::format!("    {}", s), e);
            err1(&alloc::format!("    {}    ", s), e);
            if !s.contains("+") && !s.contains("-") && !s.is_empty() {
                err1(&alloc::format!("+{}", s), e);
                err1(&alloc::format!("+{} ", s), e);
                err1(&alloc::format!(" +{}", s), e);
                err1(&alloc::format!(" +{} ", s), e);
                err1(&alloc::format!("+{}    ", s), e);
                err1(&alloc::format!("    +{}", s), e);
                err1(&alloc::format!("    +{}    ", s), e);
            }
        }

        ok("0x1234abcd", 0x1234abcd);
        ok("0x__12_34__a__b__c__d__", 0x1234abcd);

        ok("1234567890", 1234567890);
        ok("0o12345670", 0o12345670);
        ok("0b101010", 0b101010);
        ok("0xabcdef0123456789", 0xabcdef0123456789);

        ok("0o3777777777777777777777777777777777777777777", u128::MAX);
        ok("0xffffffffffffffffffffffffffffffff", u128::MAX);
        ok("0Xffffffffffffffffffffffffffffffff", u128::MAX);

        ok("340282366920938463463374607431768211455", u128::MAX);
        ok("0o3777777777777777777777777777777777777777777", u128::MAX);
        ok("0xffffffffffffffffffffffffffffffff", u128::MAX);
        ok("0Xffffffffffffffffffffffffffffffff", u128::MAX);
        ok(
            "0b11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111",
            u128::MAX,
        );

        ok("0xffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff", u128::MAX);
        ok("0xffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff___", u128::MAX);

        ok("0x__f_f_f_f_ffff_ffff_ffff_ffff_ffff_ffff_ffff___", u128::MAX);
        err("", Empty);
        err("-30", UnexpectedSign);
        err("-", UnexpectedSign);
        err("", Empty);
        err("0x1234g", InvalidDigit);
        err("0xg1234", InvalidDigit);
        err("0x1234g1234", InvalidDigit);

        err("123a", InvalidDigit);
        err("a123", InvalidDigit);
        err("123a123", InvalidDigit);

        err("0o8", InvalidDigit);
        err("0o128", InvalidDigit);
        err("0o12812", InvalidDigit);
        err("0o12a12", InvalidDigit);

        err("0b2", InvalidDigit);
        err("0b00200", InvalidDigit);
        err("0b002", InvalidDigit);
        err("0b200", InvalidDigit);
        err("0b121", InvalidDigit);
        err("0b12", InvalidDigit);
        err("0b21", InvalidDigit);

        err("0o", NoDigits);
        err("0o_", NoDigits);
        err("0o__", NoDigits);
        err("0x", NoDigits);
        err("0x_", NoDigits);
        err("0x___", NoDigits);
        err("0b", NoDigits);
        err("0b_", NoDigits);
        err("0b__", NoDigits);
        err("_0b", InvalidDigit);
        err("0o4000000000000000000000000000000000000000000", IntOverflow);

        err("0xffffffffffffffffffffffffffffffff0", IntOverflow);
        err("0xf0fffffffffffffffffffffffffffffff0", IntOverflow);

        assert_eq!(parse_unsigned(b"200", 100, 1000, false), Ok(200));

        assert_eq!(parse_unsigned(b"1000", 100, 1000, false), Ok(1000));
        assert_eq!(parse_unsigned(b"100", 100, 1000, false), Ok(100));

        assert_eq!(parse_unsigned(b"500", 600, 700, false), Err(OutOfRange),);

        assert_eq!(parse_unsigned(b"500", 200, 300, false), Err(OutOfRange),);

        assert_eq!(parse_unsigned(b"500", 600, 700, true), Ok(600));
        assert_eq!(parse_unsigned(b"500", 200, 300, true), Ok(300));
        assert_eq!(parse_unsigned(b"250", 200, 300, true), Ok(250));
        assert_eq!(parse_unsigned(b"500", 200, u128::MAX, true), Ok(500));
        assert_eq!(parse_unsigned(b"250", 0, 300, true), Ok(250));

        assert_eq!(parse_unsigned(b"0", 0, 200, true), Ok(0));
        assert_eq!(parse_unsigned(b"-1", 0, 200, true), Ok(0));
        assert_eq!(parse_unsigned(b"0", 1, 255, true), Ok(1));
        assert_eq!(parse_unsigned(b"-1", 1, 255, true), Ok(1));
        assert_eq!(parse_unsigned(b"0", 0, u128::MAX, true), Ok(0));
        assert_eq!(parse_unsigned(b"-1", 0, u128::MAX, true), Ok(0));

        assert_eq!(parse_unsigned(b"-1", 0, u128::MAX, true), Ok(0));

        assert_eq!(parse_unsigned(b"1000", 1, 255, false), Err(OutOfRange));
        assert_eq!(parse_unsigned(b"1000", 1, 255, true), Ok(255));

        assert_eq!(
            parse_unsigned(b"0o4000000000000000000000000000000000000000000", 0, u128::MAX, true,),
            Ok(u128::MAX),
        );
        assert_eq!(parse_unsigned(b"0xffffffffffffffffffffffffffffffff0", 0, u128::MAX, true), Ok(u128::MAX),);
        assert_eq!(parse_unsigned(b"0xf0fffffffffffffffffffffffffffffff0", 0, u128::MAX, true), Ok(u128::MAX),);
        assert_eq!(parse_unsigned(b"0o4000000000000000000000000000000000000000000", 0, 50, true,), Ok(50),);
        assert_eq!(parse_unsigned(b"0xffffffffffffffffffffffffffffffff0", 0, 50, true), Ok(50),);
        assert_eq!(parse_unsigned(b"0xf0fffffffffffffffffffffffffffffff0", 0, 50, true,), Ok(50),);
    }

    #[test]
    fn test_parse_signed() {
        #[track_caller]
        fn check(s: &str, res: Result<i128, ParseError>) {
            assert_eq!(parse_signed(s.as_ref(), i128::MIN, i128::MAX, false), res, "input: {:?}", (s, res),);
        }

        #[track_caller]
        fn ok1(s: &str, v: i128) {
            check(s, Ok(v));
        }

        #[track_caller]
        fn err1(s: &str, e: ParseError) {
            check(s, Err(e))
        }

        #[track_caller]
        fn ok(s: &str, v: i128) {
            ok1(s, v);
            ok1(&s.to_uppercase(), v);
            ok1(&s.to_lowercase(), v);
            ok1(&mixcase(s, true), v);
            ok1(&mixcase(s, false), v);
            ok1(&alloc::format!(" {}", s), v);
            ok1(&alloc::format!("{} ", s), v);
            ok1(&alloc::format!(" {} ", s), v);
            ok1(&alloc::format!("{}    ", s), v);
            ok1(&alloc::format!("    {}", s), v);
            ok1(&alloc::format!("    {}    ", s), v);
            if !s.contains("+") && !s.contains("-") {
                assert!(v >= 0, "bug in test {:?}", (s, v));
                ok1(&alloc::format!("+{}", s), v);
                ok1(&alloc::format!("+{} ", s), v);
                ok1(&alloc::format!(" +{}", s), v);
                ok1(&alloc::format!(" +{} ", s), v);
                ok1(&alloc::format!("+{}    ", s), v);
                ok1(&alloc::format!("    +{}", s), v);
                ok1(&alloc::format!("    +{}    ", s), v);
                if let Some(n) = v.checked_neg() {
                    ok1(&alloc::format!("-{} ", s), n);
                    ok1(&alloc::format!("-{} ", s), n);
                    ok1(&alloc::format!(" -{}", s), n);
                    ok1(&alloc::format!(" -{} ", s), n);
                    ok1(&alloc::format!("-{}    ", s), n);
                    ok1(&alloc::format!("    -{}", s), n);
                    ok1(&alloc::format!("    -{}    ", s), n);
                }
            }
        }

        #[track_caller]
        fn err(s: &str, e: ParseError) {
            err1(s, e);
            err1(&s.to_uppercase(), e);
            err1(&s.to_lowercase(), e);
            err1(&alloc::format!("{} ", s), e);
            err1(&alloc::format!(" {}", s), e);
            err1(&alloc::format!(" {} ", s), e);
            err1(&alloc::format!("{}    ", s), e);
            err1(&alloc::format!("    {}", s), e);
            err1(&alloc::format!("    {}    ", s), e);
            if !s.contains("+") && !s.contains("-") && !s.is_empty() {
                err1(&alloc::format!("+{} ", s), e);
                err1(&alloc::format!(" +{}", s), e);
                err1(&alloc::format!(" +{} ", s), e);
                err1(&alloc::format!("+{}    ", s), e);
                err1(&alloc::format!("    +{}", s), e);
                err1(&alloc::format!("    +{}    ", s), e);
            }
        }

        ok("0", 0);
        ok("1", 1);
        ok("100", 100);
        ok("0o0", 0);
        ok("0o__0__", 0);
        ok("0x1234abcd", 0x1234abcd);
        ok("0x__12_34__a__b__c__d__", 0x1234abcd);

        ok("170141183460469231731687303715884105727", i128::MAX);
        ok("-170141183460469231731687303715884105728", i128::MIN);

        ok("0x7fffffffffffffffffffffffffffffff", i128::MAX);
        ok("-0x80000000000000000000000000000000", i128::MIN);

        ok(
            "0b1111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111",
            i128::MAX,
        );
        ok(
            "-0b10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
            i128::MIN,
        );

        ok("0o1777777777777777777777777777777777777777777", i128::MAX);
        ok("-0o2000000000000000000000000000000000000000000", i128::MIN);

        ok("170141183460469231731687303715884105726", i128::MAX - 1);
        ok("-170141183460469231731687303715884105727", i128::MIN + 1);

        ok("-1234567890", -1234567890);
        ok("-0o12345670", -0o12345670);
        ok("-0b101010", -0b101010);
        ok("-0xabcdef0123456789", -0xabcdef0123456789);

        err("170141183460469231731687303715884105728", OutOfRange);
        err("1701411834604692317316873037158841057270", IntOverflow);
        err("", Empty);
        err("-", NoDigits);
        err("0x1234z", InvalidDigit);
        err("123f", InvalidDigit);

        err("0x1234z", InvalidDigit);

        err("123f", InvalidDigit);

        err("0o", NoDigits);
        err("0o_", NoDigits);
        err("0o__", NoDigits);
        err("0x", NoDigits);
        err("0x_", NoDigits);
        err("0x___", NoDigits);
        err("0b", NoDigits);
        err("0b_", NoDigits);
        err("0b__", NoDigits);
        err("_0b", InvalidDigit);
        err("0o4000000000000000000000000000000000000000000", IntOverflow);
        err("0x7fffffffffffffffffffffffffffffff0", IntOverflow);
        err("0x70fffffffffffffffffffffffffffffff0", IntOverflow);

        err("0o3777777777777777777777777777777777777777777", OutOfRange);
        err("0xffffffffffffffffffffffffffffffff", OutOfRange);
        err(
            "0b11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111",
            OutOfRange,
        );

        err("170141183460469231731687303715884105728", OutOfRange);
        err("-170141183460469231731687303715884105729", OutOfRange);

        assert_eq!(parse_signed(b"0", -1000, 1000, false), Ok(0),);
        assert_eq!(parse_signed(b"1000", -1000, 1000, false), Ok(1000),);
        assert_eq!(parse_signed(b"-1000", -1000, 1000, false), Ok(-1000),);
        assert_eq!(parse_signed(b"1000", i128::MIN, 999, false), Err(OutOfRange),);
        assert_eq!(parse_signed(b"1000", 999, i128::MAX, true), Ok(1000),);
        assert_eq!(parse_signed(b"1000", i128::MIN, 999, true), Ok(999),);
        assert_eq!(parse_signed(b"-1000", -999, i128::MAX, false), Err(OutOfRange),);
        assert_eq!(parse_signed(b"-1000", -999, i128::MAX, true), Ok(-999),);

        assert_eq!(parse_signed(b"-1001", -1000, 1000, false), Err(OutOfRange),);
        assert_eq!(parse_signed(b"1001", -1000, 1000, false), Err(OutOfRange),);
        assert_eq!(parse_signed(b"-1001", -1000, 1000, true), Ok(-1000));
        assert_eq!(parse_signed(b"1001", -1000, 1000, true), Ok(1000));
        assert_eq!(parse_signed(b"1001", -1000, 1000, true), Ok(1000));

        assert_eq!(parse_signed(b"500", 600, 700, true), Ok(600));
        assert_eq!(parse_signed(b"500", 200, 300, true), Ok(300));
        assert_eq!(parse_signed(b"500", 200, i128::MAX, true), Ok(500));

        assert_eq!(parse_signed(b"250", 200, 300, true), Ok(250));
        assert_eq!(parse_signed(b"500", 200, i128::MAX, true), Ok(500));
        assert_eq!(parse_signed(b"250", i128::MIN, 300, true), Ok(250));

        assert_eq!(parse_signed(b"150", -300, 200, true), Ok(150));
        assert_eq!(parse_signed(b"-150", -300, 200, true), Ok(-150));
        assert_eq!(parse_signed(b"300", -300, 200, true), Ok(200));
        assert_eq!(parse_signed(b"-400", -300, 200, true), Ok(-300));

        assert_eq!(parse_signed(b"-250", -300, -200, true), Ok(-250));
        assert_eq!(parse_signed(b"-500", -200, i128::MAX, true), Ok(-200));
        assert_eq!(parse_signed(b"-250", -300, i128::MAX, true), Ok(-250));

        assert_eq!(parse_signed(b"0", i128::MIN, 200, true), Ok(0));
        assert_eq!(parse_signed(b"-1", i128::MIN, 200, true), Ok(-1));
        assert_eq!(parse_signed(b"0", 1, 255, true), Ok(1));
        assert_eq!(parse_signed(b"-1", 1, 255, true), Ok(1));

        assert_eq!(parse_signed(b"0", i128::MIN, i128::MAX, true), Ok(0));
        assert_eq!(parse_signed(b"-1", i128::MIN, i128::MAX, true), Ok(-1));
        assert_eq!(parse_signed(b"1", i128::MIN, i128::MAX, true), Ok(1));

        assert_eq!(parse_signed(b"1000", 1, 255, false), Err(OutOfRange));
        assert_eq!(parse_signed(b"-1000", 1, 255, false), Err(OutOfRange));
        assert_eq!(parse_signed(b"-1000", -255, 255, false), Err(OutOfRange));

        assert_eq!(parse_signed(b"1000", 1, 255, true), Ok(255));
        assert_eq!(parse_signed(b"-1000", 1, 255, true), Ok(1));
        assert_eq!(parse_signed(b"-1000", -255, 255, true), Ok(-255));

        assert_eq!(parse_signed(b"1000", 1, 255, true), Ok(255));
        assert_eq!(parse_signed(b"1000", -255, -1, true), Ok(-1));
        assert_eq!(parse_signed(b"-1000", -255, -1, true), Ok(-255));

        assert_eq!(
            parse_signed(b"0o3777777777777777777777777777777777777777777", i128::MIN, i128::MAX, true),
            Ok(i128::MAX)
        );
        assert_eq!(parse_signed(b"0o3777777777777777777777777777777777777777777", i128::MIN, 30, true), Ok(30));
        assert_eq!(parse_signed(b"0o3777777777777777777777777777777777777777777", i128::MIN, -30, true), Ok(-30));

        assert_eq!(parse_signed(b"0xffffffffffffffffffffffffffffffff", i128::MIN, i128::MAX, true), Ok(i128::MAX));
        assert_eq!(parse_signed(b"0xffffffffffffffffffffffffffffffff", i128::MIN, 30, true), Ok(30));
        assert_eq!(parse_signed(b"0xffffffffffffffffffffffffffffffff", i128::MIN, -30, true), Ok(-30));

        assert_eq!(parse_signed(b"0b11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111", i128::MIN, i128::MAX, true), Ok(i128::MAX));
        assert_eq!(parse_signed(b"0b11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111", i128::MIN, 30, true), Ok(30));
        assert_eq!(parse_signed(b"0b11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111", i128::MIN, -30, true), Ok(-30));

        assert_eq!(parse_signed(b"170141183460469231731687303715884105728", i128::MIN, i128::MAX, true), Ok(i128::MAX));
        assert_eq!(parse_signed(b"170141183460469231731687303715884105728", i128::MIN, 30, true), Ok(30));
        assert_eq!(parse_signed(b"170141183460469231731687303715884105728", i128::MIN, -30, true), Ok(-30));

        assert_eq!(parse_signed(b"170141183460469231731687303715884105728", 10, i128::MAX, true), Ok(i128::MAX));
        assert_eq!(parse_signed(b"170141183460469231731687303715884105728", 10, 30, true), Ok(30));
        assert_eq!(parse_signed(b"170141183460469231731687303715884105728", -30, 10, true), Ok(10));

        assert_eq!(
            parse_signed(b"-170141183460469231731687303715884105729", i128::MIN, i128::MAX, true),
            Ok(i128::MIN)
        );
        assert_eq!(parse_signed(b"-170141183460469231731687303715884105729", i128::MIN, 30, true), Ok(i128::MIN));
        assert_eq!(parse_signed(b"-170141183460469231731687303715884105729", i128::MIN, -30, true), Ok(i128::MIN));

        assert_eq!(parse_signed(b"-170141183460469231731687303715884105729", i128::MIN, 10, true), Ok(i128::MIN));
        assert_eq!(parse_signed(b"-170141183460469231731687303715884105729", 30, i128::MAX, true), Ok(30));
        assert_eq!(parse_signed(b"-170141183460469231731687303715884105729", -30, i128::MAX, true), Ok(-30));
    }

    #[test]
    fn test_parse_bool() {
        #[track_caller]
        fn check(s: &str, res: Result<bool, ParseError>) {
            assert_eq!(parse_bool(s.as_ref()), res, "input: {:?}", (s, res),);
        }
        #[track_caller]
        fn ok(s: &str, res: bool) {
            check(s, Ok(res));
            check(&alloc::format!("{} ", s), Ok(res));
            check(&alloc::format!(" {}", s), Ok(res));
            check(&alloc::format!(" {} ", s), Ok(res));

            check(&s.to_lowercase(), Ok(res));
            check(&s.to_uppercase(), Ok(res));
            check(&mixcase(s, true), Ok(res));
            check(&mixcase(s, false), Ok(res));
        }

        #[track_caller]
        fn err(s: &str, e: ParseError) {
            check(s, Err(e));
            check(&alloc::format!("{} ", s), Err(e));
            check(&alloc::format!(" {}", s), Err(e));
            check(&alloc::format!(" {} ", s), Err(e));
            check(&s.to_lowercase(), Err(e));
            check(&s.to_uppercase(), Err(e));
            check(&mixcase(s, true), Err(e));
            check(&mixcase(s, false), Err(e));
        }

        ok("t", true);
        ok("f", false);
        ok("y", true);
        ok("n", false);
        ok("1", true);
        ok("0", false);

        ok("true", true);
        ok("false", false);

        ok("on", true);
        ok("off", false);

        ok("yes", true);
        ok("no", false);

        err("", Empty);
        err("foo", UnknownBoolValue);

        err("x", UnknownBoolValue);
        err("01", UnknownBoolValue);
        err("abc", UnknownBoolValue);
        err("defg", UnknownBoolValue);
        err("true1", UnknownBoolValue);
        err("0true1", UnknownBoolValue);
    }
}
