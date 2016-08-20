use std::cmp::min;
use std::io;
use std::io::{Cursor, Write};
use std::collections::HashMap;
use byteorder::{BigEndian, LittleEndian, WriteBytesExt};
use rtmp_time::RtmpTimestamp;
use super::{ChunkHeader, ChunkHeaderFormat};
use super::{write_u24_be};
use rtmp_message::MessagePayload;

const MAX_INITIAL_TIMESTAMP: u32 = 16777215;

/// Allows serializing message payloads into bytes representing rtmp chunks.
/// Note that it all operations against the Serializer are mutable due to
/// the RTMP chunking protocol compressing headers based on previously
/// serialized chunks.
///
/// Due to the nature of the RTMP chunking protocol, the same serializer should
/// be used for all messages that need to be sent to the same peer.
pub struct Serializer {
    previous_headers: HashMap<u32, ChunkHeader>,
    max_chunk_size: u32
}

quick_error! {
    #[derive(Debug)]
    pub enum SerializationError {
        MessageTooLong {
            description("An individaul RTMP message can not be larger than 16777215 bytes")
        }

        Io(err: io::Error) {
            cause(err)
            description(err.description())
            from()
        }
    }
}


impl Serializer {
    pub fn new() -> Serializer {
        Serializer {
            previous_headers: HashMap::new(),
            max_chunk_size: 128
        }
    }

    pub fn set_max_chunk_size(&mut self, max_chunk_size: u32) {
        self.max_chunk_size = max_chunk_size;
    }

    pub fn serialize(&mut self, message: &MessagePayload, force_uncompressed: bool) -> Result<Vec<u8>, SerializationError> {
        if message.data.len() > 16777215 {
            return Err(SerializationError::MessageTooLong);
        }

        let mut bytes = Cursor::new(Vec::new());

        // Since a message may have a payload greater than one chunk allows, we must
        // split the payload into slices that don't exceed the max chunk length
        let mut slices = Vec::<&[u8]>::new();
        let mut iteration = 0;
        loop {
            let start_index = iteration * self.max_chunk_size as usize;
            if start_index >= message.data.len() {
                break;
            }

            let remaining_length = message.data.len() - start_index;
            let end_index = min(start_index + self.max_chunk_size as usize, start_index + remaining_length);

            slices.push(&message.data[start_index..end_index]);

            iteration = iteration + 1;
        }

        for slice in slices.into_iter() {
            try!(add_chunk(self, &mut bytes, force_uncompressed, message, slice));
        }
        
        Ok(bytes.into_inner())
    }
}

