extern crate byteorder;

use std::collections::HashMap;
pub mod serialization;

pub enum Amf0Value {
    Number(f64),
    Boolean(bool),
    Utf8String(String),
    Object(Amf0Object),
    Null,
}

pub struct Amf0Object {
    properties: HashMap<String, Amf0Value>
}