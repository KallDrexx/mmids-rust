use std::fmt;
use std::io;
use std::error::Error;
use std::string;

#[derive(Debug)]
pub enum Amf0DeserializationError {
    UnknownMarker(u8),
    UnexpectedEmptyObjectPropertyName,
    UnexpectedEof,
    Io(io::Error),
    FromUtf8Error(string::FromUtf8Error)
}

impl fmt::Display for Amf0DeserializationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Amf0DeserializationError::Io(ref err) => err.fmt(f),
            Amf0DeserializationError::FromUtf8Error(ref err) => err.fmt(f),
            Amf0DeserializationError::UnknownMarker(ref x) => write!(f, "Marker byte of {} is not known", x),
            Amf0DeserializationError::UnexpectedEmptyObjectPropertyName => write!(f, "Unexpected empty object property name"),
            Amf0DeserializationError::UnexpectedEof => write!(f, "Hit end of the byte buffer but was expecting more data"),
        }
    }
}

impl Error for Amf0DeserializationError {
    fn description(&self) -> &str {
        match *self {
            Amf0DeserializationError::Io(ref err) => err.description(),
            Amf0DeserializationError::UnknownMarker(_) => "Unknown Marker",
            Amf0DeserializationError::FromUtf8Error(ref err) => err.description(),
            Amf0DeserializationError::UnexpectedEmptyObjectPropertyName => "Unexpected empty object property name",
            Amf0DeserializationError::UnexpectedEof => "Hit end of the byte buffer but was expecting more data",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            Amf0DeserializationError::Io(ref err) => Some(err),
            Amf0DeserializationError::FromUtf8Error(ref err) => Some(err),
            Amf0DeserializationError::UnknownMarker(_) => None,
            Amf0DeserializationError::UnexpectedEmptyObjectPropertyName => None,
            Amf0DeserializationError::UnexpectedEof => None,
        }
    }
}

impl From<io::Error> for Amf0DeserializationError {
    fn from(err: io::Error) -> Amf0DeserializationError {
        Amf0DeserializationError::Io(err)
    }
}

impl From<string::FromUtf8Error> for Amf0DeserializationError {
    fn from(err: string::FromUtf8Error) -> Amf0DeserializationError {
        Amf0DeserializationError::FromUtf8Error(err)
    }
}