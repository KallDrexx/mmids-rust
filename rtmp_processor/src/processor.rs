use std::collections::HashMap;
use std::num::Wrapping;
//use amf0; // So serialize and deserialize methods are not brought into immediate scope
use amf0::Amf0Value;
use rtmp_message::{RtmpMessage, RtmpMessageDetails, PeerBandwidthLimitType};
use rtmp_time::RtmpTimestamp;

use events::ProcessorEvent;
use errors::RtmpProcessorError;

#[derive(PartialEq, Debug)]
pub enum ProcessorResult {
    ResponseMessage(RtmpMessageDetails),
    RaisedEvent(ProcessorEvent),
    UnhandleableMessage(RtmpMessageDetails),
}

pub struct RtmpProcessorConfig {
    pub version: String,
    pub peer_bandwidth: u32,
    pub window_ack_size: u32
}

enum ProcessorState {
    Started,
    ConnectionRequested,
    ConnectionAccepted,
}

enum OutstandingRequest {
    Connection { app: String }
}

pub struct RtmpProcessor {
    config: RtmpProcessorConfig,
    current_state: ProcessorState,
    next_request_id: Wrapping<u32>,
    outstanding_requests: HashMap<u32, OutstandingRequest>,
    application_name: Option<String>
}

impl RtmpProcessor {
    pub fn new(config: RtmpProcessorConfig) -> Self {
        RtmpProcessor {
            config: config,
            current_state: ProcessorState::Started,
            next_request_id: Wrapping(0),
            outstanding_requests: HashMap::new(),
            application_name: None
        }
    }

    pub fn handle(&mut self, messages: Vec<RtmpMessageDetails>) -> Result<Vec<ProcessorResult>, RtmpProcessorError> {
        let mut all_results = Vec::new();

        for details in messages.into_iter() {
            let mut message_results = match details.message {
                RtmpMessage::Amf0Command{command_name, transaction_id, command_object, additional_arguments}
                    => try!(self.handle_amf0_command(details.stream_id, command_name, transaction_id, command_object, additional_arguments)),

                RtmpMessage::SetChunkSize{size} 
                    => self.handle_peer_chunk_size(size),

                RtmpMessage::WindowAcknowledgement{size} 
                    => self.handle_peer_window_ack(size),

                _ => vec![ProcessorResult::UnhandleableMessage(details)]
            };

            all_results.append(&mut message_results);
        };

        Ok(all_results)
    }

    /// Requests a new max size for messages to be sent
    pub fn set_chunk_size(&mut self, new_size: u32) -> Vec<ProcessorResult> {
        vec![
            ProcessorResult::RaisedEvent(ProcessorEvent::SelfChunkSizeChanged { new_chunk_size: new_size }),
            ProcessorResult::ResponseMessage(RtmpMessageDetails{
                rtmp_timestamp: RtmpTimestamp::new(0),
                stream_id: 0,
                message: RtmpMessage::SetChunkSize { size: new_size }
            })
        ]
    }

    /// Signals that a request event that the processor previously raised 
    /// has been accepted
    pub fn accept_request(&mut self, request_id: u32) -> Result<Vec<ProcessorResult>, RtmpProcessorError> {
        let request = match self.outstanding_requests.remove(&request_id) {
            Some(request) => request,
            None => return Err(RtmpProcessorError::UnknownRequestId)
        };

        match request {
            OutstandingRequest::Connection{app} => Ok(accept_connection_request(self, app))
        }
    }

    /// Signals that a request event that the processor previously raised
    /// has been rejected
    pub fn reject_request(&mut self, _request_id: u32) -> Vec<ProcessorResult> {
        unimplemented!()
    }

    fn handle_peer_chunk_size(&mut self, size: u32) -> Vec<ProcessorResult> {
        vec![
            ProcessorResult::RaisedEvent(ProcessorEvent::PeerChunkSizeChanged { new_chunk_size: size })
        ]
    }

    fn handle_peer_window_ack(&mut self, _size: u32) -> Vec<ProcessorResult> {
        vec![] // TODO: add code to track bytes to act on acknowledgements
    }

    fn handle_amf0_command(&mut self,
        stream_id: u32, 
        command_name: String, 
        transaction_id: f64, 
        command_object: Amf0Value, 
        _additional_arguments: Vec<Amf0Value>) -> Result<Vec<ProcessorResult>, RtmpProcessorError> {
            
        match command_name.as_ref() {
            "connect" => handle_connect_amf0_command(self, stream_id, transaction_id, command_object),
            _ => Ok(handle_unknown_amf0_command(stream_id, command_name, transaction_id)),
        }
    }

