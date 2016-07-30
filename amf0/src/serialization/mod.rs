//! Module contains functionality for serializing values into an
//! bytes based on the AMF0 specification 
//! (http://wwwimages.adobe.com/content/dam/Adobe/en/devnet/amf/pdf/amf0-file-format-specification.pdf)

use super::Amf0Value;
use byteorder::{BigEndian, WriteBytesExt};

const NUMBER_MARKER: u8 = 0;
const BOOLEAN_MARKER: u8 = 1;
const STRING_MARKER: u8 = 2;
const OBJECT_MARKER: u8 = 3;
const NULL_MARKER: u8 = 5; 

/// Serializes values into an amf0 encoded vector of bytes
pub fn serialize(values: &Vec<Amf0Value>) -> Vec<u8> {
    let mut result = vec![];
    for value in values {
        match *value {
            Amf0Value::Boolean(ref val) => serialize_bool(&val, &mut result),
            Amf0Value::Null => serialize_null(&mut result),
            Amf0Value::Number(ref val) => serialize_number(&val, &mut result),
            Amf0Value::Utf8String(ref val) => serialize_string(&val, &mut result),
            Amf0Value::Object(_) => panic!("Not Implemented")
        }
    }

    result
}

fn serialize_number(value: &f64, vector: &mut Vec<u8>) {
    vector.push(NUMBER_MARKER);
    vector.write_f64::<BigEndian>(value.clone()).unwrap();    
}

fn serialize_bool(value: &bool, vector: &mut Vec<u8>) {
    vector.push(BOOLEAN_MARKER);
    vector.push((value.clone()) as u8);
}

fn serialize_string(value: &String, vector: &mut Vec<u8>) {
    vector.push(STRING_MARKER);
    vector.write_u16::<BigEndian>(value.len() as u16).unwrap(); // TODO: add check if length is > u16
    vector.extend(value.as_bytes());
}

fn serialize_null(vector: &mut Vec<u8>) {
    vector.push(NULL_MARKER);
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::Amf0Value;
    use byteorder::{BigEndian, WriteBytesExt};

    #[test]
    fn can_serialize_number() {
        let number: f64 = 332.0;
       
        let input = vec![Amf0Value::Number(number)];
        let result = serialize(&input);

        let mut expected = vec![];
        expected.write_u8(super::NUMBER_MARKER).unwrap();
        expected.write_f64::<BigEndian>(number).unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn can_serialize_true_boolean() {
        let input = vec![Amf0Value::Boolean(true)];
        let result = serialize(&input);

        let mut expected = vec![];
        expected.write_u8(super::BOOLEAN_MARKER).unwrap();
        expected.write_u8(1).unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn can_serialize_false_boolean() {
        let input = vec![Amf0Value::Boolean(false)];
        let result = serialize(&input);

        let mut expected = vec![];
        expected.write_u8(super::BOOLEAN_MARKER).unwrap();
        expected.write_u8(0).unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn can_serialize_string() {
        let value = "test";

        let input = vec![Amf0Value::Utf8String(value.to_string())];
        let result = serialize(&input);

        let mut expected = vec![];
        expected.write_u8(super::STRING_MARKER).unwrap();
        expected.write_u16::<BigEndian>(value.len() as u16).unwrap();
        expected.extend(value.as_bytes());

        assert_eq!(result, expected);
    }

    #[test]
    fn can_serialize_null() {
        let input = vec![Amf0Value::Null];
        let result = serialize(&input);

        let mut expected = vec![];
        expected.write_u8(super::NULL_MARKER).unwrap();

        assert_eq!(result, expected);
    }
}