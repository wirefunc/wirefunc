use types::FieldId;

/// FieldId is an unsigned 64-bit integer representing a particular field's
/// production identifier.
///
/// * In JSON, this corresponds to its minified field name, e.g. 0 ==> "a"
/// * In compact binary format, this uniquely identifies it among its siblings.
///
/// The to_string function converts from u64 to minified JSON field name.
/// The from_string function converts from minified JSON field name to u64. This
/// is useful when reading from History, which stores the field ID by its
/// production minified field name. (This is more compact and useful as a human-
/// readable piece of information.)
///
/// For the sake of simplicity and speed, this minifies to lowercase letters in
/// JSON. It is theoretically possible to use other characters, but that code
/// would be much more error-prone.
///
/// For example, it would have to escape backslashes and percent signs because
/// those have special meaning when encoding to a query string for a HTTP GET.
/// Also if you have single-digit numbers for field names, some JS engines will
/// think they are arrays. Also you have to watch out for surrogate pairs.
///
/// Also the more complex the field string <-> integer algorithm is, the harder
/// and more error-prone it is for others to make third-party tooling around it.
///
/// All in all, this seems like the best trade-off of compactness to simplicity!

#[allow(dead_code)]
pub fn from_string(str: &str) -> Option<FieldId> {
    if str.is_empty() {
        return None;
    }

    let mut result: u64 = 0;
    let mut index: u32 = 0;
    let last_index: u32 = str.len() as u32 - 1;

    for char in str.chars() {
        let num: u64 = char as u64;

        if num > 122 {
            return None;
        } else if num >= 97 {
            let coefficient: u64 = 26u32.pow(last_index - index) as u64;

            // a - z
            result += coefficient * (num - 96);
        } else {
            return None;
        }

        index += 1;
    }

    // We've been 1-indexed up to this point, but we want to be 0-indexed.
    Some(result - 1)
}

#[allow(dead_code)]
pub fn to_string(field_id: FieldId) -> String {
    let mut num: u64 = field_id;
    let mut digits: Vec<u8> = Vec::new();

    loop {
        if num < 26 {
            digits.push(to_digit(num));
            break;
        } else {
            digits.push(to_digit(num % 26));

            num = (num / 26) - 1;
        }
    }

    digits.reverse();

    String::from_utf8(digits).unwrap()
}

fn to_digit(num: u64) -> u8 {
    97 + num as u8
}

// For reference
//
// ASCII codes (decimal)
// '0' - 48
// '1' - 49
// ...
// '9' - 57
//
// [ ':', ';', '<', '=', '>', '?', '@' ] are 58-64
//
// 'A' - 65
// 'B' - 66
// ...
// 'Z' - 90
//
// [ '[', '\\', ']', '^', '_', '`' ] are 91-96
//
// 'a' - 97
// 'b' - 98
// ...
// 'z' - 122
