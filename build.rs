use std::env;
use std::io;
use std::fs;
use std::str;
use std::path::PathBuf;
use std::io::{BufWriter, Write};
use ejdict_rs_core::{Dict, Word};

const EJDICT_URL_DEFAULT: &'static str = "https://raw.githubusercontent.com/kujirahand/EJDict/master/release/ejdic-hand-utf8.txt";

fn main() -> io::Result<()> {
    let ejdict_url = env::var("EJDICT_URL").unwrap_or(EJDICT_URL_DEFAULT.to_owned());
    let ejdict_force_update = env::var("EJDICT_FORCE_UPDATE")
        .ok()
        .is_some();
    let output_dir = env::var("OUT_DIR").unwrap();
    let output_path = PathBuf::new()
        .join(output_dir)
        .join("res")
        .with_file_name("ejdict.json");
    if output_path.exists() && !ejdict_force_update {
        return Ok(());
    }
    let res =
        reqwest::get(&ejdict_url)
            .unwrap()
            .text()
            .unwrap();
    let words = res.lines()
        .map(Word::parse_line)
        .map(Word::from)
        .collect::<Vec<_>>();
    let dict = Dict::new(words);
    let json =
        serde_json::to_string_pretty(&dict).unwrap();
    let mut output = BufWriter::new(fs::File::create(output_path)?);
    output.write_all(json.as_bytes())
}