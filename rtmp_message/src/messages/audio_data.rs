use errors::MessageDeserializationError;
use errors::MessageSerializationError;
use RtmpMessage;

/// Represents audio data
#[derive(Eq, PartialEq, Debug)]
pub struct AudioDataMessage {
    pub data: Vec<u8>
}

impl RtmpMessage for AudioDataMessage {
    fn deserialize(data: Vec<u8>) -> Result<Self, MessageDeserializationError> {
        Ok(AudioDataMessage {
            data: data
        })
    }

    fn serialize(self) -> Result<Vec<u8>, MessageSerializationError> {
        Ok(self.data)
    }

    fn get_type_id() -> u8 { 8 }
}

#[cfg(test)]
mod tests {
    use super::AudioDataMessage;
    use RtmpMessage;

    #[test]
    fn can_serialize_message() {
        let message = AudioDataMessage { data: vec![1,2,3,4] };
        let expected = vec![1,2,3,4];
        let result = message.serialize().unwrap();        
        
        assert_eq!(result, expected);
    }

    #[test]
    fn can_deserialize_message() {
        let data = vec![1,2,3,4];
        let expected = AudioDataMessage { data: vec![1,2,3,4] };
        let result = AudioDataMessage::deserialize(data).unwrap();
        
        assert_eq!(result, expected);
    }

    #[test]
    fn can_get_type_id() {
        let result = AudioDataMessage::get_type_id();
        assert_eq!(result, 8);
    }
}