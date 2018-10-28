#[derive(Debug)]
pub struct Type {
    current: TypePrimitive,
    formerly: Vec<TypePrimitive>,
}

#[derive(Debug)]
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
pub enum Optionality {
    Required,
    Optional,
}

#[derive(Debug, PartialEq)]
pub enum TypeParam {
    TypeParam(String),
}

#[derive(Debug, PartialEq)]
pub enum TypeName {
    TypeName(String),
}

#[derive(Debug)]
pub struct RecordInfo {
    name: Option<TypeName>,
    formerNames: Vec<TypeName>,
    fields: Vec<Field>,
    params: Vec<TypeParam>,
}

#[derive(Debug)]
pub struct CustomTypeInfo {
    name: TypeName,
    formerNames: Vec<TypeName>,
    variants: Vec<Variant>,
    params: Vec<TypeParam>,
}

#[derive(Debug)]
pub struct Field {
    optionality: Optionality,
    name: String,
    tipe: Type,
    fieldId: i32,
    fallback: bool,
    formerNames: Vec<String>,
}

#[derive(Debug)]
pub struct Variant {
    optionality: Optionality,
    name: String,
    tipe: Type,
    variantId: i32,
    formerNames: Vec<String>,
    contents: Vec<Type>,
}
