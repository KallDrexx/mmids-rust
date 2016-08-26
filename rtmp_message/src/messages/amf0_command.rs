

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