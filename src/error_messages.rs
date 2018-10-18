use cli;
use problems::Problem;

pub fn report(problem: Problem) -> String {
    match problem {
        Problem::Cli(cli::Problem::InvalidLanguage(lang)) => format!(
            "{} is not a supported language. Supported languages include JavaScript and Elm.",
            lang
        ),
    }
}
