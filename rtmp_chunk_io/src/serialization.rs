use std::cmp::min;
use std::io;
use std::io::{Cursor, Write};
use std::collections::HashMap;
use byteorder::{BigEndian, LittleEndian, WriteBytesExt};
use super::{ChunkHeader, ChunkHeaderFormat, MessagePayload};
use super::{write_u24_be};

const MAX_INITIAL_TIMESTAMP: u32 = 16777215;

/// Allows serializing message payloads into bytes representing rtmp chunks.
/// Note that it all operations against the Serializer are mutable due to
/// the RTMP chunking protocol compressing headers based on previously
/// serialized chunks.
///
/// Due to the nature of the RTMP chunking protocol, the same serializer should
/// be used for all messages that need to be sent to the same peer.
pub struct Serializer {
    previous_headers: HashMap<u16, ChunkHeader>,
    max_chunk_size: u32
}

impl Serializer {
    pub fn new() -> Serializer {
        Serializer {
            previous_headers: HashMap::new(),
            max_chunk_size: 128
        }
    }

    pub fn serialize(&mut self, message: &MessagePayload) -> Result<Vec<u8>, SerializationError> {
        // TODO: Verify message length is not larger than a 3 byte unsigned integer
        let header = ChunkHeader { 
            chunk_stream_id: get_csid_for_message_type(message.type_id),
            timestamp: message.timestamp,
            message_type_id: message.type_id,
            message_stream_id: message.stream_id,
            message_length: message.data.len() as u32
        };

        let header_format = ChunkHeaderFormat::Full;

        let mut bytes = Cursor::new(Vec::new());

        try!(add_basic_header(&mut bytes, &header_format, header.chunk_stream_id)
              .and_then(|_| add_initial_timestamp(&mut bytes, &header_format, header.timestamp))
              .and_then(|_| add_message_length_and_type_id(&mut bytes, &header_format, header.message_length, header.message_type_id))
              .and_then(|_| add_message_stream_id(&mut bytes, &header_format, header.message_stream_id))
              .and_then(|_| add_extended_timestamp(&mut bytes, &header_format, header.timestamp))
              .and_then(|_| add_message_payload(&mut bytes, &message.data)));

        Ok(bytes.into_inner())
    }
}

fn get_csid_for_message_type(message_type_id: u8) -> u32 {
    // Naive resolution, purpose (afaik) is to allow repeated messages
    // to utilize header compression by spreading them across chunk streams
    match message_type_id {
        1 | 2 | 3 | 4 | 5 | 6 => 2,
        18 | 19               => 3,
        9                     => 4,
        8                     => 5,
        _                     => 6
    }
}

fn add_basic_header(bytes: &mut Write, format: &ChunkHeaderFormat, csid: u32) -> Result<(), SerializationError> {
    debug_assert!(csid >= 1, "csid cannot be 0 or 1");
    debug_assert!(csid < 65600, "csid {} is above the max of 65599", csid);
    
    let format_mask = match *format {
        ChunkHeaderFormat::Full                            => 0b00000000,
        ChunkHeaderFormat::TimeDeltaWithoutMessageStreamId => 0b01000000,
        ChunkHeaderFormat::TimeDeltaOnly                   => 0b10000000,
        ChunkHeaderFormat::Empty                           => 0b11000000
    };

    let mut first_byte = match csid {
        x if x <= 63             => x as u8, 
        x if x >= 64 && x <= 319 => 0,
        _                        => 1
    };

    first_byte = first_byte | format_mask;
    try!(bytes.write_u8(first_byte));

    // TODO: Add bytes for 2 and 3 byte formats
    Ok(())
}

fn add_initial_timestamp(bytes: &mut Cursor<Vec<u8>>, format: &ChunkHeaderFormat, timestamp: u32) -> Result<(), SerializationError> {
    if *format == ChunkHeaderFormat::Empty {
        return Ok(());
    }

    let timestamp_to_write = min(timestamp, MAX_INITIAL_TIMESTAMP);
    try!(write_u24_be(bytes, timestamp_to_write));

    Ok(())
}

fn add_message_length_and_type_id(bytes: &mut Cursor<Vec<u8>>, format: &ChunkHeaderFormat, length: u32, type_id: u8) -> Result<(), SerializationError> {
    if *format == ChunkHeaderFormat::Empty || *format == ChunkHeaderFormat::TimeDeltaOnly {
        return Ok(());
    }

    try!(write_u24_be(bytes, length));
    try!(bytes.write_u8(type_id));
    Ok(())
}

fn add_message_stream_id(bytes: &mut Write, format: &ChunkHeaderFormat, stream_id: u32) -> Result<(), SerializationError> {
    if *format != ChunkHeaderFormat::Full {
        return Ok(());
    }

    try!(bytes.write_u32::<LittleEndian>(stream_id));
    Ok(())
}

fn add_extended_timestamp(bytes: &mut Write, format: &ChunkHeaderFormat, timestamp: u32) -> Result<(), SerializationError> {
    if *format == ChunkHeaderFormat::Empty {
        return Ok(());
    }

    if timestamp < MAX_INITIAL_TIMESTAMP {
        return Ok(());
    } 

    try!(bytes.write_u32::<BigEndian>(timestamp));
    Ok(())
}

fn add_message_payload(bytes: &mut Write, data: &[u8]) -> Result<(), SerializationError> {
    try!(bytes.write(data));
    Ok(())
}

quick_error! {
    #[derive(Debug)]
    pub enum SerializationError {
        Io(err: io::Error) {
            cause(err)
            description(err.description())
            from()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::get_csid_for_message_type;
    use super::super::*;

    #[test]
    fn first_message_for_csid_encodes_full_chunk() {
        let mut message = MessagePayload {
            timestamp: 72,
            type_id: 5,
            data: vec![1, 2, 3],
            stream_id: 1
        };

        let mut serializer = Serializer::new();
        let result = serializer.serialize(&message).unwrap();

        let expected_csid = get_csid_for_message_type(message.type_id); 
        let mut expected = vec![
            expected_csid as u8, 0, 0, message.timestamp as u8, 0, 0, 
            message.data.len() as u8, message.type_id, message.stream_id as u8, 0, 0, 0
        ];

        expected.append(&mut message.data);

        assert_eq!(result, expected);
    }
}