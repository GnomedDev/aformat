# `aformat`

A no-std and no-alloc version of `format!` using `ToArrayString`.

Read the documentation via `cargo doc --open --no-deps` or on [docs.rs](https://docs.rs/aformat).

## Minimum Supported Rust Version

This is currently `1.79`, and is considered a breaking change to increase.

## Credits

- [@danielhenrymantilla](https://github.com/danielhenrymantilla), aka yandros, for providing much of the tricks needed to implement this on stable.
- Everyone who has contributed to [typenum](https://github.com/paholg/typenum), again for stable compatiblity.
- The rustc developers, who unknowingly stablized enough features for this to work.
