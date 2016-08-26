

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use byteorder::{BigEndian, WriteBytesExt};    
    
    use rtmp_message::RtmpMessage;

    #[test]
    fn can_serialize_message() {
        let size = 523;
        let message = RtmpMessage::WindowAcknowledgement { size: size };
        
        let mut cursor = Cursor::new(Vec::new());
        cursor.write_u32::<BigEndian>(size).unwrap();
        let expected = cursor.into_inner();

        let raw_message = message.serialize().unwrap();
        assert_eq!(raw_message.data, expected);
        assert_eq!(raw_message.type_id, 5);
    }

    #[test]
    fn can_deserialize_message() {
        let size = 532;
        let expected = RtmpMessage::WindowAcknowledgement { size: size };

        let mut cursor = Cursor::new(Vec::new());
        cursor.write_u32::<BigEndian>(size).unwrap();

        let result = RtmpMessage::deserialize(cursor.into_inner(), 5).unwrap();     
        assert_eq!(result, expected);
    }
}