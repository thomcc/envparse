// compile-flags: --error-format=human
// rustc-env:USIZE=12
// rustc-env:ISIZE=-12
// rustc-env:U8MAX=255
// rustc-env:I8MIN=-128
// rustc-env:I8MAX=127
#![crate_type = "bin"]
extern crate envparse;

const USZ: usize = envparse::parse_env!("USIZE" as usize);
const ISZ: isize = envparse::parse_env!("ISIZE" as isize);

const U8MAX: u8 = envparse::parse_env!("U8MAX" as u8);
const I8MIN: i8 = envparse::parse_env!("I8MIN" as i8);
const I8MAX: i8 = envparse::parse_env!("I8MAX" as i8);

fn main() {
    assert_eq!(USZ, 12);
    assert_eq!(ISZ, -12);
    assert_eq!(U8MAX, 255);
    assert_eq!(I8MIN, i8::MIN);
    assert_eq!(I8MAX, i8::MAX);
}