    fn get_next_request_id(&mut self) -> Result<u32, RtmpProcessorError> {
        let last_id = self.next_request_id - Wrapping(1);
        let mut id = self.next_request_id;
        
        loop {
            if !self.outstanding_requests.contains_key(&id.0) {
                break;
            }
            
            // Make sure we haven't gone through and all u32 id numbers are in use.  This
            // should only happen if we are leaking keys
            if id == last_id {
                return Err(RtmpProcessorError::AllRequestIdsInUse);
            }
            
            id = id + Wrapping(1);
        }        

        self.next_request_id = id + Wrapping(1);
        Ok(id.0)
    }
}

fn handle_unknown_amf0_command(stream_id: u32, command_name: String, transaction_id: f64) -> Vec<ProcessorResult> {
    vec![
        get_amf0_error_response(stream_id, transaction_id, Amf0Value::Null),
        ProcessorResult::RaisedEvent(ProcessorEvent::UnhandleableAmf0Command{command_name: command_name})
    ]
}

fn handle_connect_amf0_command(processor: &mut RtmpProcessor, 
    stream_id: u32,  
    transaction_id: f64, 
    command_object: Amf0Value) -> Result<Vec<ProcessorResult>, RtmpProcessorError> {

    let mut properties;
    match command_object {
        Amf0Value::Object(props) => properties = props,
        _ => return Ok(vec![get_amf0_error_response(stream_id, transaction_id, Amf0Value::Null)])
    };

    let app_name = match properties.remove("app") {
        Some(Amf0Value::Utf8String(name)) => name,
        Some(_) => return Ok(vec![get_amf0_error_response(stream_id, transaction_id, Amf0Value::Null)]),
        None => return Ok(vec![get_amf0_error_response(stream_id, transaction_id, Amf0Value::Null)]),
    };

    let request = OutstandingRequest::Connection{app: app_name.clone()};
    let request_id = try!(processor.get_next_request_id());

    processor.outstanding_requests.insert(request_id, request);
    processor.current_state = ProcessorState::ConnectionRequested;

    Ok(vec![
        ProcessorResult::ResponseMessage(RtmpMessageDetails {
            rtmp_timestamp: RtmpTimestamp::new(0),
            stream_id: 0,
            message: RtmpMessage::SetPeerBandwidth{size: processor.config.peer_bandwidth, limit_type: PeerBandwidthLimitType::Hard}
        }),

        ProcessorResult::ResponseMessage(RtmpMessageDetails {
            rtmp_timestamp: RtmpTimestamp::new(0),
            stream_id: 0,
            message: RtmpMessage::WindowAcknowledgement { size: processor.config.window_ack_size }
        }),

        ProcessorResult::RaisedEvent(ProcessorEvent::ConnectionRequested{
            request_id: request_id,
            application_name: app_name
        })
    ])
}

fn accept_connection_request(processor: &mut RtmpProcessor, app_name: String) -> Vec<ProcessorResult> {
    processor.current_state = ProcessorState::ConnectionAccepted;
    processor.application_name = Some(app_name);

    let mut command_properties = HashMap::new();
    command_properties.insert("fmsVer".to_string(), Amf0Value::Utf8String(processor.config.version.clone()));
    command_properties.insert("capabilities".to_string(), Amf0Value::Number(31.0));

    let mut information_properties = HashMap::new();
    information_properties.insert("level".to_string(), Amf0Value::Utf8String("status".to_string()));
    information_properties.insert("code".to_string(), Amf0Value::Utf8String("NetConnection.Connect.Success".to_string()));
    information_properties.insert("description".to_string(), Amf0Value::Utf8String("Connection succeeded".to_string()));
    information_properties.insert("objectEncoding".to_string(), Amf0Value::Number(0.0));

    vec![
        ProcessorResult::ResponseMessage(RtmpMessageDetails {
            rtmp_timestamp: RtmpTimestamp::new(0),
            stream_id: 0,
            message: RtmpMessage::Amf0Command {
                command_name: "_result".to_string(),
                transaction_id: 1.0,
                command_object: Amf0Value::Object(command_properties),
                additional_arguments: vec![Amf0Value::Object(information_properties)]
            }
        })
    ]
}

