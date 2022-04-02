// Figure out if there's a good way to use this.
#[non_exhaustive]
#[derive(Clone, Copy, PartialEq)]
pub(crate) enum NumParseError {
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

    /// Got a valid number but it doesn't fit in the range of the type.
    OutOfRange,
}

pub(crate) const fn number_parse(s: &[u8], skip_sign: bool) -> Result<(u128, bool), NumParseError> {
    let (mut pos, end) = match trim_ws(s, 0) {
        Ok(tuple) => tuple,
        Err(e) => return Err(e),
    };
    let neg = match s[pos] {
        b'-' if !skip_sign => return Err(NumParseError::UnexpectedSign),
        c @ b'-' | c @ b'+' => {
            pos += 1;
            c == b'-'
        }
        _ => false,
    };
    if pos == end {
        return Err(NumParseError::NoDigits);
    }
    let radix = if pos + 2 <= end {
        let (radix, len) = match (s[pos], s[pos + 1]) {
            (b'0', b'x') | (b'0', b'X') => (16, 2),
            (b'0', b'd') | (b'0', b'D') => (10, 2),
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
            (b'0'..=b'1', 2) | (b'0'..=b'7', 8) | (b'0'..=b'9', 10) => (d - b'0') as u128,
            (b'a'..=b'f', 16) => (d - b'a') as u128 + 10,
            (b'A'..=b'F', 16) => (d - b'A') as u128 + 10,
            (b'_', _) => continue,
            _ => return Err(NumParseError::InvalidDigit),
        };
        ever_saw_digits = true;
        match accum.checked_mul(radix) {
            None => return Err(NumParseError::InvalidDigit),
            Some(shift) => match shift.checked_add(value) {
                None => return Err(NumParseError::IntOverflow),
                Some(val) => accum = val,
            },
        }
    }
    if ever_saw_digits {
        Ok((accum, neg))
    } else {
        Err(NumParseError::NoDigits)
    }
}

const fn trim_ws(s: &[u8], mut start: usize) -> Result<(usize, usize), NumParseError> {
    if s.is_empty() || s.len() <= start {
        return Err(NumParseError::Empty);
    }
    while start < s.len() && matches!(s[start], b'\t' | b' ' | b'\n' | b'\r') {
        start += 1;
    }
    if start == s.len() {
        return Err(NumParseError::Empty);
    }
    let mut end = s.len() - 1;
    while end > start && matches!(s[end], b'\t' | b' ' | b'\n' | b'\r') {
        end -= 1;
    }
    if end <= start {
        Err(NumParseError::Empty)
    } else {
        Ok((start, end + 1))
    }
}

pub(crate) const fn parse_unsigned(
    s: &[u8],
    incl_min: Option<u128>,
    incl_max: Option<u128>,
) -> Result<u128, NumParseError> {
    let val = match number_parse(s, false) {
        Ok((n, _)) => n,
        Err(e) => return Err(e),
    };
    if matches!(incl_min, Some(min) if val < min) {
        return Err(NumParseError::OutOfRange);
    }
    if matches!(incl_max, Some(max) if val > max) {
        return Err(NumParseError::OutOfRange);
    }
    Ok(val)
}

pub(crate) const fn parse_signed(
    s: &[u8],
    incl_min: Option<i128>,
    incl_max: Option<i128>,
) -> Result<i128, NumParseError> {
    const I128_MIN_MAGNITUDE: u128 = (i128::MAX as u128) + 1;
    let val = match number_parse(s, true) {
        Ok((n, true)) if n == I128_MIN_MAGNITUDE => i128::MIN,
        Ok((n, true)) if n <= (i128::MAX as u128) => -(n as i128),
        Ok((n, false)) if n <= (i128::MAX as u128) => n as i128,
        Ok((_, _)) => return Err(NumParseError::OutOfRange),
        Err(e) => return Err(e),
    };
    if matches!(incl_min, Some(min) if val < min) {
        return Err(NumParseError::OutOfRange);
    }
    if matches!(incl_max, Some(max) if val > max) {
        return Err(NumParseError::OutOfRange);
    }
    Ok(val)
}
