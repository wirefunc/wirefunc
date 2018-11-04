extern crate byteorder;

/// There are two types of pointers:
///
/// 1. Segment Pointers
/// 2. Local Pointers
///
/// The last bit (out of 64) indicates which one we're dealing with.
///
/// If it's a segment pointer, the first 31 bits indicate which segment it points
/// to. The remaining 32 bits indicate the offset within that segment.
///
/// If it's a local pointer, the remaining 63 bits are type-specific. Their
/// layout depends on whether it's an Array, Record, Dict, Set, or Variant.
/// The generated code knows its type, and will process the layout accordingly.

static IS_SEGMENT_POINTER: &'static u64 = &0b1;

#[inline]
fn is_local_pointer(word: &u64) -> bool {
    *word & *IS_SEGMENT_POINTER == 0
}

pub fn to_segment_pointer(word: &u64) -> Option<SegmentPointer> {
    if is_local_pointer(word) {
        None
    } else {
        Some(SegmentPointer {
            // shift 1 to drop the is_local bit
            // then truncate to 32 bits to get the offset
            offset: (*word >> 1) as u32,

            // shift 1 to drop the is_local bit
            // then shift another 32 to skip the offset
            // then truncate to 32 bits to get the segment
            segment: (*word >> 33) as u32,
        })
    }
}

#[inline]
pub fn encode_segment_pointer(ptr: &SegmentPointer) -> u64 {
    let offset: u64 = (*ptr).offset as u64;
    let segment: u64 = (*ptr).segment as u64;

    IS_SEGMENT_POINTER | (offset << 1) | (segment << 33)
}

#[derive(PartialEq, Debug)]
pub struct SegmentPointer {
    pub segment: u32,
    pub offset: u32,
}
