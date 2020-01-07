use ejdict_rs_core::{Dictionary, Word};
use std::env;
use std::fs;
use std::io;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::str;

fn main() -> io::Result<()> {
    let ejdict_force_update = env::var("EJDICT_FORCE_UPDATE").ok().is_some();
    let output_dir = env::var("OUT_DIR").unwrap();
    let output_path = PathBuf::new().join(output_dir).join("ejdict.json");
    if output_path.exists() && !ejdict_force_update {
        return Ok(());
    }
    let res = load_ejdict();
    let words = res.lines().map(Word::parse_line).collect::<Vec<_>>();
    let dict = Dictionary::new(words);
    let json = serde_json::to_string_pretty(&dict).unwrap();
    let mut output = BufWriter::new(fs::File::create(output_path)?);
    output.write_all(json.as_bytes())
}

fn load_ejdict() -> String {
    let manifest_dir: &'static str = env!("CARGO_MANIFEST_DIR");
    let ejdict_local_path = PathBuf::new()
        .join(&manifest_dir)
        .join("res")
        .join("ejdic-hand-utf8.txt");
    fs::read_to_string(&ejdict_local_path).unwrap()
}
