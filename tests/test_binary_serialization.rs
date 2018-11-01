extern crate byteorder;
extern crate wf;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::fs::File;
use std::io;

#[test]
fn serialize_deserialize() {
    let filename = "tests/fixtures/1337-write2.binary";
    let expected_array_length: usize = 1337;

    // Write the file
    write_test_file(filename, expected_array_length).unwrap();

    // Read the same file back in
    let result = read_test_file(filename).unwrap();

    // Check its length
    assert_eq!(expected_array_length, result.len());

    // Check its elements
    for num in 0..expected_array_length {
        assert_eq!(Some(&((num as i64) % 42)), result.get(num));
    }
}

fn write_test_file(filename: &str, desired_array_length: usize) -> std::io::Result<()> {
    let mut write_file = File::create(filename)?;

    write_i64_array_to(&mut write_file, desired_array_length as u32)?;
    write_file.sync_all()?;

    Ok(())
}

fn read_test_file(filename: &str) -> std::io::Result<Vec<i64>> {
    let mut read_file = File::open(filename)?;

    read_i64_array_from(&mut read_file)
}

/// First read the length of the array. It will be the first u32 in the bytes.
/// Then, read the elements of the array, each of which is an i64.
fn read_i64_array_from<R: io::Read>(reader: &mut R) -> std::io::Result<Vec<i64>> {
    let array_length: usize = reader.read_u32::<LittleEndian>().unwrap() as usize;
    let mut buffer: Vec<i64> = vec![0i64; array_length];

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