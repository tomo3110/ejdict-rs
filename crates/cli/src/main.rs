use clap::{
    app_from_crate, crate_authors, crate_description, crate_name, crate_version, App, Arg,
    SubCommand,
};
use ejdict_rs::{Result, SearchMode, Word};
use prettytable::{Cell, Row, Table};
use std::process;
use std::str::FromStr;

fn main() {
    let app = setup_app();
    let matches = app.get_matches();
    match run(matches) {
        Ok(_) => process::exit(0),
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
    }
}

fn run(matches: clap::ArgMatches) -> Result<()> {
    let mut table = Table::new();
    if let Some(look_matches) = matches.subcommand_matches("look") {
        let word = look_subcommand(look_matches)?;
        if look_matches.is_present("json") {
            let json = serde_json::to_string_pretty(&word)?;
            println!("{}", json);
        } else {
            table_set_header(&mut table);
            table_set_row(&mut table, word);
            table.printstd();
        }
    }
    if let Some(candidate_matches) = matches.subcommand_matches("candidates") {
        let candidates = candidate_subcommand(candidate_matches)?;
        let words = candidates.collect::<Vec<_>>();
        if candidate_matches.is_present("json") {
            let json = serde_json::to_string_pretty(&words)?;
            println!("{}", json);
        } else {
            table_set_header(&mut table);
            words.iter().for_each(|word| {
                table_set_row(&mut table, word);
            });
            table.printstd();
        }
    }
    Ok(())
}

fn setup_app<'a, 'b>() -> App<'a, 'b> {
    app_from_crate!()
        .subcommand(
            SubCommand::with_name("look")
                .about("Look it up the English-Japanese Dictionary.")
                .arg(Arg::with_name("en_word").takes_value(true).required(true))
                .arg(
                    Arg::with_name("mode")
                        .short("m")
                        .long("mode")
                        .help("Select search mode")
                        .value_name("mode")
                        .default_value("lower")
                        .required(false)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("json")
                        .long("json")
                        .help("Prints output format json")
                        .takes_value(false)
                        .required(false),
                ),
        )
        .subcommand(
            SubCommand::with_name("candidates")
                .about("Search result candidates for the English-Japanese Dictionary.")
                .arg(Arg::with_name("en_word").takes_value(true).required(true))
                .arg(
                    Arg::with_name("mode")
                        .short("m")
                        .long("mode")
                        .help("Select search mode")
                        .value_name("mode")
                        .default_value("fuzzy")
                        .required(false)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("number")
                        .short("n")
                        .long("number")
                        .help("Maximum number of hits in search results")
                        .value_name("number")
                        .default_value("5")
                        .required(false)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("json")
                        .long("json")
                        .help("Prints output format json")
                        .takes_value(false)
                        .required(false),
                ),
        )
}

fn look_subcommand<'a>(matches: &'a clap::ArgMatches) -> Result<&'a Word> {
    let en = matches.value_of("en_word").unwrap();
    let mode = matches
        .value_of("mode")
        .map(|mode| SearchMode::from_str(mode).unwrap_or(SearchMode::Lower))
        .unwrap();
    ejdict_rs::look(en, mode)
}

fn candidate_subcommand(matches: &clap::ArgMatches) -> Result<impl Iterator<Item = Word>> {
    let en = matches.value_of("en_word").unwrap();
    let mode = matches
        .value_of("mode")
        .map(|mode| SearchMode::from_str(mode).unwrap_or(SearchMode::Fuzzy))
        .unwrap();
    let number = matches
        .value_of("number")
        .unwrap_or("")
        .parse::<usize>()
        .unwrap_or(5);
    ejdict_rs::candidates(en, mode).and_then(|candidates| Ok(candidates.take(number)))
}

fn table_set_header(table: &mut Table) {
    table.add_row(Row::new(vec![Cell::new("word"), Cell::new("mean")]));
}

fn table_set_row(table: &mut Table, word: &Word) {
    let words: String = word.words().join(",");
    let means: String = word
        .mean()
        .split("/")
        .map(|mean| mean.trim())
        .collect::<Vec<_>>()
        .join("\n");
    table.add_row(Row::new(vec![Cell::new(&words), Cell::new(&means)]));
}
