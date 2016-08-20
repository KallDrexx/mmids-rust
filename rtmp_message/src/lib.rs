//! This crate contains the different RTMP message types, as well as functionality
//! for their serialization and deserialization

extern crate rtmp_time;

use rtmp_time::RtmpTimestamp;

#[derive(Eq, PartialEq, Debug)]
pub enum KnownMessageType {
    Abort,
    Acknowledgement,
    Amf0Command,
    Amf0Data,
    AudioData,
    SetChunkSize,
    SetPeerBandwidth,
    UserControl,
    VideoData,
    WindowAcknowledgement
}

/// Represents a complete (but raw) RTMP message
#[derive(PartialEq, Debug)]
pub struct MessagePayload {
    pub timestamp: RtmpTimestamp,
    pub type_id: u8,
    pub stream_id: u32,
    pub data: Vec<u8>
}

impl MessagePayload {
    pub fn new() -> MessagePayload {
        MessagePayload {
            timestamp: RtmpTimestamp::new(0),
            type_id: 0,
            stream_id: 0,
            data: Vec::new()
        }
    }

    pub fn get_message_type(&self) -> Option<KnownMessageType> {
        match self.type_id {
            1 => Some(KnownMessageType::SetChunkSize),
            2 => Some(KnownMessageType::Abort),
            3 => Some(KnownMessageType::Acknowledgement),
            4 => Some(KnownMessageType::UserControl),
            5 => Some(KnownMessageType::WindowAcknowledgement),
            6 => Some(KnownMessageType::SetPeerBandwidth),
            8 => Some(KnownMessageType::AudioData),
            9 => Some(KnownMessageType::VideoData),
            18 => Some(KnownMessageType::Amf0Data),
            20 => Some(KnownMessageType::Amf0Command),
            _ => None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_get_known_message_types() {
        struct Test { type_id: u8, expected_type: Option<KnownMessageType> }

        let tests = vec![
            Test { type_id: 1, expected_type: Some(KnownMessageType::SetChunkSize)},
            Test { type_id: 2, expected_type: Some(KnownMessageType::Abort)},
            Test { type_id: 3, expected_type: Some(KnownMessageType::Acknowledgement)},
            Test { type_id: 4, expected_type: Some(KnownMessageType::UserControl)},
            Test { type_id: 5, expected_type: Some(KnownMessageType::WindowAcknowledgement)},
            Test { type_id: 6, expected_type: Some(KnownMessageType::SetPeerBandwidth)},
            Test { type_id: 8, expected_type: Some(KnownMessageType::AudioData)},
            Test { type_id: 9, expected_type: Some(KnownMessageType::VideoData)},
            Test { type_id: 18, expected_type: Some(KnownMessageType::Amf0Data)},
            Test { type_id: 20, expected_type: Some(KnownMessageType::Amf0Command)},
            Test { type_id: 255, expected_type: None },
        ];

        for test_case in tests {
            let mut message = MessagePayload::new();
            message.type_id = test_case.type_id;

            let result = message.get_message_type();
            assert_eq!(result, test_case.expected_type);
        }
    }
}