use rtmp_time::RtmpTimestamp;
use amf0::Amf0Value;

use errors::{MessageDeserializationError, MessageSerializationError};

#[derive(Eq, PartialEq, Debug)]
pub enum PeerBandwidthLimitType { Hard, Soft, Dynamic }

#[derive(Eq, PartialEq, Debug)]
pub enum UserControlEventType {
    StreamBegin,
    StreamEof,
    StreamDry,
    SetBufferLength,
    StreamIsRecorded,
    PingRequest,
    PingResponse
}

#[derive(PartialEq, Debug)]
pub enum RtmpMessage {
    Unknown { type_id: u8, data: Vec<u8> },
    Abort { stream_id: u32 },
    Acknowledgement { sequence_number: u32 },
    Amf0Command { command_name: String, transaction_id: f64, command_object: Amf0Value, additional_arguments: Vec<Amf0Value> },
    Amf0Data { values: Vec<Amf0Value> },
    AudioData { data: Vec<u8> },
    SetChunkSize { size: u32 },
    SetPeerBandwidth { size: u32, limit_type: PeerBandwidthLimitType },
    UserControl { event_type: UserControlEventType, stream_id: Option<u32>, buffer_length: Option<u32>, timestamp: Option<RtmpTimestamp> },
    VideoData { data: Vec<u8> },
    WindowAcknowledgement { size: u32 }
}

#[derive(PartialEq, Debug)]
pub struct RawRtmpMessage {
    pub data: Vec<u8>,
    pub type_id: u8
}

impl RtmpMessage {
    pub fn serialize(self) -> Result<RawRtmpMessage, MessageSerializationError> {
        match self {
            RtmpMessage::Unknown { type_id, data } => Ok(RawRtmpMessage { type_id: type_id, data: data }),
            RtmpMessage::Abort { stream_id: _ } => unimplemented!(),
            RtmpMessage::Acknowledgement { sequence_number: _ } => unimplemented!(),
            RtmpMessage::Amf0Command { command_name: _, transaction_id: _, command_object: _, additional_arguments: _ } => unimplemented!(),
            RtmpMessage::Amf0Data { values: _ } => unimplemented!(),
            RtmpMessage::AudioData { data: _ } => unimplemented!(),
            RtmpMessage::SetChunkSize { size: _ } => unimplemented!(),
            RtmpMessage::SetPeerBandwidth { size: _, limit_type: _ } => unimplemented!(),
            RtmpMessage::UserControl { event_type: _, stream_id: _, buffer_length: _, timestamp: _ } => unimplemented!(),
            RtmpMessage::VideoData { data: _ } => unimplemented!(),
            RtmpMessage::WindowAcknowledgement { size: _ } => unimplemented!()
        }
    }

    pub fn deserialize(bytes: Vec<u8>, type_id: u8) -> Result<Self, MessageDeserializationError> {
        match type_id {
            _ => Ok(RtmpMessage::Unknown { type_id: type_id, data: bytes })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{RtmpMessage, RawRtmpMessage};

    #[test]
    fn can_serialize_unknown_message() {
        let message = RtmpMessage::Unknown { type_id: 29, data: vec![1,2,3,4] };
        let expected = RawRtmpMessage { type_id: 29, data: vec![1,2,3,4] };

        let result = message.serialize().unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn can_deserialize_unknown_message() {
        let id = 255;
        let expected = RtmpMessage::Unknown { type_id: id, data: vec![1,2,3,4] };
        let result = RtmpMessage::deserialize(vec![1,2,3,4], id).unwrap();        
        assert_eq!(expected, result);
    }
}