use std::io::Cursor;
use amf0;
use amf0::Amf0Value;

use errors::MessageDeserializationError;
use errors::MessageSerializationError;
use RtmpMessage;

/// Message containing metadata or other misc data, encoded in Amf0
#[derive(PartialEq, Debug)]
pub struct Amf0DataMessage {
    pub values: Vec<Amf0Value>
}

impl RtmpMessage for Amf0DataMessage {
    fn deserialize(data: Vec<u8>) -> Result<Self, MessageDeserializationError> {
        let mut cursor = Cursor::new(data);
        let values = try!(amf0::deserialize(&mut cursor));
        
        Ok(Amf0DataMessage {
            values: values
        })
    }

    fn serialize(self) -> Result<Vec<u8>, MessageSerializationError> {
        let bytes = try!(amf0::serialize(&self.values));
        Ok(bytes)        
    }

    fn get_type_id() -> u8 { 18 }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use amf0::{Amf0Value, serialize, deserialize};

    use super::Amf0DataMessage;
    use rtmp_message::RtmpMessage;

    #[test]
    fn can_serialize_message() {
        let message = Amf0DataMessage {
            values: vec![Amf0Value::Boolean(true), Amf0Value::Number(52.0)]
        };

        let expected = get_test_amf0_values();

        let bytes = message.serialize().unwrap();
        let mut cursor = Cursor::new(bytes);
        let result = deserialize(&mut cursor).unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn can_deserialize_message() {
        let values = get_test_amf0_values();
        let bytes = serialize(&values).unwrap();
        
        let expected = Amf0DataMessage {
            values: vec![Amf0Value::Boolean(true), Amf0Value::Number(52.0)]
        };

        let result = Amf0DataMessage::deserialize(bytes).unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn gets_expected_type_id() {
        let result = Amf0DataMessage::get_type_id();
        assert_eq!(18, result);
    }

    fn get_test_amf0_values() -> Vec<Amf0Value> {
        vec![Amf0Value::Boolean(true), Amf0Value::Number(52.0)]
    }
}