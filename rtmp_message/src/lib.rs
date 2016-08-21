//! This crate contains the different RTMP message types, as well as functionality
//! for their serialization and deserialization

#[macro_use] extern crate quick_error;
extern crate byteorder;
extern crate rtmp_time;

pub use message_payload::MessagePayload;
pub use known_message_type::KnownMessageType;
pub use errors::MessageSerializationError;
pub use errors::MessageDeserializationError;

mod message_payload;
mod known_message_type;
mod errors;

pub mod messages {
    mod abort;

    pub use self::abort::AbortMessage;
}
