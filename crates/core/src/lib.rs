//! # ejdict-rs-core
//!
//! This library is detailed implementation of ejdict-rs.
//!
//! ## Note
//!
//! We do not recommend using this library directly.
//! Please use from the following URL.
//!
//! https://github.com/tomo3110/ejdict-rs
//!
//! ## Dependencies
//!
//! - failure
//!   - Apache 2.0, MIT
//!   - Error management
//! - serde
//!   - Apache 2.0, MIT
//!   - Serialization framework
//!
//! Thanks for the great crates.
//!
//! ## License
//!
//! This software is under [MIT License](https://github.com/tomo3110/ejdict-rs/blob/master/LICENCE).
//!

use failure::Fail;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// Dictionary struct
/// This struct is holds all the words contained in the English-Japanese dictionary.
#[derive(Debug, Deserialize, Serialize)]
pub struct Dictionary {
    words: Vec<Word>,
}

impl Dictionary {
    /// Constructor for Dictionary struct.
    pub fn new(words: Vec<Word>) -> Self {
        Dictionary { words }
    }

    /// Look up words from English-Japanese dictionary.
    pub fn look(&self, pat: &str, mode: SearchMode) -> Option<&Word> {
        self.words.iter().find_map(|word| word.matched(pat, &mode))
    }

    /// Get matching candidate words.
    pub fn candidates(self, pat: &str, mode: SearchMode) -> Candidates<std::vec::IntoIter<Word>> {
        let inner_iter = self.into_iter();
        Candidates::new(inner_iter, pat.to_owned(), mode)
    }
}

impl IntoIterator for Dictionary {
    type Item = Word;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.words.into_iter()
    }
}

/// This Struct is that holds word translation information.
/// `words` field is that holds similar English words.
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Deserialize, Serialize)]
pub struct Word {
    words: Vec<String>,
    mean: String,
}

impl Word {
    /// Construct for `Word` struct.
    pub fn new(words: Vec<String>, mean: String) -> Self {
        Word { words, mean }
    }

    /// Construct `Word` structure by parsing line string.
    pub fn parse_line(line: &str) -> Self {
        let secs = line.split("\t").collect::<Vec<&str>>();
        let words = String::from(secs[0]);
        let mean = String::from(secs[1]);
        let words = words
            .split(",")
            .map(|word| word.to_owned())
            .collect::<Vec<String>>();
        Self::new(words, mean)
    }

    /// Get words reference.
    pub fn words(&self) -> &Vec<String> {
        self.words.as_ref()
    }

    /// Get mean reference.
    pub fn mean(&self) -> &str {
        self.mean.as_str()
    }

    /// Checks if this word matches the argument string.
    pub fn matched(&self, pat: &str, mode: &SearchMode) -> Option<&Word> {
        match mode {
            SearchMode::Exact => self.exact_matched(pat),
            SearchMode::Fuzzy => self.fuzzy_matched(pat),
            SearchMode::Lower => self.lower_matched(pat),
        }
    }

    fn base_matched<F>(&self, callback: F) -> Option<&Word>
    where
        F: Fn(&str) -> bool,
    {
        self.words()
            .iter()
            .find_map(|en| if callback(en) { Some(self) } else { None })
    }

    fn exact_matched(&self, pat: &str) -> Option<&Word> {
        self.base_matched(|en| en == pat)
    }

    fn fuzzy_matched(&self, pat: &str) -> Option<&Word> {
        self.base_matched(|en| en.starts_with(pat))
    }

    fn lower_matched(&self, pat: &str) -> Option<&Word> {
        self.base_matched(|en| en.to_lowercase().eq(pat))
    }
}

impl From<(Vec<String>, String)> for Word {
    fn from(line: (Vec<String>, String)) -> Self {
        let (words, mean) = line;
        Word { words, mean }
    }
}

/// SearchMode specifies how to check if a word is a match.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchMode {
    Exact,
    Fuzzy,
    Lower,
}

impl ToString for SearchMode {
    fn to_string(&self) -> String {
        use SearchMode::*;
        match self {
            Exact => "exact".to_string(),
            Fuzzy => "fuzzy".to_string(),
            Lower => "lower".to_string(),
        }
    }
}

impl FromStr for SearchMode {
    type Err = ConvertError;

    fn from_str(s: &str) -> Result<SearchMode, ConvertError> {
        use SearchMode::*;
        if Exact.to_string().eq(s) {
            return Ok(Exact);
        }
        if Fuzzy.to_string().eq(s) {
            return Ok(Fuzzy);
        }
        if Lower.to_string().eq(s) {
            return Ok(Lower);
        }
        Err(ConvertError::InvalidSearchModeName {
            argument: s.to_string(),
        })
    }
}

