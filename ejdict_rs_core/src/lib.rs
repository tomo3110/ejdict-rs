use failure::Fail;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Deserialize, Serialize)]
pub struct Dictionary {
    words: Vec<Word>,
}

impl Dictionary {
    pub fn new(words: Vec<Word>) -> Self {
        Dictionary { words }
    }

    pub fn look(&self, pat: &str, mode: SearchMode) -> Option<&Word> {
        self.words.iter().find_map(|word| word.matched(pat, &mode))
    }

    pub fn candidates(self, pat: &str, mode: SearchMode) -> Candidate<std::vec::IntoIter<Word>> {
        let inner_iter = self.into_iter();
        Candidate::new(inner_iter, pat.to_owned(), mode)
    }
}

impl IntoIterator for Dictionary {
    type Item = Word;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.words.into_iter()
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Deserialize, Serialize)]
pub struct Word {
    words: Vec<String>,
    mean: String,
}

impl Word {
    pub fn parse_line(line: &str) -> Word {
        let secs = line.split("\t").collect::<Vec<&str>>();
        let words = String::from(secs[0]);
        let mean = String::from(secs[1]);
        let words = words
            .split(",")
            .map(|word| word.to_owned())
            .collect::<Vec<String>>();
        Word { words, mean }
    }

    pub fn words(&self) -> &Vec<String> {
        self.words.as_ref()
    }

    pub fn mean(&self) -> &str {
        self.mean.as_str()
    }

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

pub struct Candidate<I>
where
    I: Iterator<Item = Word>,
{
    inner_iter: I,
    pat: String,
    mode: SearchMode,
}

impl<I> Candidate<I>
where
    I: Iterator<Item = Word>,
{
    fn new(inner_iter: I, pat: String, mode: SearchMode) -> Candidate<I> {
        Candidate { inner_iter, pat, mode }
    }
}

impl<I> Iterator for Candidate<I>
where
    I: Iterator<Item = Word>,
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

#[derive(Debug, Fail)]
pub enum ConvertError {
    #[fail(
        display = "Invalid argument: The argument isn't convertible to SearchMode. argument: {}",
        argument
    )]
    InvalidSearchModeName { argument: String },
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
