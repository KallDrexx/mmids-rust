use std::io::{Cursor, Write};
use byteorder::{BigEndian, WriteBytesExt, ReadBytesExt};
use rtmp_time::RtmpTimestamp;

use errors::MessageDeserializationError;
use errors::MessageSerializationError;
use RtmpMessage;

#[derive(PartialEq, Debug)]
pub enum UserControlEventType {
    StreamBegin,
    StreamEof,
    StreamDry,
    SetBufferLength,
    StreamIsRecorded,
    PingRequest,
    PingResponse
}

/// Message to notify the peer about user control events
#[derive(PartialEq, Debug)]
pub struct UserControlMessage {
    pub event_type: UserControlEventType,
    pub stream_id: Option<u32>,
    pub buffer_length: Option<u32>,
    pub timestamp: Option<RtmpTimestamp>
}

impl RtmpMessage for UserControlMessage {
    fn deserialize(data: Vec<u8>) -> Result<Self, MessageDeserializationError> {
        let mut cursor = Cursor::new(data);
        let event_type = match try!(cursor.read_u16::<BigEndian>()) {
            0 => UserControlEventType::StreamBegin,
            1 => UserControlEventType::StreamEof,
            2 => UserControlEventType::StreamDry,
            3 => UserControlEventType::SetBufferLength,
            4 => UserControlEventType::StreamIsRecorded,
            6 => UserControlEventType::PingRequest,
            7 => UserControlEventType::PingResponse,
            _ => return Err(MessageDeserializationError::InvalidMessageFormat)
        };

        let mut message = UserControlMessage {
            event_type: event_type,
            stream_id: None,
            buffer_length: None,
            timestamp: None
        };

        match message.event_type {
            UserControlEventType::StreamBegin => message.stream_id = Some(try!(cursor.read_u32::<BigEndian>())),
            UserControlEventType::StreamEof => message.stream_id = Some(try!(cursor.read_u32::<BigEndian>())),
            UserControlEventType::StreamDry => message.stream_id = Some(try!(cursor.read_u32::<BigEndian>())),
            UserControlEventType::StreamIsRecorded => message.stream_id = Some(try!(cursor.read_u32::<BigEndian>())),
            UserControlEventType::PingRequest => message.timestamp = Some(RtmpTimestamp::new(try!(cursor.read_u32::<BigEndian>()))),
            UserControlEventType::PingResponse => message.timestamp = Some(RtmpTimestamp::new(try!(cursor.read_u32::<BigEndian>()))),
            UserControlEventType::SetBufferLength => {
                message.stream_id = Some(try!(cursor.read_u32::<BigEndian>()));
                message.buffer_length = Some(try!(cursor.read_u32::<BigEndian>()));
            }
        }
        
        Ok(message)
    }

    fn serialize(self) -> Result<Vec<u8>, MessageSerializationError> {
        println!("Debug: {:?}", self);
        let mut cursor = Cursor::new(Vec::new());
        match self.event_type {
            UserControlEventType::StreamBegin => try!(write_stream_event(&mut cursor, 0, self.stream_id)),
            UserControlEventType::StreamEof => try!(write_stream_event(&mut cursor, 1, self.stream_id)),
            UserControlEventType::StreamDry => try!(write_stream_event(&mut cursor, 2, self.stream_id)),
            UserControlEventType::SetBufferLength => try!(write_length_event(&mut cursor, 3, self.stream_id, self.buffer_length)),
            UserControlEventType::StreamIsRecorded => try!(write_stream_event(&mut cursor, 4, self.stream_id)),
            UserControlEventType::PingRequest => try!(write_timestamp_event(&mut cursor, 6, self.timestamp)),
            UserControlEventType::PingResponse => try!(write_timestamp_event(&mut cursor, 7, self.timestamp))
        };

        Ok(cursor.into_inner())
    }

    fn get_type_id() -> u8 { 4 }
}

fn write_stream_event<W: Write>(bytes: &mut W, event_id: u16, stream_id: Option<u32>) -> Result<(), MessageSerializationError> {
    debug_assert!(stream_id.is_some(), "Stream event attempted to be serialized with a None stream id!");

    try!(bytes.write_u16::<BigEndian>(event_id));
    match stream_id {
        Some(x) => try!(bytes.write_u32::<BigEndian>(x)),
        None => try!(bytes.write_u32::<BigEndian>(0))
    };

    Ok(())
}

