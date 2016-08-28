// use std::collections::HashMap;
use rtmp_message::RtmpMessageDetails;

use events::ProcessorEvent;
// use stream::Stream;

pub struct ProcessorResult {
    pub messages: Vec<RtmpMessageDetails>,
    pub events: Vec<ProcessorEvent>
}

// enum ProcessorState {
//     Started,
    //ConnectionRequested,
    //ConnectionAccepted,
// }

pub struct RtmpProcessor {
    //current_state: ProcessorState,
    //application: Option<String>,
    //streams: HashMap<u8, Stream>, 
}

impl RtmpProcessor {
    pub fn new() -> Self {
        RtmpProcessor {
            //current_state: ProcessorState::Started,
            //application: None,
            //streams: HashMap::new(),
        }
    }

    pub fn handle(messages: Vec<RtmpMessageDetails>) -> ProcessorResult {
        unimplemented!()
    }

    /// Requests a new max size for messages to be sent
    pub fn set_chunk_size(&mut self, _new_size: u32) -> ProcessorResult {
        unimplemented!()
    }

    /// Requests a new acknowledgement window size for the processor 
    pub fn set_window_ack_size(&mut self, _new_size: u32) -> ProcessorResult {
        unimplemented!()
    }

    /// Signals that a request event that the processor previously raised 
    /// has been accepted
    pub fn accept_request(&mut self, _request_id: u32) -> ProcessorResult {
        unimplemented!();
    }

    /// Signals that a request event that the processor previously raised
    /// has been rejected
    pub fn reject_request(&mut self, _request_id: u32) -> ProcessorResult {
        unimplemented!()
    }
}