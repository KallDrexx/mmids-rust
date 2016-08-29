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

    pub fn handle(&mut self, _messages: Vec<RtmpMessageDetails>) -> ProcessorResult {
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

#[cfg(test)]
mod tests {
    use rtmp_message::RtmpMessage;

    use events::ProcessorEvent;
    use super::*;
    use tests::utils;

    #[test]
    fn self_chunk_size() {
        let mut processor = RtmpProcessor::new();
        let result = processor.set_chunk_size(4096);

        assert_eq!(result.events.len(), 1);
        assert_eq!(result.events[0], ProcessorEvent::SelfChunkSizeChanged{ new_chunk_size: 4096 });
        assert_eq!(result.messages.len(), 1);
        assert_eq!(result.messages[0].stream_id, 0);
        assert_eq!(result.messages[0].message, RtmpMessage::SetChunkSize{size: 4096});
    }
    
    #[test]
    fn self_window_ack_size() {
        let mut processor = RtmpProcessor::new();
        let result = processor.set_window_ack_size(1048576);

        assert_eq!(result.events.len(), 0);
        assert_eq!(result.messages.len(), 1);
        assert_eq!(result.messages[0].stream_id, 0);
        assert_eq!(result.messages[0].message, RtmpMessage::WindowAcknowledgement{size: 1048576});
    }

    #[test]
    fn peer_chunk_size() {
        let mut processor = RtmpProcessor::new();
        let result = processor.handle(vec![utils::create_set_chunk_size_message(4000)]);

        assert_eq!(result.events.len(), 1);
        assert_eq!(result.events[0], ProcessorEvent::PeerChunkSizeChanged{new_chunk_size: 4000});
        assert_eq!(result.messages.len(), 0);
    }

    #[test]
    fn peer_window_ack_size() {
        let mut processor = RtmpProcessor::new();
        let result = processor.handle(vec![utils::create_window_ack_message(5000000)]);

        assert_eq!(result.events.len(), 0);
        assert_eq!(result.messages.len(), 0);
    }

    #[test]
    fn connection_command_received_and_accepted() {
        let mut processor = RtmpProcessor::new();
        let app_name = "myapp".to_string();
        let command = utils::create_connect_command(app_name.clone());
        let request_id;

        let initial_result = processor.handle(vec![command]);
        assert_eq!(initial_result.messages.len(), 0);
        assert_eq!(initial_result.events.len(), 1);
        assert_match!(initial_result.events[0], 
            ProcessorEvent::ConnectionRequested { request_id: rid, application_name: ref name } if name == &app_name 
            => {request_id = rid});

        let accept_result = processor.accept_request(request_id);
        
    }
}