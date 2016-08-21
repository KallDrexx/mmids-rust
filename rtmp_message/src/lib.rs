//! This crate contains the different RTMP message types, as well as functionality
//! for their serialization and deserialization

#[macro_use] extern crate quick_error;
extern crate byteorder;
extern crate rtmp_time;

mod message_payload;
mod known_message_type;
mod errors;
mod rtmp_message;

pub use message_payload::MessagePayload;
pub use known_message_type::KnownMessageType;
pub use errors::MessageSerializationError;
pub use errors::MessageDeserializationError;
pub use rtmp_message::{RtmpMessage, RtmpMessageDetails};

pub mod messages {
    mod abort;
    mod acknowledgement;
    mod video_data;
    mod audio_data;
    mod set_chunk_size;
    mod window_acknowledgement_size;
    mod set_peer_bandwidth;

    pub use self::abort::AbortMessage;
    pub use self::acknowledgement::AcknowledgementMessage;
    pub use self::video_data::VideoDataMessage;
    pub use self::audio_data::AudioDataMessage;
    pub use self::set_chunk_size::SetChunkSizeMessage;
    pub use self::window_acknowledgement_size::WindowAcknowledgementSizeMessage;
    pub use self::set_peer_bandwidth::{SetPeerBandwidthMessage, PeerBandwidthLimitType};
}