/// Candidate words.
/// This struct is implemented Iterator.
/// Use the iterator API to check the result.
#[derive(Debug, Clone)]
pub struct Candidates<I>
where
    I: Iterator<Item = Word> + Clone,
{
    inner_iter: I,
    pat: String,
    mode: SearchMode,
}

impl<I> Candidates<I>
where
    I: Iterator<Item = Word> + Clone,
{
    /// Constructor for Candidates<I> struct
    fn new(inner_iter: I, pat: String, mode: SearchMode) -> Candidates<I> {
        Candidates {
            inner_iter,
            pat,
            mode,
        }
    }
}

impl<I> Iterator for Candidates<I>
where
    I: Iterator<Item = Word> + Clone,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Word> {
        let pat = self.pat.as_str();
        let mode = &self.mode;
        self.inner_iter.find_map(|word| {
            if word.matched(pat, mode).is_some() {
                Some(word)
            } else {
                None
            }
        })
    }
}

/// An error returned when conversion from string to `SearchMode` fails.
#[derive(Debug, Fail, PartialEq, Eq)]
pub enum ConvertError {
    #[fail(
        display = "Invalid argument: The argument isn't convertible to SearchMode. argument: {}",
        argument
    )]
    InvalidSearchModeName { argument: String },
}

#[cfg(test)]
mod tests {
    use crate::{ConvertError, Dictionary, SearchMode, Word};
    use std::str::FromStr;

    #[test]
    fn test_dictionary_look() {
        let words = get_test_words();
        let dict = Dictionary::new(words);
        let apple = dict.look("apple", SearchMode::Exact);
        assert_eq!(apple, Some(&word1()));
        let blue = dict.look("blue", SearchMode::Exact);
        assert_eq!(blue, Some(&word4()));
    }

    #[test]
    fn test_dictionary_candidates() {
        let words = get_test_words();
        let dict = Dictionary::new(words);
        let mut apple_candidates = dict.candidates("apple", SearchMode::Fuzzy);
        assert_eq!(apple_candidates.next(), Some(word1()));
        assert_eq!(apple_candidates.next(), Some(word2()));
        assert_eq!(apple_candidates.next(), Some(word3()));
        assert_eq!(apple_candidates.next(), None);
    }

    #[test]
    fn test_word_parse_list() {
        let apple = Word::parse_line("apple\t『リンゴ』;リンゴの木");
        assert_eq!(apple, word1());
    }

    #[test]
    fn test_word_matched() {
        let apple = Word::parse_line("apple\t『リンゴ』;リンゴの木");
        assert_eq!(apple.matched("apple", &SearchMode::Exact), Some(&word1()));
        assert_eq!(apple.matched("a", &SearchMode::Fuzzy), Some(&word1()));
        assert_eq!(apple.matched("Apple", &SearchMode::Lower), None);
        assert_eq!(apple.matched("blue", &SearchMode::Exact), None);
        assert_eq!(apple.matched("a", &SearchMode::Exact), None);
        assert_eq!(apple.matched("Apple", &SearchMode::Exact), None);
    }

    #[test]
    fn test_search_mode_from_str() {
        assert_eq!(SearchMode::from_str("exact"), Ok(SearchMode::Exact));
        assert_eq!(SearchMode::from_str("fuzzy"), Ok(SearchMode::Fuzzy));
        assert_eq!(SearchMode::from_str("lower"), Ok(SearchMode::Lower));
        assert_eq!(
            SearchMode::from_str("other"),
            Result::<SearchMode, ConvertError>::Err(ConvertError::InvalidSearchModeName {
                argument: "other".to_string()
            })
        );
    }

    fn get_test_words() -> Vec<Word> {
        vec![word1(), word2(), word3(), word4()]
    }

    fn word1() -> Word {
        Word::new(
            vec!["apple".to_string()],
            "『リンゴ』;リンゴの木".to_string(),
        )
    }

    fn word2() -> Word {
        Word::new(
            vec!["apple butter".to_string()],
            "リンゴジャム(リンゴに香料・砂糖を加えて煮つめたジャム)".to_string(),
        )
    }

    fn word3() -> Word {
        Word::new(
            vec!["apple green".to_string()],
            "澄んだ淡い緑色".to_string(),
        )
    }

    fn word4() -> Word {
        Word::new(
            vec!["blue".to_string()],
            "『青い』,あい色の / 青黒い / 《話》陰気な,憂うつな /\
             〈U〉『青色』,あい色;青色の着物 /\
             〈U〉〈C〉青色絵の具,あい色染料 / 《the~》《詩》青空,青い海 /\
             《the blues》《話 》気のふさぎ,うれいの色 /\
             《the blues》《ときに単数扱い》(ジャズ音楽の)ブルース /\
             …'を'青色にする"
                .to_string(),
        )
    }
}
