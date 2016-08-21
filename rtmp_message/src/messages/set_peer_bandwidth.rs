use std::io::Cursor;
use byteorder::{BigEndian, WriteBytesExt, ReadBytesExt};

use errors::MessageDeserializationError;
use errors::MessageSerializationError;
use RtmpMessage;

#[derive(Eq, PartialEq, Debug)]
pub enum PeerBandwidthLimitType { Hard, Soft, Dynamic }

/// Sender is requesting the receiver limit it's output bandwidth by
/// limiting the amount of sent but unacknowledged data to the specified window size
#[derive(Eq, PartialEq, Debug)]
pub struct SetPeerBandwidthMessage {
    pub size: u32,
    pub limit_type: PeerBandwidthLimitType
}

impl RtmpMessage for SetPeerBandwidthMessage {
    fn deserialize(data: Vec<u8>) -> Result<Self, MessageDeserializationError> {
        let mut cursor = Cursor::new(data);
        let size = try!(cursor.read_u32::<BigEndian>());
        let limit_type = match try!(cursor.read_u8()) {
            0 => PeerBandwidthLimitType::Hard,
            1 => PeerBandwidthLimitType::Soft,
            2 => PeerBandwidthLimitType::Dynamic,
            _ => return Err(MessageDeserializationError::InvalidMessageFormaat)
        };

        Ok(SetPeerBandwidthMessage{
            size: size,
            limit_type: limit_type
        })
    }

    fn serialize(self) -> Result<Vec<u8>, MessageSerializationError> {
        let type_id = match self.limit_type {
            PeerBandwidthLimitType::Hard => 0,
            PeerBandwidthLimitType::Soft => 1,
            PeerBandwidthLimitType::Dynamic => 2
        };

        let mut cursor = Cursor::new(Vec::new());
        try!(cursor.write_u32::<BigEndian>(self.size));
        try!(cursor.write_u8(type_id));

        Ok(cursor.into_inner())
    }

    fn get_type_id() -> u8 { 6 }
}


#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use byteorder::{BigEndian, WriteBytesExt};    
    
    use super::SetPeerBandwidthMessage;
    use super::PeerBandwidthLimitType;
    use rtmp_message::RtmpMessage;

    #[test]
    fn can_serialize_message_with_soft_limit_type() {
        let size = 523;
        let message = SetPeerBandwidthMessage { size: size, limit_type: PeerBandwidthLimitType::Soft };
        
        let mut cursor = Cursor::new(Vec::new());
        cursor.write_u32::<BigEndian>(size).unwrap();
        cursor.write_u8(1).unwrap();
        let expected = cursor.into_inner();

        let result = message.serialize().unwrap();
        assert_eq!(expected, result);
    }

    #[test]
    fn can_serialize_message_with_hard_limit_type() {
        let size = 523;
        let message = SetPeerBandwidthMessage { size: size, limit_type: PeerBandwidthLimitType::Hard };

        let mut cursor = Cursor::new(Vec::new());
        cursor.write_u32::<BigEndian>(size).unwrap();
        cursor.write_u8(0).unwrap();
        let expected = cursor.into_inner();

        let result = message.serialize().unwrap();
        assert_eq!(expected, result);
    }

    #[test]
    fn can_serialize_message_with_dynamic_limit_type() {
        let size = 523;
        let message = SetPeerBandwidthMessage { size: size, limit_type: PeerBandwidthLimitType::Dynamic };

        let mut cursor = Cursor::new(Vec::new());
        cursor.write_u32::<BigEndian>(size).unwrap();
        cursor.write_u8(2).unwrap();
        let expected = cursor.into_inner();

        let result = message.serialize().unwrap();
        assert_eq!(expected, result);
    }

    #[test]
    fn can_deserialize_message_with_hard_limit_type() {
        let size = 523;
        let expected = SetPeerBandwidthMessage { size: size, limit_type: PeerBandwidthLimitType::Hard };        

        let mut cursor = Cursor::new(Vec::new());
        cursor.write_u32::<BigEndian>(size).unwrap();
        cursor.write_u8(0).unwrap();
        let data = cursor.into_inner();

        let result = SetPeerBandwidthMessage::deserialize(data).unwrap();
        assert_eq!(expected, result);
    }

    #[test]
    fn can_deserialize_message_with_soft_limit_type() {
        let size = 523;
        let expected = SetPeerBandwidthMessage { size: size, limit_type: PeerBandwidthLimitType::Soft };        

        let mut cursor = Cursor::new(Vec::new());
        cursor.write_u32::<BigEndian>(size).unwrap();
        cursor.write_u8(1).unwrap();
        let data = cursor.into_inner();

        let result = SetPeerBandwidthMessage::deserialize(data).unwrap();
        assert_eq!(expected, result);
    }

    #[test]
    fn can_deserialize_message_with_dynamic_limit_type() {
        let size = 523;
        let expected = SetPeerBandwidthMessage { size: size, limit_type: PeerBandwidthLimitType::Dynamic };        

        let mut cursor = Cursor::new(Vec::new());
        cursor.write_u32::<BigEndian>(size).unwrap();
        cursor.write_u8(2).unwrap();
        let data = cursor.into_inner();

        let result = SetPeerBandwidthMessage::deserialize(data).unwrap();
        assert_eq!(expected, result);
    }

    #[test]
    fn gets_expected_type_id() {
        let result = SetPeerBandwidthMessage::get_type_id();
        assert_eq!(6, result);
    }
}