extern crate byteorder;

/// There are three types of pointers:
///
/// 1. Segment Pointers
/// 2. Composite Pointers
///
/// If the last bit (out of 64) is a 0, we're dealing with a Segment Pointer.
/// Otherwise it's a Composite Pointer.
///
/// A Segment Pointer is laid out like this:
///
/// |--Index within Segment--|--Segment--|--Data Length--|--Composite?--|
///          32 bits            16 bits       15 bits          1 bit
///
/// The reason "length within the segment" is part of the Segment Pointer is
/// that everything we can point to has a variable length that is not statically
/// known: Arrays, Dicts, Sets, Variants, Strings, Bytes, and Records.
///
/// > Records have variable lengths that are not statically known because older
/// > senders and receivers may not have the same idea of how many fields the
/// > record has, or what their types are.
///
/// Having the data length inside the pointer lets us avoid performing a pointer
/// dereference for empty collections. For example, if it's an empty Array or
/// an empty String, we can see that right in the Segment Pointer and not bother
/// hopping to another Segment to find that out.
///
/// The data length value is not in Words. Rather, its units depend on the type
/// of value we're pointing to. In the case of a Record, it is in Words.
/// In the case of an Array, it is the number of elements in the Array. In the
/// case of a String, it is the byte length of the String, so we know how many
/// bytes to read into memory for it. (This may not be the same as the character
/// count of the String, since it will be UTF-8 encoded!)
///
/// Why these numbers?
///
/// * 32 bits of (64-bit) Words for the index within the segment lets us index
///   into ~32GB worth of single-Word values in a single segment. That's plenty!
/// * 16 bits of segment ID offset lets us address into the next 65,536 segments.
///   In practice, one segment will rarely need to index more than a couple
///   segments into the future. So this is extremely conservative, and if you
///   somehow run out, the only consequence is that the remainder of the message
///   gets stuck into the final segment, which could be uncomfortably large.
/// * 15 bits of data length lets us store Arrays and Strings of up to 32,768 in
///   length before we need a composite pointer. It's okay if this is not very
///   long, because composite pointers give us theoretically infinite runway.
///   The only consequence to shortening this value is that values that don't
///   fit in this length need to use composite pointers, which are a bit slower.
///
/// A Composite Pointer stitches together multiple pointers to deal with very
/// large values - like an Array with billions of elements, or a really long
/// String, a huge binary blob, etc.
///
/// Each Composite Pointer is essentially an `Array Pointer`. Those pointers
/// can be either Segment Pointers or more Composite Pointers, meaning you can
/// use nested Composite Pointers to represent arbitrarily long values.
///
/// Ultimately, every pointer in a Composite Pointer must resolve to the same
/// type. You can't end up with a mix of Arrays and Sets, for example. This is
/// because once they're all resolved, the decoder must merge the resulting
/// values together—concatenating arrays and strings, merging Sets and record
/// fields, etc.
///
/// On the wire, a Composite Pointer is laid out the same as a Segment Pointer:
///
/// |--Index within Segment--|--Segment--|--Data Length--|--Composite?--|
///          32 bits            16 bits       15 bits          1 bit
///
/// The difference is that for a Composite Pointer, the final bit will be 1
/// instead of 0, and Data Length refers to the number of pointers in the array.

/// Layout on the wire:
///
/// |--Index within Segment--|--Segment--|--Data Length--|--Composite?--|
///          32 bits            16 bits       15 bits          1 bit
#[derive(PartialEq, Debug)]
pub struct Pointer {
    /// Index within the segment, in Words
    pub word_index: u32,

    /// Segment ID offset. Zero means use the current segment.
    pub segment_id_offset: u16,

    /// Length of data within the segment, in whatever units are appropriate for
    /// the data's type. For Composite Pointers, the data's type is Pointer.
    /// In all other cases, the data's type is hardcoded in generated code.
    pub length: u16,

    /// Type
    pub is_composite: bool,
}

#[inline]
pub fn decode(word: u64) -> Pointer {
    Pointer {
        // Shift right 32 bits to drop the other fields.
        // Then truncate to 32 bits to get the word index within the segment.
        word_index: (word >> 32) as u32,

        // Shift right 16 bits to drop the length (15 bits) and type (1 bit).
        // Then truncate to 16 bits to get the segment ID offset.
        segment_id_offset: (word >> 16) as u16,

        // Shift right 1 bit to drop the type bit.
        // Then truncate to 16 bits to get the length.
        // Then zero out the first bit because that one's for segment_id_offset.
        length: ((word >> 1) as u16) & 0b0111_1111_1111_1111,

        /// Check if the final bit in the Word is 1, indicating
        /// that this is a Composite Pointer.
        is_composite: word & 0b1 != 0,
    }
}

#[inline]
pub fn encode(ptr: Pointer) -> u64 {
    // Convert the pointer's integer fields to u64s so we can do bitwise
    // operations on them.
    let word_index: u64 = ptr.word_index as u64;
    let length: u64 = ptr.length as u64;
    let segment_id_offset: u64 = ptr.segment_id_offset as u64;

    // |--Index within Segment--|--Segment--|--Data Length--|--Composite?--|
    //          32 bits            16 bits       15 bits          1 bit
    let result = (word_index << 32) | (segment_id_offset << 16) | (length << 1);

    // Set the bit flag at the end if it's composite. Otherwise, do nothing;
    // by default the bit flag will already be set to 0. (This is a good default
    // because composite pointers should be rare in practice.)
    if ptr.is_composite {
        0b1 | result
    } else {
        result
    }
}
