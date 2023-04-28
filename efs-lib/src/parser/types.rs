use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub enum EFSType {
    Number,
    Byte,
    Short,
    Int,
    Long,
    Float,
    Double,
    String,
    Bool,
    Struct(String),
    List(Box<EFSType>),
    Dict,
    NBTByteArray,
    NBTIntArray,
    NBTLongArray,
    None,
}


#[derive(Clone, Debug, PartialEq)]
pub enum EFSValueType {
    Number(i32),
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    String(String),
    Bool(bool),
    Struct(String, HashMap<String, EFSValueType>),
    List(Vec<EFSValueType>),
    Dict(HashMap<String, EFSValueType>),
    NBTByteArray(Vec<i8>),
    NBTIntArray(Vec<i32>),
    NBTLongArray(Vec<i64>),
    None,
}
