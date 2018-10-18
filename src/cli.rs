extern crate clap;

use self::clap::{App, Arg};
use language;
use language::Language;
use std::iter::FromIterator;
use std::path::{Path, PathBuf};

// Use the version number in Cargo.toml
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[derive(PartialEq, Debug)]
pub enum Problem {
    InvalidLanguage(String),
}

pub struct CliArgs {
    pub client: Option<Language>,
    pub server: Option<Language>,
    pub file_paths: Vec<PathBuf>,
}

const ARG_CLIENT: &'static str = "client";
const ARG_SERVER: &'static str = "server";
const FILES_OR_DIRECTORIES: &'static str = "FILES_OR_DIRECTORIES";

pub fn parse_args<'a>() -> Result<CliArgs, Problem> {
    let matches = App::new("wf")
        .version(VERSION)
        .arg(
            Arg::with_name(ARG_CLIENT)
                .long("client")
                .value_name("LANGUAGE")
                .help("Language to use for the client.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name(ARG_SERVER)
                .long("server")
                .value_name("LANGUAGE")
                .help("Language to use for the server.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name(FILES_OR_DIRECTORIES)
                .help("Run all tests found in these files and directories")
                .multiple(true)
                .index(1),
        )
        .get_matches();

    let client = validate_language(matches.value_of(ARG_CLIENT))?;
    let server = validate_language(matches.value_of(ARG_SERVER))?;
    let file_paths: Vec<PathBuf> = Vec::from_iter(
        matches
            .values_of(FILES_OR_DIRECTORIES)
            .unwrap_or(Default::default())
            .map(|value| (Path::new(value).to_path_buf())),
    );

    Ok(CliArgs {
        client: client,
        server: server,
        file_paths: file_paths,
    })
}

fn validate_language(arg: Option<&str>) -> Result<Option<Language>, Problem> {
    match arg.map(String::from) {
        Some(string) => language::from_string(string.as_str())
            .ok_or(Problem::InvalidLanguage(string))
            .map(Some),
        None => Ok(None),
    }
}

// prints something like this:
//
// wirefunc 0.1.0
// --------------
pub fn print_headline() {
    let headline = String::from("wirefunc ") + VERSION;
    let bar = "-".repeat(headline.len());

    println!("\n{}\n{}\n", headline, bar);
}
