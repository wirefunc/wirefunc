#[derive(Debug)]
#[allow(dead_code)]
pub struct Type {
    current: TypePrimitive,
    formerly: Vec<TypePrimitive>,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum TypePrimitive {
    // scalars
    Bool,
    String,
    Int,
    Int8,
    Int16,
    Int32,
    Int64,
    Float,
    Float32,
    Float64,

    // iterable collections
    Array(Box<Type>),
    Dict(Box<Type>, Box<Type>),
    Set(Box<Type>),

    // custom types
    Custom(CustomTypeInfo),

    // records
    Record(RecordInfo),
}

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum Optionality {
    Required,
    Optional,
}

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum TypeParam {
    TypeParam(String),
}

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum TypeName {
    TypeName(String),
}

#[derive(Debug)]
pub struct RecordInfo {
    name: Option<TypeName>,
    former_names: Vec<TypeName>,
    fields: Vec<Field>,
    params: Vec<TypeParam>,
}

#[derive(Debug)]
pub struct CustomTypeInfo {
    name: TypeName,
    former_names: Vec<TypeName>,
    variants: Vec<Variant>,
    params: Vec<TypeParam>,
}

/// Use a 64-bit integer here for mmap alignment.
/// In JSON it'll be represented as a string anyway!
pub type FieldId = u64;

#[derive(Debug)]
pub struct Field {
    optionality: Optionality,
    name: String,
    tipe: Type,
    field_id: FieldId,
    fallback: bool,
    former_names: Vec<String>,
}

/// Use a 64-bit integer here for mmap alignment.
/// In JSON it'll be represented as a string anyway!
pub type VariantId = u64;

#[derive(Debug)]
pub struct Variant {
    optionality: Optionality,
    name: String,
    tipe: Type,
    variant_id: VariantId,
    former_names: Vec<String>,
    contents: Vec<Type>,
}
