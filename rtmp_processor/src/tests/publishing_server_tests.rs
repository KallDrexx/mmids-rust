use std::collections::HashMap;
use amf0::Amf0Value;
use rtmp_message::{RtmpMessage, RtmpMessageDetails, UserControlEventType};
use rtmp_time::RtmpTimestamp;

use processor::RtmpProcessor;
use processor::ProcessorEvent;

#[test]
/// Represents server side flow from 7.3.1 in RTMP specification
fn can_handle_publishing_client_example_from_specification() {
    let mut processor = RtmpProcessor::new();

    let chunk_size_result = processor.set_chunk_size(4096);
    assert_eq!(chunk_size_result.events.len(), 0);
    assert_eq!(chunk_size_result.messages.len(), 1);
    assert_eq!(chunk_size_result.messages[0].stream_id, 0);
    assert_eq!(chunk_size_result.messages[0].message, RtmpMessage::SetChunkSize{size: 4096});

    let window_ack_result = processor.set_window_ack_size(1048576);
    assert_eq!(window_ack_result.events.len(), 0);
    assert_eq!(window_ack_result.messages.len(), 1);
    assert_eq!(window_ack_result.messages[0].stream_id, 0);
    assert_eq!(window_ack_result.messages[0].message, RtmpMessage::WindowAcknowledgement{size: 1048576});
       
    let initial_result = processor.handle(vec![create_set_chunk_size_message(size: 4000)]);
    assert_eq!(initial_result.events.len(), 0);
    assert_eq!(initial_result.events[0], ProcessorEvent::PeerChunkSizeChanged{new_chunk_size: 4000});
    assert_eq!(initial_result.messages.len(), 0);

    
    
    unimplemented!();
}

fn create_set_chunk_size_message(size: u32) -> RtmpMessageDetails {
    RtmpMessageDetails {
        rtmp_timestamp: RtmpTimestamp::new(0),
        stream_id: 0,
        message: RtmpMessage::SetChunkSize { size: size }
    }
}

fn create_window_ack_message(size: u32) -> RtmpMessageDetails {
    RtmpMessageDetails {
        rtmp_timestamp: RtmpTimestamp::new(0),
        stream_id: 0,
        message: RtmpMessage::WindowAcknowledgement { size: size }
    }
}

fn create_stream_begin_message(stream_id: u32) -> RtmpMessageDetails {
    RtmpMessageDetails {
        rtmp_timestamp: RtmpTimestamp::new(0),
        stream_id: 0,
        message: RtmpMessage::UserControl {
            event_type: UserControlEventType::StreamBegin,
            stream_id: Some(stream_id),
            buffer_length: None,
            timestamp: None 
        }
    }
}

fn create_success_response_message(transaction_id: f64, object: Amf0Value, args: Vec<Amf0Value>) -> RtmpMessageDetails {
    RtmpMessageDetails {
        rtmp_timestamp: RtmpTimestamp::new(0),
        stream_id: 0,
        message: RtmpMessage::Amf0Command {
            command_name: "_result".to_string(),
            transaction_id: transaction_id,
            command_object: object,
            additional_arguments: args
        }
    }
}

fn create_connect_command(app: String) -> RtmpMessageDetails {
    let mut properties = HashMap::new();
    properties.insert("app".to_string(), Amf0Value::Utf8String(app));
    properties.insert("type".to_string(), Amf0Value::Utf8String("nonprivate".to_string()));
    properties.insert("flashVer".to_string(), Amf0Value::Utf8String("FMLE/3.0 (compatible; FMSc/1.0)".to_string()));
    properties.insert("swfUrl".to_string(), Amf0Value::Utf8String("rtmp://127.0.0.1/live".to_string()));
    properties.insert("tcUrl".to_string(), Amf0Value::Utf8String("rtmp://127.0.0.1/live".to_string()));

    RtmpMessageDetails {
        rtmp_timestamp: RtmpTimestamp::new(0),
        stream_id: 0,
        message: RtmpMessage::Amf0Command {
            command_name: "connect".to_string(),
            transaction_id: 1.0,
            command_object: Amf0Value::Object(properties),
            additional_arguments: vec![]
        }
    }
}

fn create_publish_command(stream_key: String, transaction_id: f64) -> RtmpMessageDetails {
    RtmpMessageDetails {
        rtmp_timestamp: RtmpTimestamp::new(0),
        stream_id: 0,
        message: RtmpMessage::Amf0Command {
            command_name: "publish".to_string(),
            transaction_id: transaction_id,
            command_object: Amf0Value::Null,
            additional_arguments: vec![
                Amf0Value::Utf8String(stream_key),
                Amf0Value::Utf8String("live".to_string()),
            ]
        }
    }
}

fn create_createStream_command(transaction_id: f64) -> RtmpMessageDetails {
    RtmpMessageDetails {
        rtmp_timestamp: RtmpTimestamp::new(0),
        stream_id: 0,
        message: RtmpMessage::Amf0Command {
            command_name: "createStream".to_string(),
            transaction_id: transaction_id,
            command_object: Amf0Value::Null,
            additional_arguments: vec![
            ]
        }
    }
}

fn create_audio_data_message(data: Vec<u8>) -> RtmpMessageDetails {
    RtmpMessageDetails {
        rtmp_timestamp: RtmpTimestamp::new(0),
        stream_id: 0,
        message: RtmpMessage::AudioData { data: data }
    }
}

fn create_video_data_message(data: Vec<u8>) -> RtmpMessageDetails {
    RtmpMessageDetails {
        rtmp_timestamp: RtmpTimestamp::new(0),
        stream_id: 0,
        message: RtmpMessage::VideoData { data: data }
    }
}
