use lazy_static::lazy_static;
use std::env;

mod errors;

pub use ejdict_rs_core::{Dictionary, SearchMode, Word};
pub use errors::{Error, ErrorKind, Result};

pub type Candidate<T> = ejdict_rs_core::Candidate<std::vec::IntoIter<T>>;

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

pub fn look(word: &str, mode: SearchMode) -> Result<&Word> {
    let ref dict: Dictionary = *EJDICT_DISCIONARY;
    dict.look(word, mode).ok_or_else(|| {
        let kind = ErrorKind::NotFound {
            en: word.to_owned(),
        };
        Error::from(kind)
    })
}

pub fn candidates(word: &str, mode: SearchMode) -> Result<Candidate<Word>> {
    let dict = load_dictionary()?;
    Ok(dict.candidates(word, mode))
}