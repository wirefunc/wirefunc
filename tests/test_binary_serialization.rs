extern crate byteorder;
extern crate tempfile;
extern crate wf;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::fs::File;
use std::io;
use std::path::PathBuf;
use tempfile::TempDir;

#[test]
fn serialize_deserialize() {
    let file_path = TempDir::new().unwrap().into_path().join("deleteme.binary");
    let expected_array_length: usize = 1337;

    // Write the file
    write_test_file(&file_path, expected_array_length).unwrap();

    // Read the same file back in
    let result = read_test_file(&file_path).unwrap();

    // Check its length
    assert_eq!(expected_array_length, result.len());

    // Check its elements
    for num in 0..expected_array_length {
        assert_eq!(Some(&((num as i64) % 42)), result.get(num));
    }
}

fn write_test_file(file_path: &PathBuf, desired_array_length: usize) -> std::io::Result<()> {
    let mut file = File::create(file_path)?;

    write_i64_array_to(&mut file, desired_array_length as u32)?;

    Ok(())
}

fn read_test_file(file_path: &PathBuf) -> std::io::Result<Vec<i64>> {
    let mut file = File::open(file_path)?;

    read_i64_array_from(&mut file)
}

/// First read the length of the array. It will be the first u32 in the bytes.
/// Then, read the elements of the array, each of which is an i64.
fn read_i64_array_from<R: io::Read>(reader: &mut R) -> std::io::Result<Vec<i64>> {
    let array_length: u32 = reader.read_u32::<LittleEndian>()?;
    let mut buffer: Vec<i64> = vec![0i64; array_length as usize];

    reader.read_i64_into::<LittleEndian>(&mut buffer)?;

    Ok(buffer)
}

/// First write the length of the array in u32
/// Then, write the elements of the array, each of which is an i64.
fn write_i64_array_to<W: io::Write>(writer: &mut W, length: u32) -> std::io::Result<()> {
    let mut buffer: Vec<u8> = vec![];

    // Write the length
    buffer.write_u32::<LittleEndian>(length).unwrap();

    // Write the elements
    for num in 0..length {
        buffer.write_i64::<LittleEndian>((num as i64) % 42).unwrap();
    }

    writer.write_all(&buffer)?;

    Ok(())
}
