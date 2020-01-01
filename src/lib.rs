use lazy_static::lazy_static;
use std::env;

mod errors;

pub use ejdict_rs_core::{Dictionary, Word};
pub use errors::{Error, ErrorKind, Result};

lazy_static! {
    static ref EJDICT_DISCIONARY: Dictionary = load_dictionary().unwrap();
}

fn load_dictionary() -> Result<Dictionary> {
    let src = if cfg!(windows) {
        include_str!(concat!(env!("OUT_DIR"), "\\ejdict.json"))
    } else {
        include_str!(concat!(env!("OUT_DIR"), "/ejdict.json"))
    };
    let dict = serde_json::from_str::<Dictionary>(src)?;
    Ok(dict)
}

pub fn search(word: &str) -> Option<&Word> {
    let ref dict: Dictionary = *EJDICT_DISCIONARY;
    dict.search(word)
}

#[cfg(test)]
mod tests {
    use crate::search;

    #[test]
    fn text_search() {
        assert_ne!(search("Will"), None)
    }
}
