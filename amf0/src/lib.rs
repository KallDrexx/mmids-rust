extern crate byteorder;

pub mod serialization;
pub mod deserialization;
pub mod errors;

use std::collections::HashMap;

#[derive(PartialEq, Debug)]
pub enum Amf0Value {
    Number(f64),
    Boolean(bool),
    Utf8String(String),
    Object(Amf0Object),
    Null,
}

#[derive(PartialEq, Debug)]
pub struct Amf0Object {
    properties: HashMap<String, Amf0Value>
}

mod markers {
    pub const NUMBER_MARKER: u8 = 0;
    pub const BOOLEAN_MARKER: u8 = 1;
    pub const STRING_MARKER: u8 = 2;
    pub const OBJECT_MARKER: u8 = 3;
    pub const NULL_MARKER: u8 = 5; 
    pub const OBJECT_END_MARKER: u8 = 9;
    pub const UTF_8_EMPTY_MARKER: u16 = 0;
}