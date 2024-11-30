// error-pattern: doesn't parse as a
// rustc-env:BAD_BOOL=yesss
#![crate_type = "lib"]
extern crate envparse;

pub const BAD_BOOL: bool = envparse::parse_env!("BAD_BOOL" as bool);
