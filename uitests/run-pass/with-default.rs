// compile-flags: --error-format=human
#![crate_type = "bin"]
extern crate envparse;

const A: usize = envparse::parse_env!("USIZE" as usize else 32);

const B: u8 = envparse::parse_env!("SMALLER" as u8 else 40);

fn main() {
    assert_eq!(A, 32);
    assert_eq!(B, 40);
}
