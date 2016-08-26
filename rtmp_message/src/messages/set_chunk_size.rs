

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use byteorder::{BigEndian, WriteBytesExt};    
    
    use rtmp_message::{RtmpMessage};

    #[test]
    fn can_serialize_message() {
        let size = 523;
        let message = RtmpMessage::SetChunkSize { size: size };

        let mut cursor = Cursor::new(Vec::new());
        cursor.write_u32::<BigEndian>(size).unwrap();
        let expected = cursor.into_inner();

        let raw_message = message.serialize().unwrap();

        assert_eq!(raw_message.data, expected);
        assert_eq!(raw_message.type_id, 1);
    }

    #[test]
    fn can_deserialize_message() {
        let size = 532;
        let mut cursor = Cursor::new(Vec::new());
        cursor.write_u32::<BigEndian>(size).unwrap();

        let result = RtmpMessage::deserialize(cursor.into_inner(), 1).unwrap();        
        let expected = RtmpMessage::SetChunkSize { size: size };
        assert_eq!(result, expected);
    }
}