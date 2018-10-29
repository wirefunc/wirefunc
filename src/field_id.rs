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

#[cfg(test)]
mod test_from_string {
    use std::char;
    use std::collections::HashMap;
    use types::FieldId;

    #[test]
    fn a_is_zero() {
        assert_eq!(super::from_string("a"), Some(0));
    }

    #[test]
    fn capital_a_is_none() {
        assert_eq!(super::from_string("A"), None);
    }

    fn assert_invalid_char(num: u32) {
        match char::from_u32(num) {
            Some(ch) => {
                assert_eq!(None, super::from_string(ch.to_string().as_str()));
            }
            None => panic!(format!("Somehow, {} is an invalid char!", num)),
        }
    }

    #[test]
    fn invalid_one_digit() {
        for num in 0..96 {
            assert_invalid_char(num);
        }

        for num in 123..255 {
            assert_invalid_char(num);
        }
    }

    fn assert_accurate_mappings(mappings: HashMap<&str, FieldId>) {
        for (key, val) in mappings.iter() {
            assert_eq!(super::from_string(key), Some(val.to_owned()));
        }
    }

    #[test]
    fn one_digit() {
        assert_accurate_mappings(hashmap!{
            "a" => 0,
            "b" => 1,
            "c" => 2,
            "d" => 3,
            "e" => 4,
            "f" => 5,
            "g" => 6,
            "h" => 7,
            "i" => 8,
            "j" => 9,
            "k" => 10,
            "l" => 11,
            "m" => 12,
            "n" => 13,
            "o" => 14,
            "p" => 15,
            "q" => 16,
            "r" => 17,
            "s" => 18,
            "t" => 19,
            "u" => 20,
            "v" => 21,
            "w" => 22,
            "x" => 23,
            "y" => 24,
            "z" => 25,
        });
    }

    #[test]
    fn two_digits_starting_with_a() {
        assert_accurate_mappings(hashmap!{
            "aa" => 26,
            "ab" => 27,
            "ac" => 28,
            "ad" => 29,
            "ae" => 30,
            "af" => 31,
            "ag" => 32,
            "ah" => 33,
            "ai" => 34,
            "aj" => 35,
            "ak" => 36,
            "al" => 37,
            "am" => 38,
            "an" => 39,
            "ao" => 40,
            "ap" => 41,
            "aq" => 42,
            "ar" => 43,
            "as" => 44,
            "at" => 45,
            "au" => 46,
            "av" => 47,
            "aw" => 48,
            "ax" => 49,
            "ay" => 50,
            "az" => 51,
        });
    }

    #[test]
    fn two_digits_starting_with_b() {
        assert_accurate_mappings(hashmap!{
            "ba" => 52,
            "bb" => 53,
            "bc" => 54,
            "bd" => 55,
            "be" => 56,
            "bf" => 57,
            "bg" => 58,
            "bh" => 59,
            "bi" => 60,
            "bj" => 61,
            "bk" => 62,
            "bl" => 63,
            "bm" => 64,
            "bn" => 65,
            "bo" => 66,
            "bp" => 67,
            "bq" => 68,
            "br" => 69,
            "bs" => 70,
            "bt" => 71,
            "bu" => 72,
            "bv" => 73,
            "bw" => 74,
            "bx" => 75,
            "by" => 76,
            "bz" => 77,
        });
    }

    #[test]
    fn two_digits_starting_with_c() {
        assert_accurate_mappings(hashmap!{
            "ca" => 78,
            "cb" => 79,
            // ...
            "cy" => 102,
            "cz" => 103,
        });
    }

    #[test]
    fn two_digits_starting_with_y() {
        assert_accurate_mappings(hashmap!{
            "ya" => 650,
            "yb" => 651,
            // ...
            "yy" => 674,
            "yz" => 675,
        });
    }

    #[test]
    fn two_digits_starting_with_z() {
        assert_accurate_mappings(hashmap!{
            "za" => 676,
            "zb" => 677,
            // ...
            "zy" => 700,
            "zz" => 701,
        });
    }

    #[test]
    fn three_digits_starting_with_a() {
        assert_accurate_mappings(hashmap!{
            "aaa" => 702,
            "aab" => 703,
            // ...
            "aay" => 726,
            "aaz" => 727,
            "aba" => 728,
            "abb" => 729,
        });
    }

    #[test]
    fn three_digits_starting_with_b() {
        assert_accurate_mappings(hashmap!{
            "baa" => 1378,
            "bab" => 1379,
            // ...
            "bay" => 1402,
            "baz" => 1403,
            "bba" => 1404,
            "bbb" => 1405,
            "bbc" => 1406,
        });
    }
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

#[cfg(test)]
mod test_to_string {
    use std::collections::HashMap;
    use types::FieldId;