fn write_length_event<W: Write>(bytes: &mut W, event_id: u16, stream_id: Option<u32>, length: Option<u32>) -> Result<(), MessageSerializationError> {
    debug_assert!(stream_id.is_some(), "Buffer length event attempted to be serialized with a None stream id!");
    debug_assert!(length.is_some(), "Buffer length event attempted to be serialized with a None length value!");

    try!(bytes.write_u16::<BigEndian>(event_id));
    match stream_id {
        Some(x) => try!(bytes.write_u32::<BigEndian>(x)),
        None => try!(bytes.write_u32::<BigEndian>(0))
    };

    match length {
        Some(x) => try!(bytes.write_u32::<BigEndian>(x)),
        None => try!(bytes.write_u32::<BigEndian>(0))
    };

    Ok(())
}

fn write_timestamp_event<W: Write>(bytes: &mut W, event_id: u16, timestamp: Option<RtmpTimestamp>) -> Result<(), MessageSerializationError> {
    debug_assert!(timestamp.is_some(), "Timestamp event attempted to be serialized with a None timestamp");

    try!(bytes.write_u16::<BigEndian>(event_id));
    match timestamp {
        Some(x) => try!(bytes.write_u32::<BigEndian>(x.value)),
        None => try!(bytes.write_u32::<BigEndian>(0))
    };

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use byteorder::{BigEndian, WriteBytesExt};
    use rtmp_time::RtmpTimestamp;    

    use super::{UserControlMessage, UserControlEventType};
    use rtmp_message::RtmpMessage;

    #[test]
    fn gets_expected_type_id() {
        let result = UserControlMessage::get_type_id();
        assert_eq!(4, result);
    }

    #[test]
    fn can_serialize_stream_begin_message() {
        let stream_id = 555;
        let message = UserControlMessage {
            event_type: UserControlEventType::StreamBegin,
            stream_id: Some(stream_id),
            buffer_length: None,
            timestamp: None
        };

        let mut cursor = Cursor::new(Vec::new());
        cursor.write_u16::<BigEndian>(0).unwrap();
        cursor.write_u32::<BigEndian>(stream_id).unwrap();
        let expected = cursor.into_inner();

        let result = message.serialize().unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn can_serialize_stream_eof_message() {
        let stream_id = 555;
        let message = UserControlMessage {
            event_type: UserControlEventType::StreamEof,
            stream_id: Some(stream_id),
            buffer_length: None,
            timestamp: None
        };

        let mut cursor = Cursor::new(Vec::new());
        cursor.write_u16::<BigEndian>(1).unwrap();
        cursor.write_u32::<BigEndian>(stream_id).unwrap();
        let expected = cursor.into_inner();

        let result = message.serialize().unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn can_serialize_stream_dry_message() {
        let stream_id = 555;
        let message = UserControlMessage {
            event_type: UserControlEventType::StreamDry,
            stream_id: Some(stream_id),
            buffer_length: None,
            timestamp: None
        };

        let mut cursor = Cursor::new(Vec::new());
        cursor.write_u16::<BigEndian>(2).unwrap();
        cursor.write_u32::<BigEndian>(stream_id).unwrap();
        let expected = cursor.into_inner();

        let result = message.serialize().unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn can_serialize_set_buffer_length_message() {
        let stream_id = 555;
        let buffer_length = 666;
        let message = UserControlMessage {
            event_type: UserControlEventType::SetBufferLength,
            stream_id: Some(stream_id),
            buffer_length: Some(buffer_length),
            timestamp: None
        };

        let mut cursor = Cursor::new(Vec::new());
        cursor.write_u16::<BigEndian>(3).unwrap();
        cursor.write_u32::<BigEndian>(stream_id).unwrap();
        cursor.write_u32::<BigEndian>(buffer_length).unwrap();
        let expected = cursor.into_inner();

        let result = message.serialize().unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn can_serialize_stream_is_recorded_message() {
        let stream_id = 555;
        let message = UserControlMessage {
            event_type: UserControlEventType::StreamIsRecorded,
            stream_id: Some(stream_id),
            buffer_length: None,
            timestamp: None
        };

        let mut cursor = Cursor::new(Vec::new());
        cursor.write_u16::<BigEndian>(4).unwrap();
        cursor.write_u32::<BigEndian>(stream_id).unwrap();
        let expected = cursor.into_inner();

        let result = message.serialize().unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn can_serialize_ping_request_message() {
        let time = 555;
        let message = UserControlMessage {
            event_type: UserControlEventType::PingRequest,
            stream_id: None,
            buffer_length: None,
            timestamp: Some(RtmpTimestamp::new(time))
        };

        let mut cursor = Cursor::new(Vec::new());
        cursor.write_u16::<BigEndian>(6).unwrap();
        cursor.write_u32::<BigEndian>(time).unwrap();
        let expected = cursor.into_inner();

        let result = message.serialize().unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn can_serialize_ping_response_message() {
        let time = 555;
        let message = UserControlMessage {
            event_type: UserControlEventType::PingResponse,
            stream_id: None,
            buffer_length: None,
            timestamp: Some(RtmpTimestamp::new(time))
        };

        let mut cursor = Cursor::new(Vec::new());
        cursor.write_u16::<BigEndian>(7).unwrap();
        cursor.write_u32::<BigEndian>(time).unwrap();
        let expected = cursor.into_inner();

        let result = message.serialize().unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn can_deserialize_stream_begin_message() {
        let stream_id = 555;
        let expected = UserControlMessage {
            event_type: UserControlEventType::StreamBegin,
            stream_id: Some(stream_id),
            buffer_length: None,
            timestamp: None
        };

        let mut cursor = Cursor::new(Vec::new());
        cursor.write_u16::<BigEndian>(0).unwrap();
        cursor.write_u32::<BigEndian>(stream_id).unwrap();
        let data = cursor.into_inner();

        let result = UserControlMessage::deserialize(data).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn can_deserialize_stream_eof_message() {
        let stream_id = 555;
        let expected = UserControlMessage {
            event_type: UserControlEventType::StreamEof,
            stream_id: Some(stream_id),
            buffer_length: None,
            timestamp: None
        };

        let mut cursor = Cursor::new(Vec::new());
        cursor.write_u16::<BigEndian>(1).unwrap();
        cursor.write_u32::<BigEndian>(stream_id).unwrap();
        let data = cursor.into_inner();

        let result = UserControlMessage::deserialize(data).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn can_deserialize_stream_dry_message() {
        let stream_id = 555;
        let expected = UserControlMessage {
            event_type: UserControlEventType::StreamDry,
            stream_id: Some(stream_id),
            buffer_length: None,
            timestamp: None
        };

        let mut cursor = Cursor::new(Vec::new());
        cursor.write_u16::<BigEndian>(2).unwrap();
        cursor.write_u32::<BigEndian>(stream_id).unwrap();
        let data = cursor.into_inner();

        let result = UserControlMessage::deserialize(data).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn can_deserialize_set_buffer_length_message() {
        let stream_id = 555;
        let buffer_length = 666;
        let expected = UserControlMessage {
            event_type: UserControlEventType::SetBufferLength,
            stream_id: Some(stream_id),
            buffer_length: Some(buffer_length),
            timestamp: None
        };

        let mut cursor = Cursor::new(Vec::new());
        cursor.write_u16::<BigEndian>(3).unwrap();
        cursor.write_u32::<BigEndian>(stream_id).unwrap();
        cursor.write_u32::<BigEndian>(buffer_length).unwrap();
        let data = cursor.into_inner();

        let result = UserControlMessage::deserialize(data).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn can_deserialize_stream_is_recorded_message() {
        let stream_id = 555;
        let expected = UserControlMessage {
            event_type: UserControlEventType::StreamIsRecorded,
            stream_id: Some(stream_id),
            buffer_length: None,
            timestamp: None
        };

        let mut cursor = Cursor::new(Vec::new());
        cursor.write_u16::<BigEndian>(4).unwrap();
        cursor.write_u32::<BigEndian>(stream_id).unwrap();
        let data = cursor.into_inner();

        let result = UserControlMessage::deserialize(data).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn can_deserialize_ping_request_message() {
        let time = 555;
        let expected = UserControlMessage {
            event_type: UserControlEventType::PingRequest,
            stream_id: None,
            buffer_length: None,
            timestamp: Some(RtmpTimestamp::new(time))
        };

        let mut cursor = Cursor::new(Vec::new());
        cursor.write_u16::<BigEndian>(6).unwrap();
        cursor.write_u32::<BigEndian>(time).unwrap();
        let data = cursor.into_inner();

        let result = UserControlMessage::deserialize(data).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn can_deserialize_ping_response_message() {
        let time = 555;
        let expected = UserControlMessage {
            event_type: UserControlEventType::PingResponse,
            stream_id: None,
            buffer_length: None,
            timestamp: Some(RtmpTimestamp::new(time))
        };

        let mut cursor = Cursor::new(Vec::new());
        cursor.write_u16::<BigEndian>(7).unwrap();
        cursor.write_u32::<BigEndian>(time).unwrap();
        let data = cursor.into_inner();

        let result = UserControlMessage::deserialize(data).unwrap();
        assert_eq!(result, expected);
    }
}