use std::io::Cursor;
use byteorder::{BigEndian, WriteBytesExt, ReadBytesExt};

use errors::MessageDeserializationError;
use errors::MessageSerializationError;
use RtmpMessage;

/// Sent to inform the peer of a change in how much data should be received before
/// the sender expects an acknowledgement sent back
#[derive(Eq, PartialEq, Debug)]
pub struct WindowAcknowledgementSizeMessage {
    pub size: u32
}

impl RtmpMessage for WindowAcknowledgementSizeMessage {
    fn deserialize(data: Vec<u8>) -> Result<Self, MessageDeserializationError> {
        let mut cursor = Cursor::new(data);
        let size = try!(cursor.read_u32::<BigEndian>());

        Ok(WindowAcknowledgementSizeMessage{
            size: size
        })
    }

    fn serialize(self) -> Result<Vec<u8>, MessageSerializationError> {
        let mut cursor = Cursor::new(Vec::new());
        try!(cursor.write_u32::<BigEndian>(self.size));

        Ok(cursor.into_inner())
    }

    fn get_type_id() -> u8 { 2 }
}


#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use byteorder::{BigEndian, WriteBytesExt};    
    
    use super::WindowAcknowledgementSizeMessage;
    use rtmp_message::RtmpMessage;

    #[test]
    fn can_serialize_message() {
        let size = 523;
        let message = WindowAcknowledgementSizeMessage { size: size };
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

        let result = WindowAcknowledgementSizeMessage::deserialize(cursor.into_inner()).unwrap();        
        let expected = WindowAcknowledgementSizeMessage { size: size };
        assert_eq!(expected, result);
    }

    #[test]
    fn gets_expected_type_id() {
        let result = WindowAcknowledgementSizeMessage::get_type_id();
        assert_eq!(2, result);
    }
}