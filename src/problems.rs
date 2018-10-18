use cli;

#[derive(Debug)]
pub enum Problem {
    Cli(cli::Problem),
}
