use std::io::Cursor;
use amf0;
use amf0::Amf0Value;

use errors::{MessageDeserializationError, MessageSerializationError};
use rtmp_message::{RtmpMessage, RawRtmpMessage};

pub fn serialize(values: Vec<Amf0Value>) -> Result<RawRtmpMessage, MessageSerializationError> {
    let bytes = try!(amf0::serialize(&values)); 

    Ok(RawRtmpMessage{ 
        data: bytes,
        type_id: 18
    })
}

pub fn deserialize(data: Vec<u8>) -> Result<RtmpMessage, MessageDeserializationError> {
    let mut cursor = Cursor::new(data);
    let values = try!(amf0::deserialize(&mut cursor));

    Ok(RtmpMessage::Amf0Data {
        values: values
    })
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use amf0::Amf0Value;
    use amf0;

    use rtmp_message::{RtmpMessage};

    #[test]
    fn can_serialize_message() {
        let message = RtmpMessage::Amf0Data {
            values: vec![Amf0Value::Boolean(true), Amf0Value::Number(52.0)]
        };

        let raw_message = RtmpMessage::serialize(message).unwrap();

        let mut cursor = Cursor::new(raw_message.data);
        let result = amf0::deserialize(&mut cursor).unwrap();
        let expected = vec![Amf0Value::Boolean(true), Amf0Value::Number(52.0)];

        assert_eq!(expected, result);
        assert_eq!(18, raw_message.type_id);
    }

    #[test]
    fn can_deserialize_message() {
        let values = vec![Amf0Value::Boolean(true), Amf0Value::Number(52.0)];
        let bytes = amf0::serialize(&values).unwrap();

        let result = RtmpMessage::deserialize(bytes, 18).unwrap();

        let expected = RtmpMessage::Amf0Data {
            values: vec![Amf0Value::Boolean(true), Amf0Value::Number(52.0)]
        };

        assert_eq!(expected, result);
    }
}