fn get_amf0_error_response(stream_id: u32, transaction_id: f64, response_data: Amf0Value) -> ProcessorResult {
    ProcessorResult::ResponseMessage(RtmpMessageDetails {
        rtmp_timestamp: RtmpTimestamp::new(0),
        stream_id: stream_id,
        message: RtmpMessage::Amf0Command{
            command_name: "_error".to_string(),
            transaction_id: transaction_id,
            command_object: response_data,
            additional_arguments: vec![]
        }
    })
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use amf0::Amf0Value;
    use rtmp_message::{RtmpMessage, RtmpMessageDetails, PeerBandwidthLimitType};

    use events::ProcessorEvent;
    use super::*;
    use tests::utils;

    #[test]
    fn self_chunk_size() {
        let mut processor = RtmpProcessor::new(get_default_config());
        let result = processor.set_chunk_size(4096);

        assert_vec_match!(result,
            ProcessorResult::RaisedEvent(ProcessorEvent::SelfChunkSizeChanged { new_chunk_size: 4096 }),
            ProcessorResult::ResponseMessage(RtmpMessageDetails {
                stream_id: 0,
                rtmp_timestamp: _,
                message: RtmpMessage::SetChunkSize{size: 4096}
            })
        );
    }

    #[test]
    fn peer_chunk_size() {
        let mut processor = RtmpProcessor::new(get_default_config());
        let result = processor.handle(vec![utils::create_set_chunk_size_message(4000)]).unwrap();

        assert_vec_match!(result,
            ProcessorResult::RaisedEvent(ProcessorEvent::PeerChunkSizeChanged{new_chunk_size: 4000})
        );
    }

    #[test]
    fn peer_window_ack_size() {
        let mut processor = RtmpProcessor::new(get_default_config());
        let result = processor.handle(vec![utils::create_window_ack_message(5000000)]).unwrap();

        assert_vec_match!(result); // should be empty
    }

    #[test]
    fn connection_command_received_and_accepted() {
        let bandwidth_size = 60000;
        let window_ack_size = 70000;
        let mut config = get_default_config();
        config.peer_bandwidth = bandwidth_size;
        config.window_ack_size = window_ack_size;

        let mut processor = RtmpProcessor::new(config);
        let app_name = "myapp".to_string();
        let command = utils::create_connect_command(app_name.clone());
        let request_id;

        let initial_result = processor.handle(vec![command]).unwrap();
        assert_vec_match!(initial_result,
            ProcessorResult::ResponseMessage(RtmpMessageDetails {
                rtmp_timestamp: _,
                stream_id: 0,
                message: RtmpMessage::SetPeerBandwidth{size, limit_type: PeerBandwidthLimitType::Hard}
            }) if size == bandwidth_size,

            ProcessorResult::ResponseMessage(RtmpMessageDetails {
                rtmp_timestamp: _,
                stream_id: 0,
                message: RtmpMessage::WindowAcknowledgement { size }
            }) if size == window_ack_size,

            ProcessorResult::RaisedEvent(ProcessorEvent::ConnectionRequested { request_id: rid, application_name: ref name }) if name == &app_name
                => {request_id = rid}
        );

        let mut command_object_properties = HashMap::new();
        command_object_properties.insert("fmsVer".to_string(), Amf0Value::Utf8String("version".to_string()));
        command_object_properties.insert("capabilities".to_string(), Amf0Value::Number(31.0));

        let mut information_properties = HashMap::new();
        information_properties.insert("level".to_string(), Amf0Value::Utf8String("status".to_string()));
        information_properties.insert("code".to_string(), Amf0Value::Utf8String("NetConnection.Connect.Success".to_string()));
        information_properties.insert("description".to_string(), Amf0Value::Utf8String("Connection succeeded".to_string()));
        information_properties.insert("objectEncoding".to_string(), Amf0Value::Number(0.0));

        let command_object = Amf0Value::Object(command_object_properties);
        let arguments = vec![Amf0Value::Object(information_properties)];

        let accept_result = processor.accept_request(request_id).unwrap();
        assert_vec_match!(accept_result,
            ProcessorResult::ResponseMessage(RtmpMessageDetails{
                rtmp_timestamp: _,
                stream_id: 0,
                message: RtmpMessage::Amf0Command{
                    command_name: ref name,
                    transaction_id: 1.0,
                    command_object: ref obj,
                    additional_arguments: ref args
                }
            }) if name == "_result" && 
                    args == &arguments &&
                    obj == &command_object
        );
    }

    fn get_default_config() -> RtmpProcessorConfig {
        RtmpProcessorConfig {
            version: "version".to_string(),
            peer_bandwidth: 50000,
            window_ack_size: 50000,
        }
    }
}