fn add_chunk(serializer: &mut Serializer, bytes: &mut Cursor<Vec<u8>>, force_uncompressed: bool, 
                message: &MessagePayload, data_to_write: &[u8]) -> Result<(), SerializationError> {
    let mut header = ChunkHeader { 
        chunk_stream_id: get_csid_for_message_type(message.type_id),
        timestamp: message.timestamp,
        timestamp_delta: 0,
        message_type_id: message.type_id,
        message_stream_id: message.stream_id,
        message_length: message.data.len() as u32
    };        

    let header_format = if force_uncompressed {
        ChunkHeaderFormat::Full
    } else {
        match serializer.previous_headers.get(&header.chunk_stream_id) {
            None => ChunkHeaderFormat::Full,
            Some(ref previous_header) => get_header_format(&mut header, &previous_header)
        }
    };        

    try!(add_basic_header(bytes, &header_format, header.chunk_stream_id)
            .and_then(|_| add_initial_timestamp(bytes, &header_format, header.timestamp))
            .and_then(|_| add_message_length_and_type_id(bytes, &header_format, header.message_length, header.message_type_id))
            .and_then(|_| add_message_stream_id(bytes, &header_format, header.message_stream_id))
            .and_then(|_| add_extended_timestamp(bytes, &header_format, header.timestamp))
            .and_then(|_| add_message_payload(bytes, data_to_write)));
    
    serializer.previous_headers.insert(header.chunk_stream_id, header);
    Ok(())
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

fn add_initial_timestamp(bytes: &mut Cursor<Vec<u8>>, format: &ChunkHeaderFormat, timestamp: RtmpTimestamp) -> Result<(), SerializationError> {
    if *format == ChunkHeaderFormat::Empty {
        return Ok(());
    }

    let timestamp_to_write = min(timestamp.value, MAX_INITIAL_TIMESTAMP);
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

fn add_extended_timestamp(bytes: &mut Write, format: &ChunkHeaderFormat, timestamp: RtmpTimestamp) -> Result<(), SerializationError> {
    if *format == ChunkHeaderFormat::Empty {
        return Ok(());
    }

    if timestamp < MAX_INITIAL_TIMESTAMP {
        return Ok(());
    } 

    try!(bytes.write_u32::<BigEndian>(timestamp.value));
    Ok(())
}

fn add_message_payload(bytes: &mut Write, data: &[u8]) -> Result<(), SerializationError> {
    try!(bytes.write(data));
    Ok(())
}

fn get_header_format(current_header: &mut ChunkHeader, previous_header: &ChunkHeader) -> ChunkHeaderFormat {
    if current_header.message_stream_id != previous_header.message_stream_id {
        return ChunkHeaderFormat::Full;
    }

    // TODO: Update to support rtmp time wrap-around
    let time_delta = current_header.timestamp - previous_header.timestamp;
    current_header.timestamp_delta = time_delta.value;

    if current_header.message_type_id != previous_header.message_type_id || current_header.message_length != previous_header.message_length {
        return ChunkHeaderFormat::TimeDeltaWithoutMessageStreamId;
    }

    if current_header.timestamp_delta != previous_header.timestamp_delta {
        return ChunkHeaderFormat::TimeDeltaOnly;
    }

    ChunkHeaderFormat::Empty  
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::get_csid_for_message_type;
    use rtmp_time::RtmpTimestamp;
    use rtmp_message::MessagePayload;

    #[test]
    fn first_message_for_csid_encodes_full_chunk() {
        let mut message = MessagePayload {
            timestamp: RtmpTimestamp::new(72),
            type_id: 5,
            data: vec![1, 2, 3],
            stream_id: 1
        };

        let mut serializer = Serializer::new();
        let result = serializer.serialize(&message, false).unwrap();

        let expected_csid = get_csid_for_message_type(message.type_id); 
        let mut expected = vec![
            expected_csid as u8, 0, 0, message.timestamp.value as u8, 0, 0, 
            message.data.len() as u8, message.type_id, message.stream_id as u8, 0, 0, 0
        ];

        expected.append(&mut message.data);

        assert_eq!(result, expected);
    }

    #[test]
    fn second_chunk_with_same_sid_but_different_message_length() {
        let message1 = MessagePayload {
            timestamp: RtmpTimestamp::new(72),
            type_id: 5,
            data: vec![1, 2, 3],
            stream_id: 1
        };

        let mut message2 = MessagePayload {
            timestamp: RtmpTimestamp::new(82),
            type_id: 5,
            data: vec![1, 2],
            stream_id: 1
        };

        let mut serializer = Serializer::new();
        serializer.serialize(&message1, false).unwrap();

        let result = serializer.serialize(&message2, false).unwrap();

        let expected_csid = get_csid_for_message_type(message2.type_id) | 0b01000000; 
        let mut expected = vec![
            expected_csid as u8, 0, 0, message2.timestamp.value as u8, 0, 0, 
            message2.data.len() as u8, message2.type_id,
        ];

        expected.append(&mut message2.data);
        assert_eq!(result, expected);
    }

    #[test]
    fn second_chunk_with_same_sid_message_length_and_type_id() {
        let message1 = MessagePayload {
            timestamp: RtmpTimestamp::new(72),
            type_id: 5,
            data: vec![1, 2, 3],
            stream_id: 1
        };

        let mut message2 = MessagePayload {
            timestamp: RtmpTimestamp::new(82),
            type_id: 5,
            data: vec![1, 2, 3],
            stream_id: 1
        };

        let mut serializer = Serializer::new();
        serializer.serialize(&message1, false).unwrap();

        let result = serializer.serialize(&message2, false).unwrap();

        let expected_csid = get_csid_for_message_type(message2.type_id) | 0b10000000; 
        let mut expected = vec![
            expected_csid as u8, 0, 0, message2.timestamp.value as u8
        ];

        expected.append(&mut message2.data);
        assert_eq!(result, expected);
    }

    #[test]
    fn third_chunk_same_everything() {
        let message1 = MessagePayload {
            timestamp: RtmpTimestamp::new(72),
            type_id: 5,
            data: vec![1, 2, 3],
            stream_id: 1
        };

        let message2 = MessagePayload {
            timestamp: RtmpTimestamp::new(82),
            type_id: 5,
            data: vec![1, 2, 3],
            stream_id: 1
        };
        
        let mut message3 = MessagePayload {
            timestamp: RtmpTimestamp::new(92),
            type_id: 5,
            data: vec![1, 2, 3],
            stream_id: 1
        };

        let mut serializer = Serializer::new();
        serializer.serialize(&message1, false).unwrap();
        serializer.serialize(&message2, false).unwrap();

        let result = serializer.serialize(&message3, false).unwrap();

        let expected_csid = get_csid_for_message_type(message3.type_id) | 0b11000000; 
        let mut expected = vec![expected_csid as u8];
        expected.append(&mut message3.data);

        assert_eq!(result, expected);
    }

    #[test]
    fn can_force_uncompressed_serialization() {
        let message1 = MessagePayload {
            timestamp: RtmpTimestamp::new(72),
            type_id: 5,
            data: vec![1, 2, 3],
            stream_id: 1
        };

        let mut message2 = MessagePayload {
            timestamp: RtmpTimestamp::new(82),
            type_id: 5,
            data: vec![1, 2, 3],
            stream_id: 1
        };

        let mut serializer = Serializer::new();
        serializer.serialize(&message1, false).unwrap();

        let result = serializer.serialize(&message2, true).unwrap();

        let expected_csid = get_csid_for_message_type(message2.type_id); 
        let mut expected = vec![
            expected_csid as u8, 0, 0, message2.timestamp.value as u8, 0, 0, 
            message2.data.len() as u8, message2.type_id, message2.stream_id as u8, 0, 0, 0
        ];

        expected.append(&mut message2.data);
        assert_eq!(result, expected);
    }

    #[test]
    fn messages_larger_than_max_chunk_size_are_split() {
        let message = MessagePayload {
            timestamp: RtmpTimestamp::new(72),
            type_id: 5,
            data: vec![1, 2, 3],
            stream_id: 1
        };

        let mut serializer = Serializer::new();
        serializer.set_max_chunk_size(2);
        
        let result = serializer.serialize(&message, false).unwrap();

        let expected_csid1 = get_csid_for_message_type(message.type_id);
        let expected_csid2 = get_csid_for_message_type(message.type_id) | 0b11000000; 
        let expected = vec![
            expected_csid1 as u8, 0, 0, message.timestamp.value as u8, 0, 0, 
            message.data.len() as u8, message.type_id, message.stream_id as u8, 0, 0, 0, 1, 2,
            expected_csid2 as u8, 3
        ];

        assert_eq!(result, expected);
    }
}