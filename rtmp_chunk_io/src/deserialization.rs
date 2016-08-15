use std::io;
use std::io::{Cursor, Write};
use std::collections::HashMap;
use std::mem;
use super::{MessagePayload, ChunkHeaderFormat, ChunkHeader};
use super::read_u24_be;
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};

/// Allows deserializing bytes representing RTMP chunks into RTMP message payloads.
/// Note that it all operations against the Deserializer are mutable due to
/// the RTMP chunking protocol compressing headers based on previously
/// deserialized chunks.
///
/// Due to the nature of the RTMP chunking protocol, the same deserializer should
/// be used for all data that is received from the peer.
pub struct Deserializer {
    previous_headers: HashMap<u32, ChunkHeader>,
    //max_chunk_size: u32,
    buffer: Vec<u8>,
    current_header: ChunkHeader,
    current_header_format: ChunkHeaderFormat,
    current_stage: ParseStage,
}

const MAX_INITIAL_TIMESTAMP: u32 = 16777215;

enum ParsedValue<T> {
    NotEnoughBytes,
    Value {val: T, next_index: u32}
}

#[derive(PartialEq)]
enum ParseResult {
    NotEnoughBytes,
    Success
}

enum ParseStage {
    Csid,
    InitialTimestamp,
    MessageLength,
    MessageTypeId,
    MessageStreamId,
    MessagePayload,
    ExtendedTimestamp,
}

