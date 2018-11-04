extern crate byteorder;

/// There are two types of pointers:
///
/// 1. Segment Pointers
/// 2. Local Pointers
///
/// The last bit (out of 64) indicates which one we're dealing with. If it's
/// a 1, we're dealing with a Segment Pointer. This will be important later!
///
/// If it's a Local Pointer, the remaining 63 bits are type-specific. Their
/// layout depends on whether it's an Array, Record, Dict, Set, or Variant.
/// The generated code knows its type, and will process the layout accordingly.
///
/// A Segment Pointer is laid out like this:
///
/// |---Offset---|---Length---|-Segment-|
///    24 bits      24 bits     16 bits
///
/// [O] Offset within the segment, in Words.
/// [L] Length of data within the segment, in Words.
/// [S] Segment ID offset. Must be at least 1, so we always move forward.
///
/// The segment ID offset must always be at least 1. If a pointer is pointing to
/// data within the current segment, then it should be a Local Pointer, not a
/// Segment Pointer!
///
/// This guarantee is automatically enforced thanks to the last bit of the
/// message being a 1 if this is a Segment Pointer. Since the Segment ID offset
/// overlaps this bit, it is impossible to have that bit set (indicating this
/// is a Segment Pointer) and also have a Segment ID offset of 0.
///
/// The reason "length within the segment" is part of the Segment Pointer is
/// that everything we can point to has a variable length. Arrays, Dicts, Sets,
/// Variants, Strings, Bytes, and Records all have runtime variable lengths.
///
/// > Records have runtime variable lengths because older senders and receivers
/// > may not have the same idea of how many fields the record has, or what
/// > their types are.
///
/// Having the data length inside the pointer lets us avoid performing a pointer
/// dereference for empty collections. For example, if it's an empty Array or
/// an empty String, we can see that right in the Segment Pointer and not bother
/// hopping to the next Segment to find that out.
///
/// 24 bits of (64-bit) Words lets us address about 130MB in a single segment.
/// 16 bits of segment ID offset lets us refer to segments that are up to 65,535
/// places ahead of the current segment.

/// Check if the final bit in the Word is 1, indicating
/// that this is a Segment Pointer and not a Local Pointer.
#[inline]
pub fn is_segment_pointer(word: u64) -> bool {
    word & 0b1 != 0
}

/// Layout on the wire:
///
/// |---Offset---|---Length---|-Segment-|
///    24 bits      24 bits     16 bits
#[derive(PartialEq, Debug)]
pub struct SegmentPointer {
    /// Offset within the segment, in Words
    pub offset: u32,

    /// Length of data within the segment, in Words
    pub length: u32,

    /// Segment ID offset
    pub segment: u16,
}

#[inline]
pub fn to_segment_pointer(word: u64) -> SegmentPointer {
    SegmentPointer {
        // Truncate to 16 bits to get the segment ID offset.
        // This will always be at least 1, because we will have already verified
        // that the final bit is a 1 to confirm that this is a Segment Pointer.
        segment: word as u16,

        // Shift right 16 bits to drop the segment ID offset.
        // Then truncate to 32 bits to get the length.
        // Then zero out the first 8 bits because those belong to the offset.
        length: ((word >> 16) as u32) & 0b0000_0000_1111_1111_1111_111_1111_11111,

        // Shift right 16 bits to drop the segment ID offset.
        // Then shift right 24 bits to drop the length.
        // (16 + 24 = 40, so combine those two steps into >> 40).
        // Then truncate to 32 bits to get the offset within the segment.
        offset: (word >> 40) as u32,
    }
}

#[inline]
pub fn encode_segment_pointer(ptr: SegmentPointer) -> u64 {
    let offset: u64 = ptr.offset as u64;
    let length: u64 = ptr.length as u64;
    let segment: u64 = ptr.segment as u64;

    // We shouldn't need to start this expression with…
    //
    // IS_SEGMENT_POINTER |
    //
    // …because offset must always be > 0 anyway.
    // This saves ~1 CPU instruction per encoded pointer.
    segment | (length << 16) | (offset << 40)
}
