// bencode subfolder and item enum implemenation
use std::fmt::{self, Display};

use serde::{de, ser};

pub mod decode;
pub mod encode;

#[derive(Debug)]
pub enum Error {
    Serde(String),
    Unimplemented,
    Overflow,
    Start,
    End,
    Integer,
    String,
    TrailingBytes,
    NoMatch,
    EndOfBytes,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        match self {
            Error::Serde(t) => f.write_str(&t),
            Error::Unimplemented => f.write_str("Primitive is unimplemented"),
            Error::Overflow => f.write_str("Integer overflow"),
            Error::Start => f.write_str("Can't start item here"),
            Error::End => f.write_str("Item doesn't end here"),
            Error::Integer => f.write_str("Integer parse error"),
            Error::String => f.write_str("String parse error"),
            Error::TrailingBytes => f.write_str("Input buffer has trailing bytes"),
            Error::NoMatch => f.write_str("Item doesn't match any Bencode types"),
            Error::EndOfBytes => f.write_str("Buffer prematurely ended"),
        }
    }
}

impl std::error::Error for Error {}

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Serde(msg.to_string())
    }
}
impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Serde(msg.to_string())
    }
}
