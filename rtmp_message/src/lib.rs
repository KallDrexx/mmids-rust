//! This crate contains the different RTMP message types, as well as functionality
//! for their serialization and deserialization

extern crate rtmp_time;

use rtmp_time::RtmpTimestamp;

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
}