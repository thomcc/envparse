# `envparse`

Quick and dirty crate for parsing values out of an environment var provided at
compile time. See also: [docs](https://docs.rs/envparse).

Feel free to [email me](chiovolonit@gmail.com) if I fail to notice a PR or issue in this
repository (my github notifications are a mess).

## Usage
Here's an example
```rs
const MAX_LEN: usize = envparse::parse_env!("MYCRATE_MAX_THING_LEN" as usize else 64);
struct Thing {
    len: [u8; MAX_LEN],
}
```

You can bound by ranges too. This one will fail because the
`MUST_BE_USER_PROVIDED` var isn't provided.

```rs
const MAX_LEN_LOG2: u32 = envparse::parse_env!("MYCRATE_MAX_LEN_LOG2" as u32 in 0..32);
const MAX_LEN: usize = 1 << MAX_LEN_LOG2;
struct Thing {
    len: [u8; MAX_LEN],
}
```

You can also `try` to parse the value, which always returns an Option. Note that if the user
provided the value, but it was out of range, this will still fail with a compile error (rather
than returning `None`), in order to let them know that their configuration was not/cannot be
respected.

```rs
const MAX_LEN_LOG2: u32 = match envparse::parse_env!(try "OPTIONAL_MAX_LEN_LOG2" as u32 in 0..32) {
    Some(v) => v,
    None => 5,
}
const MAX_LEN: usize = 1 << MAX_LEN_LOG2;
struct Thing {
    len: [u8; MAX_LEN],
}
```
