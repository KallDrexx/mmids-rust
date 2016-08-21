use std::io::Cursor;
use amf0;
use amf0::Amf0Value;

use errors::MessageDeserializationError;
use errors::MessageSerializationError;
use RtmpMessage;

/// Message used to denote an amf0 encoded command, or response to a previously issued command
#[derive(PartialEq, Debug)]
pub struct Amf0CommandMessage {
    pub command_name: String,
    pub transaction_id: f64,
    pub command_object: Amf0Value,
    pub additional_arguments: Vec<Amf0Value>
}

impl RtmpMessage for Amf0CommandMessage {
    fn deserialize(data: Vec<u8>) -> Result<Self, MessageDeserializationError> {
        let mut cursor = Cursor::new(data);
        let mut values = try!(amf0::deserialize(&mut cursor));
        if values.len() < 3 {
            return Err(MessageDeserializationError::InvalidMessageFormat);
        }

        let mut name = "".to_string();
        let mut transaction_id = 0.0;
        let mut command_object = Amf0Value::Null;
        let mut index = 0;
        for value in values.drain(0..3) {
            if index == 0 {
                name = match value {
                    Amf0Value::Utf8String(value) => value,
                    _ => return Err(MessageDeserializationError::InvalidMessageFormat)
                };
            }

            else if index == 1 {
                 transaction_id = match value {
                    Amf0Value::Number(value) => value,
                    _ => return Err(MessageDeserializationError::InvalidMessageFormat)
                };
            }
            else if index == 2 {
                command_object = value
            }

            index = index + 1;
        }

        Ok(Amf0CommandMessage {
            command_name: name,
            transaction_id: transaction_id,
            command_object: command_object,
            additional_arguments: values
        })
    }

    fn serialize(mut self) -> Result<Vec<u8>, MessageSerializationError> {
        let mut values = vec![
            Amf0Value::Utf8String(self.command_name),
            Amf0Value::Number(self.transaction_id),
            self.command_object
        ];

        values.append(&mut self.additional_arguments);

        let bytes = try!(amf0::serialize(&values));
        Ok(bytes)        
    }

    fn get_type_id() -> u8 { 20 }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use std::collections::HashMap;
    use amf0::{Amf0Value, serialize, deserialize};

    use super::Amf0CommandMessage;
    use rtmp_message::RtmpMessage;

    #[test]
    fn can_serialize_message() {
        let message = Amf0CommandMessage {
            command_name: "test".to_string(),
            transaction_id: 23.0,
            command_object: Amf0Value::Object(get_test_amf0_properties()),
            additional_arguments: vec![Amf0Value::Boolean(true), Amf0Value::Number(52.0)]
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
        
        let expected = Amf0CommandMessage {
            command_name: "test".to_string(),
            transaction_id: 23.0,
            command_object: Amf0Value::Object(get_test_amf0_properties()),
            additional_arguments: vec![Amf0Value::Boolean(true), Amf0Value::Number(52.0)]
        };

        let result = Amf0CommandMessage::deserialize(bytes).unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn gets_expected_type_id() {
        let result = Amf0CommandMessage::get_type_id();
        assert_eq!(20, result);
    }

    fn get_test_amf0_values() -> Vec<Amf0Value> {
        vec![
            Amf0Value::Utf8String("test".to_string()),
            Amf0Value::Number(23.0),
            Amf0Value::Object(get_test_amf0_properties()),
            Amf0Value::Boolean(true),
            Amf0Value::Number(52.0)
        ]
    }

    fn get_test_amf0_properties() -> HashMap<String, Amf0Value> {
        let mut properties = HashMap::new();
        properties.insert("prop1".to_string(), Amf0Value::Utf8String("abc".to_string()));
        properties.insert("prop2".to_string(), Amf0Value::Null);

        properties
    }
}