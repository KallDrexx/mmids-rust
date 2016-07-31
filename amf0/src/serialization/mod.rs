//! Module contains functionality for serializing values into an
//! bytes based on the AMF0 specification 
//! (http://wwwimages.adobe.com/content/dam/Adobe/en/devnet/amf/pdf/amf0-file-format-specification.pdf)

use {Amf0Value, Amf0Object};
use std::io::Error;
use byteorder::{BigEndian, WriteBytesExt};
use markers;

/// Serializes values into an amf0 encoded vector of bytes
pub fn serialize(values: &Vec<Amf0Value>) -> Result<Vec<u8>, Error> {
    let mut bytes = vec![];
    for value in values {
        try!(serialize_value(value, &mut bytes));
    }

    Ok(bytes)
}

fn serialize_value(value: &Amf0Value, bytes: &mut Vec<u8>) -> Result<(), Error> {
    match *value {
        Amf0Value::Boolean(ref val) => Ok(serialize_bool(&val, bytes)),
        Amf0Value::Null => Ok(serialize_null(bytes)),
        Amf0Value::Number(ref val) => serialize_number(&val, bytes),
        Amf0Value::Utf8String(ref val) => serialize_string(&val, bytes),
        Amf0Value::Object(ref val) => serialize_object(&val, bytes)
    }
}

fn serialize_number(value: &f64, bytes: &mut Vec<u8>) -> Result<(), Error> {
    bytes.push(markers::NUMBER_MARKER);
    try!(bytes.write_f64::<BigEndian>(value.clone()));
    Ok(())    
}

fn serialize_bool(value: &bool, bytes: &mut Vec<u8>) {
    bytes.push(markers::BOOLEAN_MARKER);
    bytes.push((value.clone()) as u8);
}

fn serialize_string(value: &String, bytes: &mut Vec<u8>) -> Result<(), Error> {
    // TODO: add check if length is > u16
    bytes.push(markers::STRING_MARKER);
    try!(bytes.write_u16::<BigEndian>(value.len() as u16));
    bytes.extend(value.as_bytes());
    Ok(())
}

fn serialize_null(bytes: &mut Vec<u8>) {
    bytes.push(markers::NULL_MARKER);
}

fn serialize_object(object: &Amf0Object, bytes: &mut Vec<u8>) -> Result<(), Error> {
    bytes.push(markers::OBJECT_MARKER);

    for (name, value) in &object.properties {
        // TODO: Add check that property name isn't greater than a u16
        try!(bytes.write_u16::<BigEndian>(name.len() as u16));
        bytes.extend(name.as_bytes());
        try!(serialize_value(&value, bytes));
    }

    try!(bytes.write_u16::<BigEndian>(markers::UTF_8_EMPTY_MARKER));
    bytes.push(markers::OBJECT_END_MARKER);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::serialize;
    use super::super::Amf0Value;
    use super::super::Amf0Object;
    use markers;
    use byteorder::{BigEndian, WriteBytesExt};
    use std::collections::HashMap;

    #[test]
    fn can_serialize_number() {
        let number: f64 = 332.0;
       
        let input = vec![Amf0Value::Number(number)];
        let result = serialize(&input).unwrap();

        let mut expected = vec![];
        expected.write_u8(markers::NUMBER_MARKER).unwrap();
        expected.write_f64::<BigEndian>(number).unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn can_serialize_true_boolean() {
        let input = vec![Amf0Value::Boolean(true)];
        let result = serialize(&input).unwrap();

        let mut expected = vec![];
        expected.write_u8(markers::BOOLEAN_MARKER).unwrap();
        expected.write_u8(1).unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn can_serialize_false_boolean() {
        let input = vec![Amf0Value::Boolean(false)];
        let result = serialize(&input).unwrap();

        let mut expected = vec![];
        expected.write_u8(markers::BOOLEAN_MARKER).unwrap();
        expected.write_u8(0).unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn can_serialize_string() {
        let value = "test";

        let input = vec![Amf0Value::Utf8String(value.to_string())];
        let result = serialize(&input).unwrap();

        let mut expected = vec![];
        expected.write_u8(markers::STRING_MARKER).unwrap();
        expected.write_u16::<BigEndian>(value.len() as u16).unwrap();
        expected.extend(value.as_bytes());

        assert_eq!(result, expected);
    }

    #[test]
    fn can_serialize_null() {
        let input = vec![Amf0Value::Null];
        let result = serialize(&input).unwrap();

        let mut expected = vec![];
        expected.write_u8(markers::NULL_MARKER).unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn can_serialize_object() {
        const NUMBER: f64 = 332.0;

        let mut properties = HashMap::new();
        properties.insert("test".to_string(), Amf0Value::Number(NUMBER));

        let object = Amf0Object {properties: properties};
        let input = vec![Amf0Value::Object(object)];
        let result = serialize(&input).unwrap();

        let mut expected = vec![];
        expected.push(markers::OBJECT_MARKER);
        expected.write_u16::<BigEndian>(4).unwrap();
        expected.extend("test".as_bytes());
        expected.push(markers::NUMBER_MARKER);
        expected.write_f64::<BigEndian>(NUMBER).unwrap();
        expected.write_u16::<BigEndian>(markers::UTF_8_EMPTY_MARKER).unwrap();;
        expected.push(markers::OBJECT_END_MARKER);

        assert_eq!(result, expected);
    }
}