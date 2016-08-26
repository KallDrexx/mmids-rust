#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use byteorder::{BigEndian, WriteBytesExt};    

    use rtmp_message::RtmpMessage;

    #[test]
    fn can_serialize_message() {
        let id = 523;
        let mut cursor = Cursor::new(Vec::new());
        cursor.write_u32::<BigEndian>(id).unwrap();
        let expected = cursor.into_inner();

        let message = RtmpMessage::Abort { stream_id: id };
        let raw_message = message.serialize().unwrap();

        assert_eq!(raw_message.data, expected);
        assert_eq!(raw_message.type_id, 2);
    }

    #[test]
    fn can_deserialize_message() {
        let id = 532;
        let expected = RtmpMessage::Abort{stream_id: id};

        let mut cursor = Cursor::new(Vec::new());
        cursor.write_u32::<BigEndian>(id).unwrap();

        let result = RtmpMessage::deserialize(cursor.into_inner(), 2).unwrap();                
        assert_eq!(result, expected);
    }
}