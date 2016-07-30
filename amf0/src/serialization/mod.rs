use super::Amf0Value;

/// Serializes values into an amf0 encoded vector of bytes
pub fn serialize(values: &Vec<Amf0Value>) -> Vec<u8> {
    vec![]
}

const NUMBER_MARKER: u8 = 0;
const BOOLEAN_MARKER: u8 = 1;
const STRING_MARKER: u8 = 2;
const OBJECT_MARKER: u8 = 3;
const NULL_MARKER: u8 = 5; 

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