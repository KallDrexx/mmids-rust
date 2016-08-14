use std::io;
use std::io::{Cursor, Write};
//use std::collections::HashMap;
use super::{MessagePayload, ChunkHeaderFormat, ChunkHeader};
use super::read_u24_be;
use byteorder::{LittleEndian, ReadBytesExt};

/// Allows deserializing bytes representing RTMP chunks into RTMP message payloads.
/// Note that it all operations against the Deserializer are mutable due to
/// the RTMP chunking protocol compressing headers based on previously
/// deserialized chunks.
///
/// Due to the nature of the RTMP chunking protocol, the same deserializer should
/// be used for all data that is received from the peer.
pub struct Deserializer {
    //previous_headers: HashMap<u32, ChunkHeader>,
    //max_chunk_size: u32,
    buffer: Vec<u8>
}

enum ParsedValue<T> {
    NotEnoughBytes,
    Value {val: T, next_index: u32}
}

quick_error! {
    #[derive(Debug)]
    pub enum DeerializationError {
        Io(err: io::Error) {
            cause(err)
            description(err.description())
            from()
        }
    }
}

impl Deserializer {
    pub fn new() -> Deserializer {
        Deserializer {
            //previous_headers: HashMap::new(),
            //max_chunk_size: 128,
            buffer: Vec::new()
        }
    }

    pub fn process_bytes(&mut self, bytes: &Vec<u8>) -> Result<Vec<MessagePayload>, DeerializationError> {
        try!(self.buffer.write(bytes));
        if self.buffer.len() < 1 {
            return Ok(Vec::new());
        }

        let mut results = Vec::new();

        let header_format = get_format(&self.buffer[0]);
        let (csid, next_index) = match get_csid(&self.buffer) {
            ParsedValue::NotEnoughBytes => return Ok(results),
            ParsedValue::Value{val, next_index} => (val, next_index)
        };

        let parsed_value = match header_format {
            ChunkHeaderFormat::Full => try!(get_full_header(&mut self.buffer, next_index, csid)),
            ChunkHeaderFormat::TimeDeltaWithoutMessageStreamId => unimplemented!(),
            ChunkHeaderFormat::TimeDeltaOnly => unimplemented!(),
            ChunkHeaderFormat::Empty => unimplemented!()
        };

        let (header, next_index) = match parsed_value {
            ParsedValue::NotEnoughBytes => return Ok(results),
            ParsedValue::Value {val, next_index} => (val, next_index)
        };

        let (data, _next_index) = match get_message_data(&self.buffer, next_index as usize, header.message_length as usize) {
            ParsedValue::NotEnoughBytes => return Ok(results),
            ParsedValue::Value{val, next_index} => (val, next_index) 
        };

        let message = MessagePayload {
            timestamp: header.timestamp,
            type_id: header.message_type_id,
            stream_id: header.message_stream_id,
            data: data
        };

        results.push(message);
        Ok(results)
    }
}

fn get_format(byte: &u8) -> ChunkHeaderFormat {
    const TYPE_0_MASK: u8 = 0b00000000;
    const TYPE_1_MASK: u8 = 0b01000000;
    const TYPE_2_MASK: u8 = 0b10000000;
    const FORMAT_MASK: u8 = 0b11000000;    

    let format_id = *byte & FORMAT_MASK;

    match format_id {
        TYPE_0_MASK => ChunkHeaderFormat::Full,
        TYPE_1_MASK => ChunkHeaderFormat::TimeDeltaWithoutMessageStreamId,
        TYPE_2_MASK => ChunkHeaderFormat::TimeDeltaOnly,
        _ => ChunkHeaderFormat::Empty
    }
} 

fn get_csid(buffer: &Vec<u8>) -> ParsedValue<u32> {
    const CSID_MASK: u8 = 0b00111111;

    if buffer.len() < 1 {
        return ParsedValue::NotEnoughBytes;
    }

    match buffer[0] & CSID_MASK {
        0 => {
            if buffer.len() < 2 { 
                ParsedValue::NotEnoughBytes 
            } else { 
                ParsedValue::Value{val: buffer[1] as u32 + 64, next_index: 2} 
            }            
        },

        1 => {
            if buffer.len() < 3 { 
                ParsedValue::NotEnoughBytes 
            } else { 
                ParsedValue::Value{val: (buffer[2] as u32 * 256) + buffer[1] as u32 + 64, next_index: 3} 
            }
        },

        x => ParsedValue::Value{val: x as u32, next_index: 1}
    }
}

fn get_full_header(buffer: &mut Vec<u8>, start_index: u32, csid: u32) -> Result<ParsedValue<ChunkHeader>, DeerializationError> {
    if buffer.len() as u32 - start_index < 11 {
        // Not enough bytes
        return Ok(ParsedValue::NotEnoughBytes);
    }

    let mut cursor = Cursor::new(buffer);
    cursor.set_position(start_index as u64);

    let timestamp = try!(read_u24_be(&mut cursor));
    let length = try!(read_u24_be(&mut cursor));
    let type_id = try!(cursor.read_u8());
    let stream_id = try!(cursor.read_u32::<LittleEndian>());
    
    let header = ChunkHeader {
        chunk_stream_id: csid,
        timestamp: timestamp,
        timestamp_delta: 0,
        message_length: length, 
        message_type_id: type_id,
        message_stream_id: stream_id
    };

    Ok(ParsedValue::Value {val: header, next_index: start_index + 11})
}

fn get_message_data(buffer: &Vec<u8>, start_index: usize, length: usize) -> ParsedValue<Vec<u8>> {
    if buffer.len() - start_index < length {
        return ParsedValue::NotEnoughBytes;
    }

    let mut result = Vec::new();
    for x in start_index..(start_index + length) {
        result.push(buffer[x]);
    }

    ParsedValue::Value {val: result, next_index: (start_index + length) as u32}
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn type_0_chunk_with_small_stream_id() {
        let csid = 50u8;
        let timestamp = 25u32;
        let message_stream_id = 5u32;
        let type_id = 3;
        let payload = vec![1, 2, 3];
        let length = payload.len() as u8;

        let mut bytes = vec![csid, 0, 0, timestamp as u8, 0, 0, length, type_id, message_stream_id as u8, 0, 0, 0];
        bytes.write(&payload).unwrap();

        let mut deserializer = Deserializer::new();
        let result = deserializer.process_bytes(&bytes).unwrap();

        assert_eq!(1, result.len());
        assert_eq!(timestamp, result[0].timestamp);
        assert_eq!(type_id, result[0].type_id);
        assert_eq!(message_stream_id, result[0].stream_id);
        assert_eq!(payload, result[0].data);
    }
}
