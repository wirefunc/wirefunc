extern crate byteorder;
extern crate tempfile;
extern crate wf;

use std::fs::File;
use tempfile::TempDir;
use wf::arrays;

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
