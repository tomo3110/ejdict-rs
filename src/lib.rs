//! # ejdict-rs
//!
//! This library is an English-Japanese dictionary that can be used via implemented API by Rust language.
//!
//! ## Overview
//!
//! This library is available through a simple API.
//! Since the dictionary data to be referenced is embedded in this crate,
//! The Japanese-English dictionary can be used immediately by simply obtaining  the crate from crates.io without depending on the database or file.
//!
//! The dictionary data of this library is "ejdict" which is a public domain dictionary.
//! See the following URL for details.
//!
//! https://github.com/kujirahand/EJDict
//!
//! ## Examples
//!
//! This library is used through two functions.
//!
//! **case1**: Look up words from dictionary.
//!
//! ```
//! use ejdict_rs::SearchMode;
//!
//! # fn main() -> ejdict_rs::Result<()> {
//! let word = ejdict_rs::look("apple", SearchMode::Exact)?;
//! assert_eq!(word.mean(), "『リンゴ』;リンゴの木");
//! #   Ok(())
//! # }
//! ```
//!
//! **case2**: Candidate list from dictionary.
//!
//! ```
//! use ejdict_rs::SearchMode;
//!
//! # fn main() -> ejdict_rs::Result<()> {
//! let candidates = ejdict_rs::candidates("apple", SearchMode::Fuzzy)?;
//! for word in candidates {
//!     // something ...
//! }
//! #   Ok(())
//! # }
//! ```
//!
//! ## Install
//!
//! Write the following contents in Cargo.toml.
//!
//! ```toml
//! [dependencies]
//! ejdict_rs = { git = "https://github.com/tomo3110/ejdict-rs" }
//! ```
//!
//! If you use the development version or a specific version, write as follows.
//!
//!  ```toml
//! [dependencies]
//! ejdict_rs = { git = "https://github.com/tomo3110/ejdict-rs" }
//! ```
//!
//! For details, check the following URL.
//!
//! https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#specifying-dependencies-from-git-repositories
//!
//! ## Dependencies
//!
//! - crates
//!   - failure
//!     - Apache 2.0, MIT
//!     - Error management
//!   - lazy_static
//!     - Apache 2.0, MIT
//!     - Copyright (c) 2010 The Rust Project Developers
//!     - A small macro for defining lazy evaluated static variables in Rust.
//!   - serde_json
//!     - Apache 2.0, MIT
//!     - Strongly typed JSON library.
//!   - reqwest
//!     - Apache 2.0, MIT
//!     - Copyright (c) 2016 Sean McArthur
//!     - Rust HTTP Client
//! - dictionary data
//!   - ejdict-hand
//!     - MIT
//!     - Copyright (c) 2016 kujirahand
//!     - English-Japanese Dictionary data (Public Domain)
//!
//! Thanks for the great crates and dictionary data.
//!
//! ## License
//!
//! This software is under [MIT License](https://github.com/tomo3110/ejdict-rs/blob/master/LICENCE).
//!
//! ## Author
//!
//! [tomo3110](https://github.com/tomo3110)
//!

use lazy_static::lazy_static;
use std::env;

mod errors;

pub use ejdict_rs_core::{Dictionary, SearchMode, Word};
pub use errors::{Error, ErrorKind, Result};

/// List of candidates that can be obtained as search results
pub type Candidate<T> = ejdict_rs_core::Candidates<std::vec::IntoIter<T>>;

lazy_static! {
    static ref EJDICT_DISCIONARY: Dictionary = load_dictionary().unwrap();
}

#[cfg(windows)]
fn get_ejdict_json<'a>() -> &'a str {
    include_str!(concat!(env!("OUT_DIR"), "\\ejdict.json"))
}

#[cfg(not(windows))]
fn get_ejdict_json<'a>() -> &'a str {
    include_str!(concat!(env!("OUT_DIR"), "/ejdict.json"))
}

fn load_dictionary() -> Result<Dictionary> {
    let src = get_ejdict_json();
    let dict = serde_json::from_str::<Dictionary>(src)?;
    Ok(dict)
}

/// Look up words from an English-Japanese Dictionary.
///
/// # Example
///
/// The following example shows how to Look up words.
///
///
/// ```
/// use ejdict_rs::SearchMode;
///
/// # fn main() -> ejdict_rs::Result<()> {
/// let word = ejdict_rs::look("apple", SearchMode::Exact)?;
/// assert_eq!(word.mean(), "『リンゴ』;リンゴの木");
/// #   Ok(())
/// # }
/// ```
///
pub fn look(word: &str, mode: SearchMode) -> Result<&Word> {
    let ref dict: Dictionary = *EJDICT_DISCIONARY;
    dict.look(word, mode).ok_or_else(|| {
        let kind = ErrorKind::NotFound {
            en: word.to_owned(),
        };
        Error::from(kind)
    })
}

/// Get matching candidate words.
///
/// # Example
///
/// ```
/// use ejdict_rs::SearchMode;
///
/// # fn main() -> ejdict_rs::Result<()> {
/// let candidates = ejdict_rs::candidates("apple", SearchMode::Fuzzy)?;
/// for word in candidates {
///     // something ...
/// }
/// # Ok(())
/// # }
/// ```
///
pub fn candidates(word: &str, mode: SearchMode) -> Result<Candidate<Word>> {
    let dict = load_dictionary()?;
    Ok(dict.candidates(word, mode))
}
