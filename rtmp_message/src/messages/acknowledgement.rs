use std::io::Cursor;
use byteorder::{BigEndian, WriteBytesExt, ReadBytesExt};

use errors::MessageDeserializationError;
use errors::MessageSerializationError;
use RtmpMessage;

/// Sent when the peer has received bytes equal to the window size
#[derive(Eq, PartialEq, Debug)]
pub struct AcknowledgementMessage {
    pub sequence_number: u32
}

impl RtmpMessage for AcknowledgementMessage {
    fn deserialize(data: Vec<u8>) -> Result<Self, MessageDeserializationError> {
        let mut cursor = Cursor::new(data);
        let sequence_number = try!(cursor.read_u32::<BigEndian>());

        Ok(AcknowledgementMessage{
            sequence_number: sequence_number
        })
    }

    fn serialize(self) -> Result<Vec<u8>, MessageSerializationError> {
        let mut cursor = Cursor::new(Vec::new());
        try!(cursor.write_u32::<BigEndian>(self.sequence_number));

        Ok(cursor.into_inner())
    }

    fn get_type_id() -> u8 { 3 }
}


#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use byteorder::{BigEndian, WriteBytesExt};    
    
    use super::AcknowledgementMessage;
    use rtmp_message::RtmpMessage;

    #[test]
    fn can_serialize_message() {
        let number = 523;
        let message = AcknowledgementMessage { sequence_number: number };
        let result = message.serialize().unwrap();

        let mut cursor = Cursor::new(Vec::new());
        cursor.write_u32::<BigEndian>(number).unwrap();
        let expected = cursor.into_inner();

        assert_eq!(expected, result);
    }

    #[test]
    fn can_deserialize_message() {
        let number = 532;
        let mut cursor = Cursor::new(Vec::new());
        cursor.write_u32::<BigEndian>(number).unwrap();

        let result = AcknowledgementMessage::deserialize(cursor.into_inner()).unwrap();        
        let expected = AcknowledgementMessage { sequence_number: number };
        assert_eq!(expected, result);
    }

    #[test]
    fn gets_expected_type_id() {
        let result = AcknowledgementMessage::get_type_id();
        assert_eq!(3, result);
    }
}