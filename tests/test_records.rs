extern crate byteorder;
extern crate wf;

use std::f64;
use std::slice;
use std::u64;
use wf::pointer;
use wf::pointer::Pointer;
use wf::record::FieldId;

// TODO test the case where FieldId order is wildly different from slot
// order. To do this, we'll need to load all the presence tables first,
// before doing anything else.

// TODO check out this approach: https://doc.rust-lang.org/std/primitive.slice.html#method.copy_from_slice
//      if that doesn't work, maybe try this approach: https://doc.rust-lang.org/nightly/std/ptr/fn.copy_nonoverlapping.html
#[cfg(test)]
mod primitive_fields_test {
    use byteorder::{ByteOrder, LittleEndian};
    use wf::record;
    // fn fields() -> HashMap<u32, Value> {
    //     hashmap!{
    //         3 => Value::Int(204),
    //         5 => Value::UInt64(114),
    //         7 => Value::Int32(104),
    //         9 => Value::UInt32(11),
    //         11 => Value::Int16(23),
    //         32 => Value::UInt16(42),
    //         54 => Value::Byte(7),
    //         55 => Value::Float(2.8),
    //         57 => Value::Float32(3.14),
    //         61 => Value::Bool(true),
    //         62 => Value::Bool(false),
    //     }
    // }

    #[derive(PartialEq, Debug)]
    struct Record {
        i64: i64,
        u64: u64,
        bool_true: bool,
        bool_false: bool,
    }

    fn decode<'a>(buffer: &'a [u8]) -> Record {
        let presence_table: &[u8] = record::decode_presence_table(buffer);
        let presence_table_page = presence_table[0];

        let i64 = if presence_table_page & 0b1000_0000 != 0 {
            LittleEndian::read_i64(&buffer[0..64])
        } else {
            0
        };

        let u64 = if presence_table_page & 0b0100_0000 != 0 {
            LittleEndian::read_u64(&buffer[64..128])
        } else {
            0
        };

        let bool_true = if presence_table_page & 0b0010_0000 != 0 {
            buffer[191] != 0
        } else {
            false
        };

        let bool_false = if presence_table_page & 0b0010_0000 != 0 {
            buffer[255] != 0
        } else {
            false
        };

        Record {
            i64: i64,
            u64: u64,
            bool_true: bool_true,
            bool_false: bool_false,
        }
    }

    fn encode(record: &Record, buffer: &mut [u8]) -> () {
        //         3 => Value::Int(204),
        //         5 => Value::UInt64(114),
        //         7 => Value::Int32(104),
        //         9 => Value::UInt32(11),
        //         11 => Value::Int16(23),
        //         32 => Value::UInt16(42),
        //         54 => Value::Byte(7),
        //         55 => Value::Float(2.8),
        //         57 => Value::Float32(3.14),
        //         61 => Value::Bool(true),
        //         62 => Value::Bool(false),
        LittleEndian::write_i64(&mut buffer[0..64], record.i64);
        LittleEndian::write_u64(&mut buffer[64..128], record.u64);
        LittleEndian::write_u64(&mut buffer[128..192], record.bool_true as u64);
        LittleEndian::write_u64(&mut buffer[192..256], record.bool_false as u64);
    }

    #[test]
    fn encode_decode_reflexive() {
        let record: Record = Record {
            i64: 1,
            u64: 2,
            bool_true: true,
            bool_false: false,
        };
        let mut buffer = [0; 11 * 8];

        encode(&record, &mut buffer);

        assert_eq!(record, decode(&buffer));
    }
}

