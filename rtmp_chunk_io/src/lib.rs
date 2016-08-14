//! This module handles the serialization and deserialization of 
//! RTMP chunks.

#[macro_use] extern crate quick_error;
extern crate byteorder;

pub mod serialization;  

use std::io::{Cursor, Result, Seek, SeekFrom};
use byteorder::{BigEndian, WriteBytesExt};

/// Represents bytes making up a complete RTMP message
pub struct MessagePayload {
    pub timestamp: u32,
    pub type_id: u8,
    pub stream_id: u32,
    pub data: Vec<u8>
}

#[derive(PartialEq)]
enum ChunkHeaderFormat {
    Full, // Format 0
    TimeDeltaWithoutMessageStreamId, // Format 1
    TimeDeltaOnly, // Format 2
    Empty // Format 3
}

struct ChunkHeader {
    chunk_stream_id: u32,
    timestamp: u32,
    message_length: u32, 
    message_type_id: u8,
    message_stream_id: u32
}

fn write_u24_be(cursor: &mut Cursor<Vec<u8>>, value: u32) -> Result<()> {
    debug_assert!(value <= 16777215, "Value is greater than what can fit in 3 bytes");

    try!(cursor.write_u32::<BigEndian>(value));
    
    {
        let mut inner = cursor.get_mut();
        let index_to_remove = inner.len() - 1 - 3;
        inner.remove(index_to_remove);
    }

    try!(cursor.seek(SeekFrom::End(0)));
    Ok(())
}

#[cfg(test)]
mod test {
    use super::write_u24_be;
    use std::io::Cursor;
    use byteorder::WriteBytesExt;

    #[test]
    fn can_write_u24() {
        let mut cursor = Cursor::new(Vec::new());
        write_u24_be(&mut cursor, 16777215).unwrap();

        // Make sure next writes are at the 4th byte
        cursor.write_u8(8).unwrap();
        
        assert_eq!(cursor.into_inner(), vec![255, 255, 255, 8]);
    }
}