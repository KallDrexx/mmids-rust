use std::collections::HashMap;
use amf0::Amf0Value;
use rtmp_message::{RtmpMessage, RtmpMessageDetails};
use rtmp_time::RtmpTimestamp;


pub fn create_set_chunk_size_message(size: u32) -> RtmpMessageDetails {
    RtmpMessageDetails {
        rtmp_timestamp: RtmpTimestamp::new(0),
        stream_id: 0,
        message: RtmpMessage::SetChunkSize { size: size }
    }
}

pub fn create_window_ack_message(size: u32) -> RtmpMessageDetails {
    RtmpMessageDetails {
        rtmp_timestamp: RtmpTimestamp::new(0),
        stream_id: 0,
        message: RtmpMessage::WindowAcknowledgement { size: size }
    }
}

pub fn create_connect_command(app: String) -> RtmpMessageDetails {
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

pub fn create_publish_command(stream_key: String, transaction_id: f64) -> RtmpMessageDetails {
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

pub fn create_createStream_command(transaction_id: f64) -> RtmpMessageDetails {
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

pub fn create_audio_data_message(data: Vec<u8>) -> RtmpMessageDetails {
    RtmpMessageDetails {
        rtmp_timestamp: RtmpTimestamp::new(0),
        stream_id: 0,
        message: RtmpMessage::AudioData { data: data }
    }
}

pub fn create_video_data_message(data: Vec<u8>) -> RtmpMessageDetails {
    RtmpMessageDetails {
        rtmp_timestamp: RtmpTimestamp::new(0),
        stream_id: 0,
        message: RtmpMessage::VideoData { data: data }
    }
}