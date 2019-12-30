use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Dict {
    words: Vec<Word>,
}

impl Dict {
    pub fn new(words: Vec<Word>) -> Self {
        Dict { words }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Word {
    pub words: Vec<String>,
    pub mean: String,
}

impl Word {
    pub fn parse_line(line: &str) -> (Vec<String>, String) {
        let secs = line.split("\t").collect::<Vec<&str>>();
        let words = String::from(secs[0]);
        let mean = String::from(secs[1]);
        let words = words
            .split(",")
            .map(|word| word.to_owned())
            .collect::<Vec<String>>();
        (words, mean)
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
