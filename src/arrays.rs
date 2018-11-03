extern crate byteorder;

/// In all cases, the first u64 in the bytes represents the length of the array.
/// Then each element in the array is packed back-to-back, right afterwards.
///
/// We have two types of arrays: primitive arrays and pointer arrays.
///
/// Primitive arrays: [f64], [f32], [i64], [u64], [i32], [u32], [i16], [u16], [i8], [u8], [bool]
/// Pointer arrays: [string], [bytes], [record], [custom]
///
/// Primitive arrays have different slot sizes based on their type.
/// Each element in a pointer array is 64 bits. It's a pointer to some data.
/// For that reason, decoding pointer arrays takes multiple passes. First we
/// decode all the pointers, then we follow them all to decode the rest.
///
/// Enums are based on the underlying type they wrap, which could be a
/// primitive like Int64 or a pointer like String.
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std;
use std::io;

#[inline]
fn read_length<R: io::Read>(reader: &mut R) -> std::io::Result<u64> {
    reader.read_u64::<LittleEndian>()
}

#[inline]
fn write_length(buffer: &mut Vec<u8>, length: u64) -> std::io::Result<()> {
    buffer.write_u64::<LittleEndian>(length)
}

/// Read an Array Float64
pub fn read_f64_array_from<R: io::Read>(reader: &mut R) -> std::io::Result<Vec<f64>> {
    let array_length: u64 = read_length(reader)?;
    let mut buffer: Vec<f64> = vec![0f64; array_length as usize];

    reader.read_f64_into::<LittleEndian>(&mut buffer)?;

    Ok(buffer)
}

/// Read an Array Float32
pub fn read_f32_array_from<R: io::Read>(reader: &mut R) -> std::io::Result<Vec<f32>> {
    let array_length: u64 = read_length(reader)?;
    let mut buffer: Vec<f32> = vec![0f32; array_length as usize];

    reader.read_f32_into::<LittleEndian>(&mut buffer)?;

    Ok(buffer)
}

/// Read an Array Int64
pub fn read_i64_array_from<R: io::Read>(reader: &mut R) -> std::io::Result<Vec<i64>> {
    let array_length: u64 = read_length(reader)?;
    let mut buffer: Vec<i64> = vec![0i64; array_length as usize];

    reader.read_i64_into::<LittleEndian>(&mut buffer)?;

    Ok(buffer)
}

/// Read an Array UInt64
pub fn read_u64_array_from<R: io::Read>(reader: &mut R) -> std::io::Result<Vec<u64>> {
    let array_length: u64 = read_length(reader)?;
    let mut buffer: Vec<u64> = vec![0u64; array_length as usize];

    reader.read_u64_into::<LittleEndian>(&mut buffer)?;

    Ok(buffer)
}

/// Read an Array Int32
pub fn read_i32_array_from<R: io::Read>(reader: &mut R) -> std::io::Result<Vec<i32>> {
    let array_length: u64 = read_length(reader)?;
    let mut buffer: Vec<i32> = vec![0i32; array_length as usize];

    reader.read_i32_into::<LittleEndian>(&mut buffer)?;

    Ok(buffer)
}

/// Read an Array UInt32
pub fn read_u32_array_from<R: io::Read>(reader: &mut R) -> std::io::Result<Vec<u32>> {
    let array_length: u64 = read_length(reader)?;
    let mut buffer: Vec<u32> = vec![0u32; array_length as usize];

    reader.read_u32_into::<LittleEndian>(&mut buffer)?;

    Ok(buffer)
}

/// Read an Array Int16
pub fn read_i16_array_from<R: io::Read>(reader: &mut R) -> std::io::Result<Vec<i16>> {
    let array_length: u64 = read_length(reader)?;
    let mut buffer: Vec<i16> = vec![0i16; array_length as usize];

    reader.read_i16_into::<LittleEndian>(&mut buffer)?;

    Ok(buffer)
}

/// Read an Array UInt16
pub fn read_u16_array_from<R: io::Read>(reader: &mut R) -> std::io::Result<Vec<u16>> {
    let array_length: u64 = read_length(reader)?;
    let mut buffer: Vec<u16> = vec![0u16; array_length as usize];

    reader.read_u16_into::<LittleEndian>(&mut buffer)?;

    Ok(buffer)
}

pub fn write_i64_array_to<W: io::Write>(writer: &mut W, arr: &[i64]) -> std::io::Result<()> {
    let length: u64 = arr.len() as u64;
    let mut buffer: Vec<u8> = vec![];

    write_length(&mut buffer, length)?;

    // Write the elements
    for elem in arr {
        buffer.write_i64::<LittleEndian>(*elem)?;
    }

    writer.write_all(&buffer)
}

pub fn write_i32_array_to(buffer: &mut Vec<u8>, arr: &[i32]) -> std::io::Result<()> {
    let length: u64 = arr.len() as u64;

    write_length(buffer, length)?;

    // Write the elements
    for elem in arr {
        buffer.write_i32::<LittleEndian>(*elem)?;
    }

    // If we have an odd number of 32-bit integers, pad the end with 32 zeroes
    // so we end on a multiple of 64, and reads will be Word-aligned.
    if length % 2 == 1 {
        buffer.write_i32::<LittleEndian>(0)?;
    }

    Ok(())
}

pub fn write_nested_i32_array_to<W: io::Write>(
    writer: &mut W,
    arr: &[&[i32]],
) -> std::io::Result<()> {
    let length: u64 = arr.len() as u64;
    let mut buffer: Vec<u8> = vec![];

    write_length(&mut buffer, length)?;

    // Write the elements
    for inner_arr in arr {
        write_i32_array_to(&mut buffer, &inner_arr)?;
    }

    writer.write_all(&buffer)
}
