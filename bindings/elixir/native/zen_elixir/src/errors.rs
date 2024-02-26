use std::{error::Error, fmt::Display};

#[derive(Debug, Clone)]
pub enum TypeEncodingError {
    InvalidBinary,
    InvalidAtom,
    InvalidInteger,
    InvalidFloat,
    InvalidArray,
    InvalidArrayItem,
    InvalidTuple,
    InvalidTupleItem,
    UnsupportedType,
    InvalidMap,
    InvalidMapItem,
    InvalidBoolean,
}

impl Error for TypeEncodingError {}

impl Display for TypeEncodingError {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
