

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use byteorder::{BigEndian, WriteBytesExt};
    use rtmp_time::RtmpTimestamp;    

    use rtmp_message::{RtmpMessage, UserControlEventType};

    #[test]
    fn can_serializel_stream_begin_message() {
        let stream_id = 555;
        let message = RtmpMessage::UserControl {
            event_type: UserControlEventType::StreamBegin,
            stream_id: Some(stream_id),
            buffer_length: None,
            timestamp: None
        };

        let mut cursor = Cursor::new(Vec::new());
        cursor.write_u16::<BigEndian>(0).unwrap();
        cursor.write_u32::<BigEndian>(stream_id).unwrap();
        let expected = cursor.into_inner();

        let raw_message = message.serialize().unwrap();
        assert_eq!(raw_message.data, expected);
        assert_eq!(raw_message.type_id, 4);
    }

    #[test]
    fn can_serializel_stream_eof_message() {
        let stream_id = 555;
        let message = RtmpMessage::UserControl {
            event_type: UserControlEventType::StreamEof,
            stream_id: Some(stream_id),
            buffer_length: None,
            timestamp: None
        };

        let mut cursor = Cursor::new(Vec::new());
        cursor.write_u16::<BigEndian>(1).unwrap();
        cursor.write_u32::<BigEndian>(stream_id).unwrap();
        let expected = cursor.into_inner();

        let raw_message = message.serialize().unwrap();
        assert_eq!(raw_message.data, expected);
        assert_eq!(raw_message.type_id, 4);
    }

    #[test]
    fn can_serializel_stream_dry_message() {
        let stream_id = 555;
        let message = RtmpMessage::UserControl {
            event_type: UserControlEventType::StreamDry,
            stream_id: Some(stream_id),
            buffer_length: None,
            timestamp: None
        };

        let mut cursor = Cursor::new(Vec::new());
        cursor.write_u16::<BigEndian>(2).unwrap();
        cursor.write_u32::<BigEndian>(stream_id).unwrap();
        let expected = cursor.into_inner();

        let raw_message = message.serialize().unwrap();
        assert_eq!(raw_message.data, expected);
        assert_eq!(raw_message.type_id, 4);
    }

    #[test]
    fn can_serializel_set_buffer_length_message() {
        let stream_id = 555;
        let buffer_length = 666;
        let message = RtmpMessage::UserControl {
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

        let raw_message = message.serialize().unwrap();
        assert_eq!(raw_message.data, expected);
        assert_eq!(raw_message.type_id, 4);
    }

    #[test]
    fn can_serializel_stream_is_recorded_message() {
        let stream_id = 555;
        let message = RtmpMessage::UserControl {
            event_type: UserControlEventType::StreamIsRecorded,
            stream_id: Some(stream_id),
            buffer_length: None,
            timestamp: None
        };

        let mut cursor = Cursor::new(Vec::new());
        cursor.write_u16::<BigEndian>(4).unwrap();
        cursor.write_u32::<BigEndian>(stream_id).unwrap();
        let expected = cursor.into_inner();

        let raw_message = message.serialize().unwrap();
        assert_eq!(raw_message.data, expected);
        assert_eq!(raw_message.type_id, 4);
    }

    #[test]
    fn can_serializel_ping_request_message() {
        let time = 555;
        let message = RtmpMessage::UserControl {
            event_type: UserControlEventType::PingRequest,
            stream_id: None,
            buffer_length: None,
            timestamp: Some(RtmpTimestamp::new(time))
        };

        let mut cursor = Cursor::new(Vec::new());
        cursor.write_u16::<BigEndian>(6).unwrap();
        cursor.write_u32::<BigEndian>(time).unwrap();
        let expected = cursor.into_inner();

        let raw_message = message.serialize().unwrap();
        assert_eq!(raw_message.data, expected);
        assert_eq!(raw_message.type_id, 4);
    }

    #[test]
    fn can_serializel_ping_response_message() {
        let time = 555;
        let message = RtmpMessage::UserControl {
            event_type: UserControlEventType::PingResponse,
            stream_id: None,
            buffer_length: None,
            timestamp: Some(RtmpTimestamp::new(time))
        };

        let mut cursor = Cursor::new(Vec::new());
        cursor.write_u16::<BigEndian>(7).unwrap();
        cursor.write_u32::<BigEndian>(time).unwrap();
        let expected = cursor.into_inner();

        let raw_message = message.serialize().unwrap();
        assert_eq!(raw_message.data, expected);
        assert_eq!(raw_message.type_id, 4);
    }

    #[test]
    fn can_deserializel_stream_begin_message() {
        let stream_id = 555;
        let expected = RtmpMessage::UserControl {
            event_type: UserControlEventType::StreamBegin,
            stream_id: Some(stream_id),
            buffer_length: None,
            timestamp: None
        };

        let mut cursor = Cursor::new(Vec::new());
        cursor.write_u16::<BigEndian>(0).unwrap();
        cursor.write_u32::<BigEndian>(stream_id).unwrap();
        let data = cursor.into_inner();

        let result = RtmpMessage::deserialize(data, 4).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn can_deserializel_stream_eof_message() {
        let stream_id = 555;
        let expected = RtmpMessage::UserControl {
            event_type: UserControlEventType::StreamEof,
            stream_id: Some(stream_id),
            buffer_length: None,
            timestamp: None
        };

        let mut cursor = Cursor::new(Vec::new());
        cursor.write_u16::<BigEndian>(1).unwrap();
        cursor.write_u32::<BigEndian>(stream_id).unwrap();
        let data = cursor.into_inner();

        let result = RtmpMessage::deserialize(data, 4).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn can_deserializel_stream_dry_message() {
        let stream_id = 555;
        let expected = RtmpMessage::UserControl {
            event_type: UserControlEventType::StreamDry,
            stream_id: Some(stream_id),
            buffer_length: None,
            timestamp: None
        };

        let mut cursor = Cursor::new(Vec::new());
        cursor.write_u16::<BigEndian>(2).unwrap();
        cursor.write_u32::<BigEndian>(stream_id).unwrap();
        let data = cursor.into_inner();

        let result = RtmpMessage::deserialize(data, 4).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn can_deserializel_set_buffer_length_message() {
        let stream_id = 555;
        let buffer_length = 666;
        let expected = RtmpMessage::UserControl {
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

        let result = RtmpMessage::deserialize(data, 4).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn can_deserializel_stream_is_recorded_message() {
        let stream_id = 555;
        let expected = RtmpMessage::UserControl {
            event_type: UserControlEventType::StreamIsRecorded,
            stream_id: Some(stream_id),
            buffer_length: None,
            timestamp: None
        };

        let mut cursor = Cursor::new(Vec::new());
        cursor.write_u16::<BigEndian>(4).unwrap();
        cursor.write_u32::<BigEndian>(stream_id).unwrap();
        let data = cursor.into_inner();

        let result = RtmpMessage::deserialize(data, 4).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn can_deserializel_ping_request_message() {
        let time = 555;
        let expected = RtmpMessage::UserControl {
            event_type: UserControlEventType::PingRequest,
            stream_id: None,
            buffer_length: None,
            timestamp: Some(RtmpTimestamp::new(time))
        };

        let mut cursor = Cursor::new(Vec::new());
        cursor.write_u16::<BigEndian>(6).unwrap();
        cursor.write_u32::<BigEndian>(time).unwrap();
        let data = cursor.into_inner();

        let result = RtmpMessage::deserialize(data, 4).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn can_deserializel_ping_response_message() {
        let time = 555;
        let expected = RtmpMessage::UserControl {
            event_type: UserControlEventType::PingResponse,
            stream_id: None,
            buffer_length: None,
            timestamp: Some(RtmpTimestamp::new(time))
        };

        let mut cursor = Cursor::new(Vec::new());
        cursor.write_u16::<BigEndian>(7).unwrap();
        cursor.write_u32::<BigEndian>(time).unwrap();
        let data = cursor.into_inner();

        let result = RtmpMessage::deserialize(data, 4).unwrap();
        assert_eq!(result, expected);
    }
}