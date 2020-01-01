use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Dictionary {
    words: Vec<Word>,
}

impl Dictionary {
    pub fn new(words: Vec<Word>) -> Self {
        Dictionary { words }
    }

    pub fn search(&self, pat: &str, mode: SearchMode) -> Option<&Word> {
        self.words.iter().find_map(|word| word.matched(pat, &mode))
    }

    pub fn candidates(self, pat: &str) -> Candidate<std::vec::IntoIter<Word>> {
        let inner_iter = self.into_iter();
        Candidate::new(inner_iter, pat.to_owned())
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
        self.words().iter().find_map(|en| {
            if callback(en) {
                Some(self)
            } else {
                None
            }
        })
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

pub enum SearchMode {
    Exact,
    Fuzzy,
    Lower,
}

pub struct Candidate<I>
    where I: Iterator<Item=Word>
{
    inner_iter: I,
    pat: String,
}

impl<I> Candidate<I>
    where I: Iterator<Item=Word>
{
    fn new(inner_iter: I, pat: String) -> Candidate<I> {
        Candidate { inner_iter, pat }
    }
}

impl<I> Iterator for Candidate<I>
    where I: Iterator<Item=Word>
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Word> {
        let Candidate { inner_iter, pat } = self;
        inner_iter
            .find_map(|word| {
                if word.matched(pat, &SearchMode::Fuzzy).is_some() {
                    Some(word)
                } else {
                    None
                }
            })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
