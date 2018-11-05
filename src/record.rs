extern crate byteorder;

use std::collections::HashMap;
use std::f64;

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
    String(String),
    Array(Box<Value>),
    Dict(Box<Value>, Box<Value>),
    Set(Box<Value>),
    Custom(Box<Value>),
    Record(Box<Value>),
    Retired(u8),
}

#[inline]
fn field_length(field: Value) -> usize {
    match field {
        // Sorted by expected frequency of use
        Value::String(_) => 8,
        Value::Int(_) => 8,
        Value::Array(_) => 8,
        Value::Record(_) => 8,
        Value::Float(_) => 8,
        Value::Custom(_) => 8,
        Value::Dict(_, _) => 8,
        Value::Set(_) => 8,
        Value::Byte(_) => 1,
        Value::Int32(_) => 4,
        Value::UInt64(_) => 8,
        Value::UInt32(_) => 4,
        Value::Int16(_) => 2,
        Value::UInt16(_) => 2,
        Value::Float32(_) => 4,
        Value::Retired(bytes) => bytes as usize,
    }
}

// TODO how can we make this a thing
pub type FieldId = u64;

pub fn encode(fields: HashMap<FieldId, (u8, Value)>) -> Vec<u8> {
    let num_fields: usize = fields.len();

    if num_fields == 0 {
        return vec![];
    }

    // Each byte can store presence for 8 fields. Round up to the nearest byte.
    let presence_table_bytes: usize = f64::ceil((num_fields as f64) / 8.0) as usize;

    // Calculate the total bytes we'll need to represent the message.
    // This will be tricky and require multiple passes, to pack it efficiently!
    let mut total_bytes: usize = 0;

    for field in fields {
        total_bytes += field_length(field);
    }

    let mut result: Vec<u8> = vec![0; total_bytes];

    result
}
