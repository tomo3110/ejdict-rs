ejdict-rs
====

[![Build and test](https://github.com/tomo3110/ejdict-rs/workflows/ejdict_rs/badge.svg)](https://github.com/tomo3110/ejdict-rs)
[![ejdict_rs at crates.io](https://img.shields.io/crates/v/ejdict_rs.svg)](https://crates.io/crates/ejdict_rs)
[![ejdict_rs at docs.rs](https://docs.rs/ejdict_rs/badge.svg)](https://docs.rs/ejdict_rs)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/tomo3110/ejdict-rs/blob/master/LICENSE)

This library is an English-Japanese dictionary that can be used via implemented API by Rust language.

DEMO
![DEMO](https://github.com/tomo3110/ejdict-rs/blob/master/doc/assets/ejdict_rs_cli_DEMO.gif)

## Overview

This library is available through a simple API.
Since the dictionary data to be referenced is embedded in this crate,
The Japanese-English dictionary can be used immediately by simply obtaining  the crate from crates.io without depending on the database or file.

The dictionary data of this library is "ejdict" which is a public domain dictionary.
See the following URL for details.

https://github.com/kujirahand/EJDict

## Examples

This library is used through two functions.

**case1**: Look up words from dictionary.

```rust
use ejdict_rs::SearchMode;

fn main() -> ejdict_rs::Result<()> {
    let word = ejdict_rs::look("apple", SeachMode::Exact)?;
    assert_eq!(word.mean(), "『リンゴ』;リンゴの木");
    Ok(())
}
```

**case2**: Candidate list from dictionary.

```rust
use ejdict_rs::SearchMode;

fn main() -> ejdict_rs::Result<()> {
    let candidates = ejdict_rs::candidates("apple", SeachMode::Fuzzy)?;
    for word in candidates {
        // something ...
    }
    Ok(())
}
```

## Install

Write the following contents in Cargo.toml.

```toml
[dependencies]
ejdict_rs = "0.0.1"
```

If you use the development version or a specific version, write as follows.

```toml
[dependencies]
ejdict_rs = { git = "https://github.com/tomo3110/ejdict-rs" }
```

For details, check the following URL.

https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#specifying-dependencies-from-git-repositories

## Dependencies

- crates
  - failure
    - Apache 2.0, MIT
    - Error management
  - lazy_static
    - Apache 2.0, MIT
    - Copyright (c) 2010 The Rust Project Developers
    - A small macro for defining lazy evaluated static variables in Rust.
  - serde_json
    - Apache 2.0, MIT
    - Strongly typed JSON library.
  - reqwest
    - Apache 2.0, MIT
    - Copyright (c) 2016 Sean McArthur
    - Rust HTTP Client
- dictionary data
  - ejdict-hand
    - MIT
    - Copyright (c) 2016 kujirahand
    - English-Japanese Dictionary data (Public Domain)

Thanks for the great crates and dictionary data.

## License

This software is under [MIT License](https://github.com/tomo3110/ejdict-rs/blob/master/LICENCE).

## Author

[tomo3110](https://github.com/tomo3110)

