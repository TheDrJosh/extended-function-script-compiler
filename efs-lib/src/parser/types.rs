use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    Integer,
    Float,
    String,
    Bool,
    Struct(String),
    NBT(NBTType),
}

#[derive(Clone, Debug, PartialEq)]
pub enum NBTType {
    NBTByte,
    NBTShort,
    NBTInt,
    NBTLong,
    NBTFloat,
    NBTDouble,
    NBTString,
    NBTList(Box<Type>),
    NBTCompound,
    NBTByteArray,
    NBTIntArray,
    NBTLongArray,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ValueType {
    Integer(i64),
    Float(f64),
    String(String),
    Struct(HashMap<String, ValueType>),
    NBT(NBTValue),
}

#[derive(Clone, Debug, PartialEq)]
pub enum NBTValue {
    NBTByte(i8),
    NBTShort(i16),
    NBTInt(i32),
    NBTLong(i64),
    NBTFloat(f32),
    NBTDouble(f64),
    NBTString(String),
    NBTList(NBTList),
    NBTCompound(HashMap<String, ValueType>),
    NBTByteArray(Vec<i8>),
    NBTIntArray(Vec<i32>),
    NBTLongArray(Vec<i64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum NBTList {
    Byte(Vec<i8>),
    Short(Vec<i16>),
    Int(Vec<i32>),
    Long(Vec<i64>),
    Float(Vec<f32>),
    Double(Vec<f64>),
    String(Vec<String>),
    List(Vec<Box<Type>>),
    Compound(Vec<HashMap<String, ValueType>>),
    ByteArray(Vec<Vec<i8>>),
    IntArray(Vec<Vec<i32>>),
    LongArray(Vec<Vec<i64>>),
}