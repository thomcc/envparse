# `envparse`

Quick and dirty crate for parsing values out of an environment var provided at
compile time.

## Usage
Here's an example
```rs
use envparse::parse_env;
const MAX_LEN: usize = parse_env!("MYCRATE_MAX_THING_LEN" as usize else 64);
struct Thing {
    len: [u8; MAX_LEN],
}
```

You can bound by ranges too. This one will fail because the
`MUST_BE_USER_PROVIDED` var isn't provided.

```rs
use envparse::parse_env;
const MAX_LEN_LOG2: u32 = envparse::parse_env!("MYCRATE_MAX_LEN_LOG2" as u32 in 0..32);
const MAX_LEN: usize = 1 << MAX_LEN_LOG2;
struct Thing {
    len: [u8; MAX_LEN],
}
```

You can also 

```
use envparse::parse_env;
const MAX_LEN_LOG2: u32 = envparse::parse_env!(try "OPTIONAL_MAX_LEN_LOG2" as u32 in 1..32);
const MAX_LEN: usize = 1 << MAX_LEN_LOG2;
struct Thing {
    len: [u8; MAX_LEN],
}
```

```
