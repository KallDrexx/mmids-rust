use errors::MessageDeserializationError;
use errors::MessageSerializationError;
use RtmpMessage;

/// Represents video data
#[derive(Eq, PartialEq, Debug)]
pub struct VideoDataMessage {
    pub data: Vec<u8>
}

impl RtmpMessage for VideoDataMessage {
    fn deserialize(data: Vec<u8>) -> Result<Self, MessageDeserializationError> {
        Ok(VideoDataMessage {
            data: data
        })
    }

    fn serialize(self) -> Result<Vec<u8>, MessageSerializationError> {
        Ok(self.data)
    }

    fn get_type_id() -> u8 { 9 }
}

#[cfg(test)]
mod tests {
    use super::VideoDataMessage;
    use RtmpMessage;

    #[test]
    fn can_serialize_message() {
        let message = VideoDataMessage { data: vec![1,2,3,4] };
        let expected = vec![1,2,3,4];
        let result = message.serialize().unwrap();        
        
        assert_eq!(result, expected);
    }

    #[test]
    fn can_deserialize_message() {
        let data = vec![1,2,3,4];
        let expected = VideoDataMessage { data: vec![1,2,3,4] };
        let result = VideoDataMessage::deserialize(data).unwrap();
        
        assert_eq!(result, expected);
    }

    #[test]
    fn can_get_type_id() {
        let result = VideoDataMessage::get_type_id();
        assert_eq!(result, 9);
    }
}