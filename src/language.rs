#[derive(PartialEq, Debug)]
pub enum Language {
    JavaScript,
    Elm,
}

pub fn from_string(lang_name: &str) -> Option<Language> {
    match lang_name.to_ascii_lowercase().as_str() {
        "javascript" => Some(Language::JavaScript),
        "elm" => Some(Language::Elm),
        _ => None,
    }
}
