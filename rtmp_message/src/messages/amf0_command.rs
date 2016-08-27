use std::io::Cursor;
use amf0::Amf0Value;
use amf0;

use errors::{MessageDeserializationError, MessageSerializationError};
use rtmp_message::{RtmpMessage, RawRtmpMessage};

pub fn serialize(command_name: String, 
                    transaction_id: f64, 
                    command_object: Amf0Value, 
                    mut additional_arguments: Vec<Amf0Value>) -> Result<RawRtmpMessage, MessageSerializationError> {
    let mut values = vec![
        Amf0Value::Utf8String(command_name),
        Amf0Value::Number(transaction_id),
        command_object
    ];

    values.append(&mut additional_arguments);
    let bytes = try!(amf0::serialize(&values));

    Ok(RawRtmpMessage{ 
        data: bytes,
        type_id: 20
    })
}

pub fn deserialize(data: Vec<u8>) -> Result<RtmpMessage, MessageDeserializationError> {
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

        Ok(RtmpMessage::Amf0Command {
            command_name: name,
            transaction_id: transaction_id,
            command_object: command_object,
            additional_arguments: values
        })
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use std::collections::HashMap;
    use amf0::Amf0Value;
    use amf0;

    use rtmp_message::RtmpMessage;

    #[test]
    fn can_serialize_message() {
        let mut properties1 = HashMap::new();
        properties1.insert("prop1".to_string(), Amf0Value::Utf8String("abc".to_string()));
        properties1.insert("prop2".to_string(), Amf0Value::Null);

        let mut properties2 = HashMap::new();
        properties2.insert("prop1".to_string(), Amf0Value::Utf8String("abc".to_string()));
        properties2.insert("prop2".to_string(), Amf0Value::Null);  

        let message = RtmpMessage::Amf0Command {
            command_name: "test".to_string(),
            transaction_id: 23.0,
            command_object: Amf0Value::Object(properties1),
            additional_arguments: vec![Amf0Value::Boolean(true), Amf0Value::Number(52.0)]
        };

        let raw_message = message.serialize().unwrap();
        let mut cursor = Cursor::new(raw_message.data);
        let result = amf0::deserialize(&mut cursor).unwrap();        

        let expected = vec![
            Amf0Value::Utf8String("test".to_string()),
            Amf0Value::Number(23.0),
            Amf0Value::Object(properties2),
            Amf0Value::Boolean(true),
            Amf0Value::Number(52.0)
        ];

        assert_eq!(expected, result);
        assert_eq!(20, raw_message.type_id);
    }

    #[test]
    fn can_deserialize_message() {
        let mut properties1 = HashMap::new();
        properties1.insert("prop1".to_string(), Amf0Value::Utf8String("abc".to_string()));
        properties1.insert("prop2".to_string(), Amf0Value::Null);

        let mut properties2 = HashMap::new();
        properties2.insert("prop1".to_string(), Amf0Value::Utf8String("abc".to_string()));
        properties2.insert("prop2".to_string(), Amf0Value::Null);        

        let values = vec![
            Amf0Value::Utf8String("test".to_string()),
            Amf0Value::Number(23.0),
            Amf0Value::Object(properties1),
            Amf0Value::Boolean(true),
            Amf0Value::Number(52.0)
        ];

        let bytes = amf0::serialize(&values).unwrap();
        
        let expected = RtmpMessage::Amf0Command {
            command_name: "test".to_string(),
            transaction_id: 23.0,
            command_object: Amf0Value::Object(properties2),
            additional_arguments: vec![Amf0Value::Boolean(true), Amf0Value::Number(52.0)]
        };

        let result = RtmpMessage::deserialize(bytes, 20).unwrap();

        assert_eq!(expected, result);
    }
}