#[macro_use] extern crate quick_error;
extern crate amf0;
extern crate rtmp_message;
extern crate rtmp_time;

#[macro_use] mod macros;
mod processor;
mod events;
mod metadata;
mod stream;
mod errors;

pub use events::ProcessorEvent;
pub use metadata::StreamMetadata;
pub use processor::{RtmpProcessor, ProcessorResult};
pub use errors::RtmpProcessorError;

#[cfg(test)]
mod tests{
    pub mod utils;
}