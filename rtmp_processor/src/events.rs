use metadata::StreamMetadata;

#[derive(PartialEq, Debug)]
pub enum ProcessorEvent {
    PeerChunkSizeChanged { new_chunk_size: u32 },
    ConnectionRequested { request_id: u32, application_name: String },
    ReleaseStreamRequested { request_id: u32, application_name: String, stream_key: String },
    PublishStreamRequested { request_id: u32, application_name: String, stream_key: String },
    StreamMetaDataChanged { application_name: String, stream_key: String, meta_data: StreamMetadata },
    AudioDataReceived { application_name: String, stream_key: String, data: Vec<u8> },
    VideoDataReceived { application_name: String, stream_key: String, data: Vec<u8> },
}