extern crate byteorder;

// TODO should make this pub struct
pub type FieldId = u32;

/// Read the field presence table from a record's data bytes.
///
/// * The presence table is always at the beginning of a record's data bytes.
/// * Each page of the presence table is 64 bits long.
/// * The 64th bit in a page is 1 iff there's another page after it.
///
/// This function checks the 64th bit of consecutive pages until it hits a 0,
/// at which point it returns the subslice of the original byte range that
/// holds the properly sized presence table.
///
/// ## Panics
///
/// This function does no bounds checking. So if the given byte range is fewer
/// than 8 bytes in length, this will try to read outside its range and panic.
///
/// Similarly, if its 64th bit is 1, and there are not at least 8 bytes
/// following that 64th bit, it will try to read outside its range and panic.
#[inline]
pub fn decode_presence_table(bytes: &[u8]) -> &[u8] {
    let mut presence_table_length = 1;

    // If the last bit in the (64-bit) presence table page is 1, then there is
    // another page to this presence table! We need to keep reading.
    while bytes[(presence_table_length * 8) - 1] & 0b1 != 0 {
        presence_table_length += 1;
    }

    &bytes[0..presence_table_length]
}
