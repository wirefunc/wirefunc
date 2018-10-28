use std::collections::HashSet;
use std::fs::File;
use types::Field;

#[derive(Debug)]
pub enum Problem<'a> {
    RetiredNamesUsed(HashSet<&'a str>),
}

pub fn productionize<'a>(
    retired_names: HashSet<&str>,
    fields_by_name: Vec<(&'a str, Box<Field>)>,
) -> Result<Vec<(String, (String, Box<Field>))>, Problem<'a>> {
    let invalid_names: HashSet<&'a str> = check_for_retired_names(retired_names, &fields_by_name);

    if invalid_names.is_empty() {
        Ok(minify_names(fields_by_name))
    } else {
        Err(Problem::RetiredNamesUsed(invalid_names))
    }
}

pub fn generate_elm_code(fields: Vec<(String, (String, Box<Field>))>, file: File) {
    // TODO stream output to the file as we figure out what to write!
}

pub fn generate_js_code(fields: Vec<(String, (String, Box<Field>))>, file: File) {
    // TODO stream output to the file as we figure out what to write!
}

fn check_for_retired_names<'a>(
    retired_names: HashSet<&str>,
    fields_by_name: &Vec<(&'a str, Box<Field>)>,
) -> HashSet<&'a str> {
    let mut violations: HashSet<&'a str> = HashSet::new();

    for (name, _) in fields_by_name.iter() {
        if retired_names.contains(name) {
            violations.insert(name);
        }
    }

    violations
    // TODO intersect the retiredNames set with names. Return a HashSet of any
    // retired names that appear in the names set.
    // We're starting with a HashMap<&str, Field>, so use whatever seems appropriate for names.
}

fn minify_names<'a>(
    fields_by_name: Vec<(&'a str, Box<Field>)>,
) -> Vec<(String, (String, Box<Field>))> {
    // TODO iterate over the fields
    panic!("TODO");
}

// TODO:
//
//
// {-| ALWAYS sort all fields alphabetically before doing anything else! Crucial for diffing, for history, for numbering, etc. This way reordering is harmless. -}
// fun minifyNames(retiredNames: Set<String>, fields: Array<Field>): Array<Field, String>
//
// We need to add this as metadata rather than mutating the Array, so we can
// generate the code that back-converts and applies defaults.
//
// So we take this Array<Field, String> and output:
//
// JS:
//
// var json = JSON.parse(body);
//
// return { userId /*: int*/: json.a, email /*: string*/: json.b, ... }
//
// [TODO: also generate .d.ts file]
//
// Elm:
//
// Json.Decode.succeed (...record constructor...)
//     |> required "a" int
//     |> required "b" string
//
