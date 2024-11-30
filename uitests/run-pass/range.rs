// compile-flags: --error-format=human
// rustc-env:THIRTY_TWO=32
// rustc-env:NEGATIVE_SIXTY=-60
#![crate_type = "bin"]
extern crate envparse;

#[rustfmt::skip]
macro_rules! def_check {
    ($module:ident, $tsigned:ident, $tunsigned: ident) => {
        pub mod $module {
            const THIRTY2_1: $tunsigned = envparse::parse_env!("THIRTY_TWO" as $tunsigned);
            const THIRTY2_2: $tunsigned = envparse::parse_env!("THIRTY_TWO" as $tunsigned in 0..33);
            const THIRTY2_3: $tunsigned = envparse::parse_env!("THIRTY_TWO" as $tunsigned in 0..=32);
            const THIRTY2_4: $tunsigned = envparse::parse_env!("THIRTY_TWO" as $tunsigned in ..=32);
            const THIRTY2_5: $tunsigned = envparse::parse_env!("THIRTY_TWO" as $tunsigned in ..33);
            const THIRTY2_6: $tunsigned = envparse::parse_env!("THIRTY_TWO" as $tunsigned in ..);
            const THIRTY2_7: $tunsigned = envparse::parse_env!("THIRTY_TWO" as $tunsigned in 30..);

            const NEG_SIXTY_1: $tsigned = envparse::parse_env!("NEGATIVE_SIXTY" as $tsigned);
            const NEG_SIXTY_2: $tsigned = envparse::parse_env!("NEGATIVE_SIXTY" as $tsigned in -60..0);
            const NEG_SIXTY_3: $tsigned = envparse::parse_env!("NEGATIVE_SIXTY" as $tsigned in -60..=0);
            const NEG_SIXTY_4: $tsigned = envparse::parse_env!("NEGATIVE_SIXTY" as $tsigned in -60..-59);
            const NEG_SIXTY_5: $tsigned = envparse::parse_env!("NEGATIVE_SIXTY" as $tsigned in ..);
            const NEG_SIXTY_6: $tsigned = envparse::parse_env!("NEGATIVE_SIXTY" as $tsigned in -60..);

            const TRY_THIRTY2_1: Option<$tunsigned> = envparse::parse_env!(try "THIRTY_TWO" as $tunsigned);
            const TRY_THIRTY2_2: Option<$tunsigned> = envparse::parse_env!(try "THIRTY_TWO" as $tunsigned in 0..33);
            const TRY_THIRTY2_3: Option<$tunsigned> = envparse::parse_env!(try "THIRTY_TWO" as $tunsigned in 0..=32);
            const TRY_THIRTY2_4: Option<$tunsigned> = envparse::parse_env!(try "THIRTY_TWO" as $tunsigned in ..=32);
            const TRY_THIRTY2_5: Option<$tunsigned> = envparse::parse_env!(try "THIRTY_TWO" as $tunsigned in ..33);
            const TRY_THIRTY2_6: Option<$tunsigned> = envparse::parse_env!(try "THIRTY_TWO" as $tunsigned in ..);
            const TRY_THIRTY2_7: Option<$tunsigned> = envparse::parse_env!(try "THIRTY_TWO" as $tunsigned in 30..);

            const THIRTY2_DEF_1: $tunsigned = envparse::parse_env!("THIRTY_TWO" as $tunsigned else 42);
            const THIRTY2_DEF_2: $tunsigned = envparse::parse_env!("THIRTY_TWO" as $tunsigned (in 0..33) else 42);
            const THIRTY2_DEF_3: $tunsigned = envparse::parse_env!("THIRTY_TWO" as $tunsigned (in 0..=32) else 42);
            const THIRTY2_DEF_4: $tunsigned = envparse::parse_env!("THIRTY_TWO" as $tunsigned (in ..=32) else 42);
            const THIRTY2_DEF_5: $tunsigned = envparse::parse_env!("THIRTY_TWO" as $tunsigned (in ..33) else 42);
            const THIRTY2_DEF_6: $tunsigned = envparse::parse_env!("THIRTY_TWO" as $tunsigned (in ..) else 42);
            const THIRTY2_DEF_7: $tunsigned = envparse::parse_env!("THIRTY_TWO" as $tunsigned (in 30..) else 42);

            const MISSING_1: $tunsigned = envparse::parse_env!("MISSING" as $tunsigned else 42);
            const MISSING_2: $tunsigned = envparse::parse_env!("MISSING" as $tunsigned (in 0..33) else 42);
            const MISSING_3: $tunsigned = envparse::parse_env!("MISSING" as $tunsigned (in 0..=32) else 42);
            const MISSING_4: $tunsigned = envparse::parse_env!("MISSING" as $tunsigned (in ..=32) else 42);
            const MISSING_5: $tunsigned = envparse::parse_env!("MISSING" as $tunsigned (in ..33) else 42);
            const MISSING_6: $tunsigned = envparse::parse_env!("MISSING" as $tunsigned (in ..) else 42);
            const MISSING_7: $tunsigned = envparse::parse_env!("MISSING" as $tunsigned (in 30..) else 42);

            pub fn check() {
                assert_eq!(THIRTY2_1, 32);
                assert_eq!(THIRTY2_2, 32);
                assert_eq!(THIRTY2_3, 32);
                assert_eq!(THIRTY2_4, 32);
                assert_eq!(THIRTY2_5, 32);
                assert_eq!(THIRTY2_6, 32);
                assert_eq!(THIRTY2_7, 32);

                assert_eq!(NEG_SIXTY_1, -60);
                assert_eq!(NEG_SIXTY_2, -60);
                assert_eq!(NEG_SIXTY_3, -60);
                assert_eq!(NEG_SIXTY_4, -60);
                assert_eq!(NEG_SIXTY_5, -60);
                assert_eq!(NEG_SIXTY_6, -60);

                assert_eq!(THIRTY2_DEF_1, 32);
                assert_eq!(THIRTY2_DEF_2, 32);
                assert_eq!(THIRTY2_DEF_3, 32);
                assert_eq!(THIRTY2_DEF_4, 32);
                assert_eq!(THIRTY2_DEF_5, 32);
                assert_eq!(THIRTY2_DEF_6, 32);
                assert_eq!(THIRTY2_DEF_7, 32);

                assert_eq!(TRY_THIRTY2_1, Some(32));
                assert_eq!(TRY_THIRTY2_2, Some(32));
                assert_eq!(TRY_THIRTY2_3, Some(32));
                assert_eq!(TRY_THIRTY2_4, Some(32));
                assert_eq!(TRY_THIRTY2_5, Some(32));
                assert_eq!(TRY_THIRTY2_6, Some(32));
                assert_eq!(TRY_THIRTY2_7, Some(32));

                assert_eq!(MISSING_1, 42);
                assert_eq!(MISSING_2, 42);
                assert_eq!(MISSING_3, 42);
                assert_eq!(MISSING_4, 42);
                assert_eq!(MISSING_5, 42);
                assert_eq!(MISSING_6, 42);
                assert_eq!(MISSING_7, 42);
            }
        }
    };
}

def_check!(n8, i8, u8);
def_check!(n16, i16, u16);
def_check!(n32, i32, u32);
def_check!(n64, i64, u64);
def_check!(n128, i128, u128);
def_check!(nsize, isize, usize);

fn main() {
    n8::check();
    n16::check();
    n32::check();
    n64::check();
    n128::check();
    nsize::check();
}
