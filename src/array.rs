extern crate byteorder;

use pointer;
use pointer::Pointer;
use std::u16;

pub fn encode_u64_array(word_index: u32, segment_id_offset: u16, arr: &[u64]) -> u64 {
    // Since `length` will be cast into a u16, this function will work fine as
    // long as usize is u16 or bigger. In other words, it works on 16-bit
    // systems and higher.  Anyone trying to run WireFunc on an 8-bit system is
    // (A) super hardcore and (B) out of luck.
    let length: usize = arr.len();

    if length == 0 {
        // It's empty; bail out early. Decoders know to special-case this!
        return 0;
    } else if length > (u16::max_value() as usize) {
        // It's too long! Make a Composite Pointer
        panic!("Composite Pointers aren't supported yet!");
    } else {
        // Make a regular Segment Pointer
        pointer::encode(Pointer {
            word_index: word_index,
            segment_id_offset: segment_id_offset,
            length: length as u16,
            is_composite: false,
        })
    }
}

pub fn decode_u64_array(raw_ptr: u64) -> Vec<u64> {
    if raw_ptr == 0 {
        // If the whole pointer is zeroes, it's an empty vector. Bail out!
        return vec![];
    } else {
        let _ptr: Pointer = pointer::decode(raw_ptr);

        panic!("We don't know how to decode arrays yet!");
    }
}