quick_error! {
    #[derive(Debug)]
    pub enum DeserializationError {
        NoPreviousChunkOnStream(csid: u32) {
            description("Received non type 0 chunk but no previous chunk has been received on that csid")
        }

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
            previous_headers: HashMap::new(),
            //max_chunk_size: 128,
            buffer: Vec::new(),
            current_header: ChunkHeader::new(),
            current_header_format: ChunkHeaderFormat::Full,
            current_stage: ParseStage::Csid
        }
    }

    pub fn process_bytes(&mut self, bytes: &Vec<u8>) -> Result<Vec<MessagePayload>, DeserializationError> {
        try!(self.buffer.write(bytes));
        if self.buffer.len() < 1 {
            return Ok(Vec::new());
        }

        let mut results = Vec::new();
        loop {
            let result = match self.current_stage {
                ParseStage::Csid => try!(self.form_header()),
                ParseStage::InitialTimestamp => try!(self.get_initial_timestamp()),
                ParseStage::MessageLength => try!(self.get_message_length()),
                ParseStage::MessageTypeId => try!(self.get_message_type_id()),
                ParseStage::MessageStreamId => try!(self.get_message_stream_id()),
                ParseStage::ExtendedTimestamp => try!(self.get_extended_timestamp()),
                ParseStage::MessagePayload => try!(self.get_message_data(&mut results)),
            };

            if result == ParseResult::NotEnoughBytes {
                break;
            }
        }

        Ok(results)        
    }

    fn form_header(&mut self) -> Result<ParseResult, DeserializationError> {
        if self.buffer.len() < 1 {
            return Ok(ParseResult::NotEnoughBytes);
        }

        self.current_header_format = get_format(&self.buffer[0]);
        let (csid, next_index) = match get_csid(&self.buffer) {
            ParsedValue::NotEnoughBytes => return Ok(ParseResult::NotEnoughBytes),
            ParsedValue::Value{val, next_index} => (val, next_index) 
        };

        self.current_header = match self.current_header_format {
            ChunkHeaderFormat::Full => {
                let mut new_header = ChunkHeader::new();
                new_header.chunk_stream_id = csid;
                new_header
            },

            _ => match self.previous_headers.remove(&csid) {
                None => return Err(DeserializationError::NoPreviousChunkOnStream(csid)),
                Some(header) => header
            }
        };

        self.buffer.drain(0..(next_index as usize));
        self.current_stage = ParseStage::InitialTimestamp;
        Ok(ParseResult::Success)
    }

    fn get_initial_timestamp(&mut self) -> Result<ParseResult, DeserializationError> {
        if self.current_header_format == ChunkHeaderFormat::Empty {
            self.current_stage = ParseStage::MessageLength;
            return Ok(ParseResult::Success);
        }

        if self.buffer.len() < 3 {
            return Ok(ParseResult::NotEnoughBytes);
        }

        let timestamp;
        {
            let mut cursor = Cursor::new(&mut self.buffer);
            timestamp = try!(read_u24_be(&mut cursor));
        }
        
        if self.current_header_format == ChunkHeaderFormat::Full {
            self.current_header.timestamp = timestamp;
        } else {
            // Non full headers are deltas only
            self.current_header.timestamp = self.current_header.timestamp + timestamp;
            self.current_header.timestamp_delta = timestamp;
        }

        self.buffer.drain(0..3);
        self.current_stage = ParseStage::MessageLength;
        Ok(ParseResult::Success)
    }

    fn get_message_length(&mut self) -> Result<ParseResult, DeserializationError> {
        if self.current_header_format == ChunkHeaderFormat::TimeDeltaOnly || self.current_header_format == ChunkHeaderFormat::Empty {
            self.current_stage = ParseStage::MessageTypeId;
            return Ok(ParseResult::Success);
        }

        if self.buffer.len() < 3 {
            return Ok(ParseResult::NotEnoughBytes);
        }

        let length;
        {
            let mut cursor = Cursor::new(&mut self.buffer);
            length = try!(read_u24_be(&mut cursor));
        }

        self.buffer.drain(0..3);
        self.current_header.message_length = length;
        self.current_stage = ParseStage::MessageTypeId;
        Ok(ParseResult::Success)
    }

    fn get_message_type_id(&mut self) -> Result<ParseResult, DeserializationError> {
        if self.current_header_format == ChunkHeaderFormat::TimeDeltaOnly || self.current_header_format == ChunkHeaderFormat::Empty {
            self.current_stage = ParseStage::MessageStreamId;
            return Ok(ParseResult::Success);
        }

        if self.buffer.len() < 1 {
            return Ok(ParseResult::NotEnoughBytes);
        }

        self.current_header.message_type_id = self.buffer[0];
        self.buffer.drain(0..1);
        self.current_stage = ParseStage::MessageStreamId;
        Ok(ParseResult::Success)
    }

    fn get_message_stream_id(&mut self) -> Result<ParseResult, DeserializationError> {
        if self.current_header_format != ChunkHeaderFormat::Full {
            self.current_stage = ParseStage::ExtendedTimestamp;
            return Ok(ParseResult::Success);
        }

        if self.buffer.len() < 4 {
            return Ok(ParseResult::NotEnoughBytes);
        }

        let stream_id;
        {
            let mut cursor = Cursor::new(&mut self.buffer);
            stream_id = try!(cursor.read_u32::<LittleEndian>());
        }

        self.buffer.drain(0..4);
        self.current_header.message_stream_id = stream_id;
        self.current_stage = ParseStage::ExtendedTimestamp;
        Ok(ParseResult::Success)
    }

    fn get_extended_timestamp(&mut self) -> Result<ParseResult, DeserializationError> {
        if self.current_header_format == ChunkHeaderFormat::Full && self.current_header.timestamp < MAX_INITIAL_TIMESTAMP {
            self.current_stage = ParseStage::MessagePayload;
            return Ok(ParseResult::Success);
        }
        else if self.current_header.timestamp_delta < MAX_INITIAL_TIMESTAMP {
            self.current_stage = ParseStage::MessagePayload;
            return Ok(ParseResult::Success);
        }

        if self.buffer.len() < 4 {
            return Ok(ParseResult::NotEnoughBytes);
        }

        let timestamp;
        {
            let mut cursor = Cursor::new(&mut self.buffer);
            timestamp = try!(cursor.read_u32::<BigEndian>());
        }

        if self.current_header_format == ChunkHeaderFormat::Full {
            self.current_header.timestamp = timestamp;
        } else {
            self.current_header.timestamp_delta = timestamp;

            // Since we already added the MAX_INITIAL_TIMESTAMP to the timestamp, only add the delta difference
            self.current_header.timestamp = self.current_header.timestamp + (MAX_INITIAL_TIMESTAMP - self.current_header.timestamp_delta);
        }

        self.buffer.drain(0..4);
        self.current_stage = ParseStage::MessagePayload;
        Ok(ParseResult::Success)        
    }

    fn get_message_data(&mut self, results: &mut Vec<MessagePayload>) -> Result<ParseResult, DeserializationError> {
        let length = self.current_header.message_length as usize;
        if self.buffer.len() < length {
            return Ok(ParseResult::NotEnoughBytes);
        }

        let mut data = Vec::new();
        for byte in self.buffer.drain(0..(length as usize)) {
            data.push(byte);
        }

        let payload = MessagePayload {
            timestamp: self.current_header.timestamp,
            type_id: self.current_header.message_type_id,
            stream_id: self.current_header.message_stream_id,
            data: data
        };

        results.push(payload);

        let current_header = mem::replace(&mut self.current_header, ChunkHeader::new());
        self.previous_headers.insert(current_header.chunk_stream_id, current_header);
        self.current_stage = ParseStage::Csid;
        Ok(ParseResult::Success)
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

    #[test]
    fn type_0_chunk_with_medium_stream_id() {
        let csid = 200u8;
        let timestamp = 25u32;
        let message_stream_id = 5u32;
        let type_id = 3;
        let payload = vec![1, 2, 3];
        let length = payload.len() as u8;

        let mut bytes = vec![0, csid, 0, 0, timestamp as u8, 0, 0, length, type_id, message_stream_id as u8, 0, 0, 0];
        bytes.write(&payload).unwrap();

        let mut deserializer = Deserializer::new();
        let result = deserializer.process_bytes(&bytes).unwrap();

        assert_eq!(1, result.len());
        assert_eq!(timestamp, result[0].timestamp);
        assert_eq!(type_id, result[0].type_id);
        assert_eq!(message_stream_id, result[0].stream_id);
        assert_eq!(payload, result[0].data);
    }

    #[test]
    fn type_0_chunk_with_large_stream_id() {
        let timestamp = 25u32;
        let message_stream_id = 5u32;
        let type_id = 3;
        let payload = vec![1, 2, 3];
        let length = payload.len() as u8;

        let mut bytes = vec![1, 234, 97, 0, 0, timestamp as u8, 0, 0, length, type_id, message_stream_id as u8, 0, 0, 0];
        bytes.write(&payload).unwrap();

        let mut deserializer = Deserializer::new();
        let result = deserializer.process_bytes(&bytes).unwrap();

        assert_eq!(1, result.len());
        assert_eq!(timestamp, result[0].timestamp);
        assert_eq!(type_id, result[0].type_id);
        assert_eq!(message_stream_id, result[0].stream_id);
        assert_eq!(payload, result[0].data);
    }

    #[test]
    fn can_read_full_type_1_chunk() {
        let csid = 50;
        let timestamp = 20;
        let delta = 10;
        let message_stream_id = 52;
        let type_id = 3;
        let payload1 = vec![1, 2, 3];
        let payload2 = vec![1, 2, 3];

        let chunk_0_bytes = get_type_0_chunk(csid, timestamp, message_stream_id, type_id, payload1);        

        let length = payload2.len() as u8;
        let mut chunk_1_bytes = vec![csid | 0b01000000, 0, 0, delta as u8, 0, 0, length, type_id];
        chunk_1_bytes.write(&payload2).unwrap();

        let mut deserializer = Deserializer::new();
        deserializer.process_bytes(&chunk_0_bytes).unwrap();

        let result = deserializer.process_bytes(&chunk_1_bytes).unwrap();

        assert_eq!(1, result.len());
        assert_eq!(timestamp + delta, result[0].timestamp);
        assert_eq!(type_id, result[0].type_id);
        assert_eq!(message_stream_id, result[0].stream_id);
        assert_eq!(payload2, result[0].data);
    }

    fn get_type_0_chunk(csid: u8, timestamp: u32, message_stream_id: u32, type_id: u8, payload: Vec<u8>) -> Vec<u8> {
        let mut bytes = vec![csid, 0, 0, timestamp as u8, 0, 0, payload.len() as u8, type_id, message_stream_id as u8, 0, 0, 0];
        bytes.write(&payload).unwrap();

        bytes
    }
}
