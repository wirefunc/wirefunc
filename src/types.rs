#[derive(Debug)]
pub enum Tipe {
    Int32,
    Int64,
    Int8,
    Int16,
    Float32,
    Float64,
    Float,
    Int,
    String,
    Array(Box<Tipe>),
    Dict(Box<Tipe>, Box<Tipe>),
    Set(Box<Tipe>),
    Record,
    Custom,
}

#[derive(Debug)]
pub enum Formerly {
    Renamed(String),
    Retyped(Tipe),
    RenamedAndRetyped(String, Tipe),
}

#[derive(Debug)]
pub struct Field {
    tipe: Tipe,
    fallback: bool,
    formerly: [Formerly],
}
