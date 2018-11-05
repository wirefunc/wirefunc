/// The beginning of each segment is an u64 representing
/// the length (in Words) of the segment.
///
/// The very first segment of every message contains only
/// a Record, so this describes the length of that record.
/// A single record's data may not exceed one u32 worth of
/// Words in length, so for this very first segment length
/// value we don't actually need the entire u64.
///
/// In other segments, the segment typically holds data
/// for multiple values, not just a single record.
