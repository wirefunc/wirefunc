#[derive(Debug)]
pub struct Type {
    current: TypePrimitive,
    formerly: Vec<TypePrimitive>,
}

#[derive(Debug)]
pub enum TypePrimitive {
    Int,
    Int8,
    Int16,
    Int32,
    Int64,
    Float,
    Float32,
    Float64,
    String,
    Array(Box<Type>),
    Dict(Box<Type>, Box<Type>),
    Set(Box<Type>),
    Record(RecordInfo),
    Custom(CustomTypeInfo),
    Enum(EnumInfo),
    Param(TypeParam),
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
pub struct EnumInfo {
    name: TypeName,
    formerNames: Vec<TypeName>,
    variants: Vec<EnumVariant>,
}

#[derive(Debug, PartialEq)]
pub struct EnumVariant {
    optionality: Optionality,
    name: TypeName,
    value: i32,
    formerNames: Vec<String>,
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
    fallback: bool,
    formerNames: Vec<String>,
    contents: Vec<Type>,
}
