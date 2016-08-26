
#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use byteorder::{BigEndian, WriteBytesExt};    
    
    use rtmp_message::{RtmpMessage, RawRtmpMessage};

    #[test]
    fn can_serialize_message() {
        let number = 523;
        let message = RtmpMessage::Acknowledgement { sequence_number: number };

        let result = message.serialize().unwrap();

        let mut cursor = Cursor::new(Vec::new());
        cursor.write_u32::<BigEndian>(number).unwrap();
        let expected = RawRtmpMessage { type_id: 3, data: cursor.into_inner() };

        assert_eq!(expected, result);
    }

    #[test]
    fn can_deserialize_message() {
        let number = 532;
        let mut cursor = Cursor::new(Vec::new());
        cursor.write_u32::<BigEndian>(number).unwrap();

        let result = RtmpMessage::deserialize(cursor.into_inner(), 3).unwrap();

        let expected = RtmpMessage::Acknowledgement { sequence_number: number };
        assert_eq!(expected, result);
    }
}