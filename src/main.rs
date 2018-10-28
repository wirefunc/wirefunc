mod cli;
mod code_gen;
mod error_messages;
mod language;
mod problems;
mod types;

use problems::Problem;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::path::PathBuf;

fn main() {
    run().unwrap_or_else(report_problem);
}

fn report_problem(problem: Problem) {
    // TODO color "Error:" in red.
    eprintln!("\nError: {}", error_messages::report(problem));

    std::process::exit(1);
}

fn run() -> Result<(), Problem> {
    // Parse and validate CLI arguments
    let args = cli::parse_args().map_err(Problem::Cli)?;

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

    Ok(())
}
