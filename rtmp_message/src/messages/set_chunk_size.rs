use std::io::Cursor;
use byteorder::{BigEndian, WriteBytesExt, ReadBytesExt};

use errors::MessageDeserializationError;
use errors::MessageSerializationError;
use RtmpMessage;

/// Represents the maximum chunk size the sender will be sending.  Default is 128 bytes
#[derive(Eq, PartialEq, Debug)]
pub struct SetChunkSizeMessage {
    pub size: u32
}

const MAX_SIZE: u32 = 0x80000000 - 1;

impl RtmpMessage for SetChunkSizeMessage {
    fn deserialize(data: Vec<u8>) -> Result<Self, MessageDeserializationError> {
        let mut cursor = Cursor::new(data);
        let size = try!(cursor.read_u32::<BigEndian>());

        if size > MAX_SIZE {
            return Err(MessageDeserializationError::InvalidMessageFormat);
        }

        Ok(SetChunkSizeMessage{
            size: size
        })
    }

    fn serialize(self) -> Result<Vec<u8>, MessageSerializationError> {
        if self.size > MAX_SIZE {
            return Err(MessageSerializationError::InvalidChunkSize);
        }

        let mut cursor = Cursor::new(Vec::new());
        try!(cursor.write_u32::<BigEndian>(self.size));

        Ok(cursor.into_inner())
    }

    fn get_type_id() -> u8 { 1 }
}


#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use byteorder::{BigEndian, WriteBytesExt};    
    
    use super::SetChunkSizeMessage;
    use rtmp_message::RtmpMessage;

    #[test]
    fn can_serialize_message() {
        let size = 523;
        let message = SetChunkSizeMessage { size: size };
        let result = message.serialize().unwrap();

        let mut cursor = Cursor::new(Vec::new());
        cursor.write_u32::<BigEndian>(size).unwrap();
        let expected = cursor.into_inner();

        assert_eq!(expected, result);
    }

    #[test]
    fn can_deserialize_message() {
        let size = 532;
        let mut cursor = Cursor::new(Vec::new());
        cursor.write_u32::<BigEndian>(size).unwrap();

        let result = SetChunkSizeMessage::deserialize(cursor.into_inner()).unwrap();        
        let expected = SetChunkSizeMessage { size: size };
        assert_eq!(expected, result);
    }

    #[test]
    fn gets_expected_type_id() {
        let result = SetChunkSizeMessage::get_type_id();
        assert_eq!(1, result);
    }
}