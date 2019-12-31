use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Dictionary {
    words: Vec<Word>,
}

impl Dictionary {
    pub fn new(words: Vec<Word>) -> Self {
        Dictionary { words }
    }

    pub fn search(&self, pat: &str) -> Option<&Word> {
        self.words
            .iter()
            .find_map(|word| word.matched(pat))
    }
}

impl IntoIterator for Dictionary {
    type Item = Word;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.words.into_iter()
    }
}

#[derive(Debug, Deserialize, Serialize)]
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

    pub fn matched(&self, pat: &str) -> Option<&Word> {
        self.words()
            .iter()
            .find_map(|en| {
                if en.starts_with(pat) {
                    Some(self)
                } else {
                    None
                }
            })
    }
}

impl From<(Vec<String>, String)> for Word {
    fn from(line: (Vec<String>, String)) -> Self {
        let (words, mean) = line;
        Word { words, mean }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
