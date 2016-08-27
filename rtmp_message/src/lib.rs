//! This crate contains the functionality to work with RTMP messages
//!
//! # Examples
//!
//! Deserialize a message payload:
//!
//! ```
//! extern crate rtmp_message; // This crate
//! extern crate rtmp_time; // Required for working with timestamps in rtmp messages
//!
//! use rtmp_time::RtmpTimestamp;
//!
//! use rtmp_message::{MessagePayload, RtmpMessageDetails, RtmpMessage};
//! 
//! # fn main() { 
//!
//! let payload = MessagePayload {
//!     timestamp: RtmpTimestamp::new(5),
//!     stream_id: 5,
//!     type_id: 1,
//!     data: vec![0, 0, 0, 128]
//! };
//!
//! let details = RtmpMessageDetails::from_payload(payload).unwrap();
//! 
//! assert_eq!(details.rtmp_timestamp, RtmpTimestamp::new(5));
//! assert_eq!(details.stream_id, 5);
//! assert_eq!(details.message, RtmpMessage::SetChunkSize { size: 128 });
//!
//! # }
//! ```
//!
//! Serialize a RTMP message into a message payload:
//!
//! ``` 
//! extern crate rtmp_message; // This crate
//! extern crate rtmp_time; // Required for working with timestamps in rtmp messages
//!
//! use rtmp_time::RtmpTimestamp;
//!
//! use rtmp_message::{MessagePayload, RtmpMessage, RtmpMessageDetails};
//! 
//! # fn main() { 
//! let details = RtmpMessageDetails {
//!     rtmp_timestamp: RtmpTimestamp::new(5),
//!     stream_id: 5,
//!     message: RtmpMessage::SetChunkSize { size: 128 }    
//! };
//!
//! let payload = details.to_payload().unwrap();
//!
//! assert_eq!(payload.type_id, 1);
//! assert_eq!(payload.stream_id, 5);
//! assert_eq!(payload.timestamp, RtmpTimestamp::new(5));
//! assert_eq!(payload.data, vec![0, 0, 0, 128]);
//!
//! # }
//! ```


#[macro_use] extern crate quick_error;
extern crate byteorder;
extern crate rtmp_time;
extern crate amf0;

mod message_payload;
mod known_message_type;
mod errors;
mod rtmp_message;
mod rtmp_message_details;

pub use message_payload::MessagePayload;
pub use known_message_type::KnownMessageType;
pub use errors::MessageSerializationError;
pub use errors::MessageDeserializationError;
pub use rtmp_message::RtmpMessage;
pub use rtmp_message_details::RtmpMessageDetails;

mod messages {
    pub mod abort;
    pub mod acknowledgement;
    pub mod amf0_command;
    pub mod amf0_data;
    pub mod audio_data;
    pub mod set_chunk_size;
    pub mod set_peer_bandwidth;
    pub mod user_control;
    pub mod video_data;
    pub mod window_acknowledgement_size;


}