    #[test]
    fn zero_is_a() {
        assert_eq!(super::to_string(0), "a");
    }

    fn assert_accurate_mappings(mappings: HashMap<FieldId, &str>) {
        for (key, val) in mappings.iter() {
            assert_eq!(super::to_string(*key), val.to_owned());
        }
    }

    #[test]
    fn one_digit() {
        assert_accurate_mappings(hashmap!{
            0 => "a",
            1 => "b",
            2 => "c" ,
            3 => "d" ,
            4 => "e" ,
            5 => "f" ,
            6 => "g" ,
            7 => "h" ,
            8 => "i" ,
            9 => "j" ,
            10 => "k" ,
            11 => "l" ,
            12 => "m" ,
            13 => "n" ,
            14 => "o" ,
            15 => "p" ,
            16 => "q" ,
            17 => "r" ,
            18 => "s" ,
            19 => "t" ,
            20 => "u" ,
            21 => "v" ,
            22 => "w" ,
            23 => "x" ,
            24 => "y" ,
            25 => "z" ,
        });
    }

    #[test]
    fn two_digits_starting_with_a() {
        assert_accurate_mappings(hashmap!{
            26 => "aa" ,
            27 => "ab" ,
            28 => "ac" ,
            29 => "ad" ,
            30 => "ae" ,
            31 => "af" ,
            32 => "ag" ,
            33 => "ah" ,
            34 => "ai" ,
            35 => "aj" ,
            36 => "ak" ,
            37 => "al" ,
            38 => "am" ,
            39 => "an" ,
            40 => "ao" ,
            41 => "ap" ,
            42 => "aq" ,
            43 => "ar" ,
            44 => "as" ,
            45 => "at" ,
            46 => "au" ,
            47 => "av" ,
            48 => "aw" ,
            49 => "ax" ,
            50 => "ay" ,
            51 => "az" ,
        });
    }

    #[test]
    fn two_digits_starting_with_b() {
        assert_accurate_mappings(hashmap!{
            52 => "ba" ,
            53 => "bb" ,
            54 => "bc" ,
            55 => "bd" ,
            56 => "be" ,
            57 => "bf" ,
            58 => "bg" ,
            59 => "bh" ,
            60 => "bi" ,
            61 => "bj" ,
            62 => "bk" ,
            63 => "bl" ,
            64 => "bm" ,
            65 => "bn" ,
            66 => "bo" ,
            67 => "bp" ,
            68 => "bq" ,
            69 => "br" ,
            70 => "bs" ,
            71 => "bt" ,
            72 => "bu" ,
            73 => "bv" ,
            74 => "bw" ,
            75 => "bx" ,
            76 => "by" ,
            77 => "bz" ,
        });
    }

    #[test]
    fn two_digits_starting_with_c() {
        assert_accurate_mappings(hashmap!{
            78 => "ca" ,
            79 => "cb" ,
            // ...
            102 => "cy" ,
            103 => "cz" ,
        });
    }

    #[test]
    fn two_digits_starting_with_y() {
        assert_accurate_mappings(hashmap!{
            650 => "ya" ,
            651 => "yb" ,
            // ...
            674 => "yy" ,
            675 => "yz" ,
        });
    }

    #[test]
    fn two_digits_starting_with_z() {
        assert_accurate_mappings(hashmap!{
            676 => "za" ,
            677 => "zb" ,
            // ...
            700 => "zy" ,
            701 => "zz" ,
        });
    }

    #[test]
    fn three_digits_starting_with_a() {
        assert_accurate_mappings(hashmap!{
            702 => "aaa" ,
            703 => "aab" ,
            // ...
            726 => "aay" ,
            727 => "aaz" ,
            728 => "aba" ,
            729 => "abb" ,
        });
    }

    #[test]
    fn three_digits_starting_with_b() {
        assert_accurate_mappings(hashmap!{
            1378 => "baa" ,
            1379 => "bab" ,
            // ...
            1402 => "bay" ,
            1403 => "baz" ,
            1404 => "bba" ,
            1405 => "bbb" ,
            1406 => "bbc" ,
        });
    }
}

#[cfg(test)]
mod test_reflexive {
    #[test]
    fn to_string_and_from_string_are_reflexive() {
        for num in 0..123456 {
            let str_version = super::to_string(num);

            assert_eq!(super::from_string(str_version.as_str()), Some(num));
        }
    }
}
