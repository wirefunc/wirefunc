extern crate byteorder;

pub mod array;
pub mod arrays;
pub mod cli;
pub mod code_gen;
pub mod error_messages;
pub mod field_id;
pub mod language;
pub mod pointer;
pub mod problems;
pub mod record;
pub mod types;

use problems::Problem;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::path::PathBuf;

fn report_problem(problem: Problem) {
    // TODO color "Error:" in red.
    eprintln!("\nError: {}", error_messages::report(problem));

    std::process::exit(1);
}

pub fn run() -> () {
    // Parse and validate CLI arguments
    match cli::parse_args().map_err(Problem::Cli) {
        Err(problem) => {
            report_problem(problem);
        }
        Ok(args) => {
            let _unique_file_paths: HashSet<PathBuf> = HashSet::from_iter(
                args.file_paths
                    .iter()
                    .map(|file_path| file_path.to_path_buf()),
            );

            // Print the headline. Something like:
            //
            // wirefunc 0.1.0
            // --------------
            cli::print_headline();
        }
    }
}
