// compile-flags: --error-format=human
// rustc-env:TRUE0=t
// rustc-env:TRUE1=Y
// rustc-env:TRUE2=on
// rustc-env:TRUE3=yes
// rustc-env:TRUE4=true
// rustc-env:FALSE0=f
// rustc-env:FALSE1=n
// rustc-env:FALSE2=no
// rustc-env:FALSE3=off
// rustc-env:FALSE4=false
#![crate_type = "bin"]
extern crate envparse;

const TRUE0: bool = envparse::parse_env!("TRUE0" as bool);
const TRUE1: bool = envparse::parse_env!("TRUE1" as bool);
const TRUE2: bool = envparse::parse_env!("TRUE2" as bool);
const TRUE3: bool = envparse::parse_env!("TRUE3" as bool);
const TRUE4: bool = envparse::parse_env!("TRUE4" as bool);

const FALSE0: bool = envparse::parse_env!("FALSE0" as bool);
const FALSE1: bool = envparse::parse_env!("FALSE1" as bool);
const FALSE2: bool = envparse::parse_env!("FALSE2" as bool);
const FALSE3: bool = envparse::parse_env!("FALSE3" as bool);
const FALSE4: bool = envparse::parse_env!("FALSE4" as bool);

fn main() {
    assert_eq!(TRUE0, true);
    assert_eq!(TRUE1, true);
    assert_eq!(TRUE2, true);
    assert_eq!(TRUE3, true);
    assert_eq!(TRUE4, true);
    assert_eq!(FALSE0, false);
    assert_eq!(FALSE1, false);
    assert_eq!(FALSE2, false);
    assert_eq!(FALSE3, false);
    assert_eq!(FALSE4, false);
}
