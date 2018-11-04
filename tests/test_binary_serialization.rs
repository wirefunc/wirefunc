extern crate byteorder;
extern crate tempfile;
extern crate wf;

use std::fs::File;
use tempfile::TempDir;
use wf::arrays;
use wf::pointers;
use wf::pointers::Pointer;

// For reference:
//
// |--Index within Segment--|--Segment--|--Data Length--|--Composite?--|
//          32 bits            16 bits       15 bits          1 bit
//

#[test]
fn test_decode() {
    let word: u64 =
        0b0000_0000_0000_0000_0000_0000_0000_0011__0000_0000_0000_0101__000_0000_0000_0110__1;
    let expected = Pointer {
        word_index: 3,
        length: 6,
        segment_id_offset: 5,
        is_composite: true,
    };
    let actual = pointers::decode(word);

    assert_eq!(expected, actual);
}

#[test]
fn test_segment_pointer_reflexive() {
    let expected: u64 =
        0b0000_0000_0000_0000_0000_0000_0000_0011__0000_0000_0000_0101__000_0000_0000_0110__1;
    let actual = pointers::encode(Pointer {
        word_index: 3,
        length: 6,
        segment_id_offset: 5,
        is_composite: true,
    });

    assert_eq!(expected, actual);
}

#[test]
fn test_i64_arrays() {
    // Write several arrays to the same file, then read them back.

    let file_path = TempDir::new().unwrap().into_path().join("deleteme.binary");
    let mut file = File::create(&file_path).unwrap();
    let array_lengths = vec![1, 22, 333, 4444];

    // Write several arrays to the file
    for array_length in &array_lengths {
        let mut arr: Vec<i64> = vec![];

        for num in 0..*array_length {
            arr.push(num % 42);
        }

        arrays::write_i64_array_to(&mut file, &arr).unwrap();
    }

    // Read several arrays out of the file
    let mut file = File::open(file_path).unwrap();

    for expected_array_length in array_lengths {
        let result: Vec<i64> = arrays::read_i64_array_from(&mut file).unwrap();

        // Check its length
        assert_eq!(expected_array_length, result.len() as i64);

        // Check its elements
        for num in 0..expected_array_length {
            let actual: Option<&i64> = result.get(num as usize);

            assert_eq!(Some(&((num as i64) % 42)), actual);
        }
    }
}
