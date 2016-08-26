

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use byteorder::{BigEndian, WriteBytesExt};    
    
    use rtmp_message::{RtmpMessage, PeerBandwidthLimitType};

    #[test]
    fn can_serialize_message_with_soft_limit_type() {
        let size = 523;
        let message = RtmpMessage::SetPeerBandwidth { size: size, limit_type: PeerBandwidthLimitType::Soft };
        
        let mut cursor = Cursor::new(Vec::new());
        cursor.write_u32::<BigEndian>(size).unwrap();
        cursor.write_u8(1).unwrap();
        let expected = cursor.into_inner();

        let raw_message = message.serialize().unwrap();
        assert_eq!(raw_message.data, expected);
        assert_eq!(raw_message.type_id, 6);
    }

    #[test]
    fn can_serialize_message_with_hard_limit_type() {
        let size = 523;
        let message = RtmpMessage::SetPeerBandwidth { size: size, limit_type: PeerBandwidthLimitType::Hard };

        let mut cursor = Cursor::new(Vec::new());
        cursor.write_u32::<BigEndian>(size).unwrap();
        cursor.write_u8(0).unwrap();
        let expected = cursor.into_inner();

        let raw_message = message.serialize().unwrap();
        assert_eq!(raw_message.data, expected);
        assert_eq!(raw_message.type_id, 6);
    }

    #[test]
    fn can_serialize_message_with_dynamic_limit_type() {
        let size = 523;
        let message = RtmpMessage::SetPeerBandwidth { size: size, limit_type: PeerBandwidthLimitType::Dynamic };

        let mut cursor = Cursor::new(Vec::new());
        cursor.write_u32::<BigEndian>(size).unwrap();
        cursor.write_u8(2).unwrap();
        let expected = cursor.into_inner();

        let raw_message = message.serialize().unwrap();
        assert_eq!(raw_message.data, expected);
        assert_eq!(raw_message.type_id, 6);
    }

    #[test]
    fn can_deserialize_message_with_hard_limit_type() {
        let size = 523;
        let expected = RtmpMessage::SetPeerBandwidth { size: size, limit_type: PeerBandwidthLimitType::Hard };        

        let mut cursor = Cursor::new(Vec::new());
        cursor.write_u32::<BigEndian>(size).unwrap();
        cursor.write_u8(0).unwrap();
        let data = cursor.into_inner();

        let result = RtmpMessage::deserialize(data, 6).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn can_deserialize_message_with_soft_limit_type() {
        let size = 523;
        let expected = RtmpMessage::SetPeerBandwidth { size: size, limit_type: PeerBandwidthLimitType::Soft };        

        let mut cursor = Cursor::new(Vec::new());
        cursor.write_u32::<BigEndian>(size).unwrap();
        cursor.write_u8(1).unwrap();
        let data = cursor.into_inner();

        let result = RtmpMessage::deserialize(data, 6).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn can_deserialize_message_with_dynamic_limit_type() {
        let size = 523;
        let expected = RtmpMessage::SetPeerBandwidth { size: size, limit_type: PeerBandwidthLimitType::Dynamic };        

        let mut cursor = Cursor::new(Vec::new());
        cursor.write_u32::<BigEndian>(size).unwrap();
        cursor.write_u8(2).unwrap();
        let data = cursor.into_inner();

        let result = RtmpMessage::deserialize(data, 6).unwrap();
        assert_eq!(result, expected);
    }
}