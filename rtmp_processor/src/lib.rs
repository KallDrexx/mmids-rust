extern crate amf0;
extern crate rtmp_message;
extern crate rtmp_time;

mod processor;
mod events;
mod metadata;
mod stream;

pub use events::HandlerEvent;
pub use metadata::StreamMetadata;
pub use processor::{RtmpProcessor, ProcessorResult};

#[cfg(test)]
mod tests{
    mod publishing_server_tests;
}