fn encode_fields<'a>(fields: Vec<(FieldId, Value)>, highest_field_id: FieldId) -> &'a [u8] {
    let num_fields: usize = fields.len();

    if num_fields == 0 {
        let empty: Vec<u8> = vec![];

        // TODO: Figure out a non-unsafe way to return a fresh, empty slice.
        unsafe {
            return slice::from_raw_parts(empty.as_ptr() as *const _, 0);
        }
    }

    // Each Word can store presence for 64 fields. Round up to the nearest Word.
    let presence_table_words: usize = f64::ceil(((highest_field_id + 1) as f64) / 64.0) as usize;
    let total_words = num_fields + presence_table_words;

    let mut result: Vec<u64> = vec![0; total_words];
    let mut index: usize = 0;
    let mut presence_table_index: usize = 0;

    // These have already been sorted by word index, so we can iterate through
    // them directly.
    for (field_id, field) in fields {
        let field_id_usize = field_id as usize;

        // Write the value to the appropriate place in the result words.
        result[index] = encode_field(field);
        index += 1;

        // Advance presence_table_index to the current field_id.
        // We'll leave everything else as zeroes.
        while (presence_table_index + 1) * 64 <= field_id_usize {
            presence_table_index += 1;
        }

        // TODO If we need multiple presence table pages, write a 1 at the end
        // of each of them.

        // TODO write the presence table at the start of the segment, not the end!

        // Set a bit to 1 in the presence table for this field_id.
        let presence_table_index_in_result = total_words - presence_table_index - 1;
        let presence_table = result[presence_table_index_in_result];
        // TODO this math is wrong because it doesn't account for the 64th bit
        // in the presence table being responsible for telling if there are more pages
        let presence_table_bit_index: i64 = 64 * presence_table_index as i64;
        let field_id_offset: i64 = field_id_usize as i64 - presence_table_bit_index;

        result[presence_table_index_in_result] = presence_table | (1 << field_id_offset);
    }

    // Now that we're done working in terms of u64, cast the vector to &[u8].
    unsafe { slice::from_raw_parts(result.as_mut_ptr() as *const _, result.len() * 8) }
}

#[test]
fn test_encode_record() {
    // A record with a single boolean field @0
    let actual = encode_fields(vec![(0, Value::Bool(true))], 0);
    let expected: &[u8] = &vec![
        // The boolean field - all 1s, indicating True
        0b1111_1111,
        0b1111_1111,
        0b1111_1111,
        0b1111_1111,
        0b1111_1111,
        0b1111_1111,
        0b1111_1111,
        0b1111_1111,
        // The presence table - only the final bit is set
        0b0000_0000,
        0b0000_0000,
        0b0000_0000,
        0b0000_0000,
        0b0000_0000,
        0b0000_0000,
        0b0000_0000,
        0b0000_0001,
    ];

    assert_eq!(expected, actual);
}

#[derive(PartialEq, Debug)]
pub enum Value {
    Int(i64),
    UInt64(u64),
    Int32(i32),
    UInt32(u32),
    Int16(i16),
    UInt16(u16),
    Byte(u8),
    Float(f64),
    Float32(f32),
    Bool(bool),
    String(Pointer),
    Array(Pointer),
    Dict(Pointer),
    Set(Pointer),
    Custom(Pointer),
    Record(Pointer),
}

fn encode_field(field: Value) -> u64 {
    match field {
        Value::Int(inner) => inner as u64,
        Value::UInt64(inner) => inner as u64,
        Value::Int32(inner) => inner as u64,
        Value::UInt32(inner) => inner as u64,
        Value::Int16(inner) => inner as u64,
        Value::UInt16(inner) => inner as u64,
        Value::Byte(inner) => inner as u64,
        Value::Float(inner) => inner as u64,
        Value::Float32(inner) => inner as u64,
        Value::Bool(inner) => inner as u64,
        Value::String(ptr) => pointer::encode(ptr),
        Value::Array(ptr) => pointer::encode(ptr),
        Value::Dict(ptr) => pointer::encode(ptr),
        Value::Set(ptr) => pointer::encode(ptr),
        Value::Custom(ptr) => pointer::encode(ptr),
        Value::Record(ptr) => pointer::encode(ptr),
    }
}
