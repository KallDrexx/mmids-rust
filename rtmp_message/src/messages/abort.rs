use std::io::Cursor;
use byteorder::{BigEndian, WriteBytesExt, ReadBytesExt};

use errors::MessageDeserializationError;
use errors::MessageSerializationError;
use RtmpMessage;

/// Represents a message notifying the peer that if it is waiting for chunks to complete
/// a message, then discard the partially received message
#[derive(Eq, PartialEq, Debug)]
pub struct AbortMessage {
    pub stream_id: u32
}

impl RtmpMessage for AbortMessage {
    fn deserialize(data: Vec<u8>) -> Result<Self, MessageDeserializationError> {
        let mut cursor = Cursor::new(data);
        let stream_id = try!(cursor.read_u32::<BigEndian>());

        Ok(AbortMessage{
            stream_id: stream_id
        })
    }

    fn serialize(self) -> Result<Vec<u8>, MessageSerializationError> {
        let mut cursor = Cursor::new(Vec::new());
        try!(cursor.write_u32::<BigEndian>(self.stream_id));

        Ok(cursor.into_inner())
    }

    fn get_type_id() -> u8 { 2 }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use byteorder::{BigEndian, WriteBytesExt};    

    use super::AbortMessage;
    use rtmp_message::RtmpMessage;

    #[test]
    fn can_serialize_message() {
        let id = 523;
        let message = AbortMessage { stream_id: id };
        let result = message.serialize().unwrap();

        let mut cursor = Cursor::new(Vec::new());
        cursor.write_u32::<BigEndian>(id).unwrap();
        let expected = cursor.into_inner();

        assert_eq!(expected, result);
    }

    #[test]
    fn can_deserialize_message() {
        let id = 532;
        let mut cursor = Cursor::new(Vec::new());
        cursor.write_u32::<BigEndian>(id).unwrap();

        let result = AbortMessage::deserialize(cursor.into_inner()).unwrap();        
        let expected = AbortMessage{stream_id: id};
        assert_eq!(expected, result);
    }

    #[test]
    fn gets_expected_type_id() {
        let result = AbortMessage::get_type_id();
        assert_eq!(2, result);
    }
}