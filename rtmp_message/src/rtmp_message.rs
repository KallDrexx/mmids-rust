use rtmp_time::RtmpTimestamp;
use amf0::Amf0Value;

use errors::{MessageDeserializationError, MessageSerializationError};

#[derive(Eq, PartialEq, Debug)]
pub enum PeerBandwidthLimitType { Hard, Soft, Dynamic }

#[derive(Eq, PartialEq, Debug)]
pub enum UserControlEventType {
    StreamBegin,
    StreamEof,
    StreamDry,
    SetBufferLength,
    StreamIsRecorded,
    PingRequest,
    PingResponse
}

#[derive(PartialEq, Debug)]
pub enum RtmpMessage {
    Unknown { type_id: u8, data: Vec<u8> },
    Abort { stream_id: u32 },
    Acknowledgement { sequence_number: u32 },
    Amf0Command { command_name: String, transaction_id: f64, command_object: Amf0Value, additional_arguments: Vec<Amf0Value> },
    Amf0Data { values: Vec<Amf0Value> },
    AudioData { data: Vec<u8> },
    SetChunkSize { size: u32 },
    SetPeerBandwidth { size: u32, limit_type: PeerBandwidthLimitType },
    UserControl { event_type: UserControlEventType, stream_id: Option<u32>, buffer_length: Option<u32>, timestamp: Option<RtmpTimestamp> },
    VideoData { data: Vec<u8> },
    WindowAcknowledgement { size: u32 }
}

#[derive(PartialEq, Debug)]
pub struct RawRtmpMessage {
    pub data: Vec<u8>,
    pub type_id: u8
}

impl RtmpMessage {
    pub fn serialize(message: RtmpMessage) -> Result<RawRtmpMessage, MessageSerializationError> {
        match message {
            RtmpMessage::Unknown { type_id, data } => Ok(RawRtmpMessage { type_id: type_id, data: data }),
            RtmpMessage::Abort { stream_id: _ } => unimplemented!(),
            RtmpMessage::Acknowledgement { sequence_number: _ } => unimplemented!(),
            RtmpMessage::Amf0Command { command_name: _, transaction_id: _, command_object: _, additional_arguments: _ } => unimplemented!(),
            RtmpMessage::Amf0Data { values: _ } => unimplemented!(),
            RtmpMessage::AudioData { data: _ } => unimplemented!(),
            RtmpMessage::SetChunkSize { size: _ } => unimplemented!(),
            RtmpMessage::SetPeerBandwidth { size: _, limit_type: _ } => unimplemented!(),
            RtmpMessage::UserControl { event_type: _, stream_id: _, buffer_length: _, timestamp: _ } => unimplemented!(),
            RtmpMessage::VideoData { data: _ } => unimplemented!(),
            RtmpMessage::WindowAcknowledgement { size: _ } => unimplemented!()
        }
    }

    pub fn deserialize(bytes: Vec<u8>, type_id: u8) -> Result<Self, MessageDeserializationError> {
        match type_id {
            _ => Ok(RtmpMessage::Unknown { type_id: type_id, data: bytes })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{RtmpMessage, RawRtmpMessage};

    use std::io::Cursor;
    use std::collections::HashMap;
    use byteorder::{BigEndian, WriteBytesExt};
    use amf0::Amf0Value;
    use amf0;

    #[test]
    fn can_serialize_unknown_message() {
        let message = RtmpMessage::Unknown { type_id: 29, data: vec![1,2,3,4] };
        let expected = RawRtmpMessage { type_id: 29, data: vec![1,2,3,4] };

        let result = RtmpMessage::serialize(message).unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn can_deserialize_unknown_message() {
        let id = 255;
        let expected = RtmpMessage::Unknown { type_id: id, data: vec![1,2,3,4] };
        let result = RtmpMessage::deserialize(vec![1,2,3,4], id).unwrap();        
        assert_eq!(expected, result);
    }

    #[test]
    fn can_serialize_abort_message() {
        let id = 523;
        let message = RtmpMessage::Abort { stream_id: id };
        let result = RtmpMessage::serialize(message).unwrap();

        let mut cursor = Cursor::new(Vec::new());
        cursor.write_u32::<BigEndian>(id).unwrap();
        let expected = RawRtmpMessage { type_id: 2, data: cursor.into_inner() };

        assert_eq!(expected, result);
    }

    #[test]
    fn can_deserialize_abort_message() {
        let id = 532;
        let mut cursor = Cursor::new(Vec::new());
        cursor.write_u32::<BigEndian>(id).unwrap();

        let expected = RtmpMessage::Abort{stream_id: id};

        let result = RtmpMessage::deserialize(cursor.into_inner(), 2).unwrap();        
        
        assert_eq!(expected, result);
    }

    #[test]
    fn can_serialize_acknowledgement_message() {
        let number = 523;
        let message = RtmpMessage::Acknowledgement { sequence_number: number };

        let result = RtmpMessage::serialize(message).unwrap();

        let mut cursor = Cursor::new(Vec::new());
        cursor.write_u32::<BigEndian>(number).unwrap();
        let expected = RawRtmpMessage { type_id: 3, data: cursor.into_inner() };

        assert_eq!(expected, result);
    }

    #[test]
    fn can_deserialize_acknowledgement_message() {
        let number = 532;
        let mut cursor = Cursor::new(Vec::new());
        cursor.write_u32::<BigEndian>(number).unwrap();

        let result = RtmpMessage::deserialize(cursor.into_inner(), 3).unwrap();

        let expected = RtmpMessage::Acknowledgement { sequence_number: number };
        assert_eq!(expected, result);
    }

    #[test]
    fn can_serialize_amf0_command_message() {
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

        let raw_message = RtmpMessage::serialize(message).unwrap();
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
    fn can_deserialize_amf0_command_message() {
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

    #[test]
    fn can_serialize_amf0_data_message() {
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
    fn can_deserialize_amf0_data_message() {
        let values = vec![Amf0Value::Boolean(true), Amf0Value::Number(52.0)];
        let bytes = amf0::serialize(&values).unwrap();

        let result = RtmpMessage::deserialize(bytes, 18).unwrap();

        let expected = RtmpMessage::Amf0Data {
            values: vec![Amf0Value::Boolean(true), Amf0Value::Number(52.0)]
        };

        assert_eq!(expected, result);
    }
}