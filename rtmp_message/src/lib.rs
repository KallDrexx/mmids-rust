//! This crate contains the different RTMP message types, as well as functionality
//! for their serialization and deserialization

extern crate rtmp_time;

pub use message_payload::MessagePayload;
pub use known_message_type::KnownMessageType;

mod message_payload;
mod known_message_type;
