use std::io::Cursor;
use byteorder::{BigEndian, WriteBytesExt, ReadBytesExt};

use errors::{MessageDeserializationError, MessageSerializationError};
use rtmp_message::{RtmpMessage, RawRtmpMessage};

pub fn serialize(stream_id: u32) -> Result<RawRtmpMessage, MessageSerializationError> {
    let mut cursor = Cursor::new(Vec::new());
    try!(cursor.write_u32::<BigEndian>(stream_id));

    Ok(RawRtmpMessage{ 
        data: cursor.into_inner(),
        type_id: 2
    })
}

pub fn deserialize(data: Vec<u8>) -> Result<RtmpMessage, MessageDeserializationError> {
    let mut cursor = Cursor::new(data);
    let stream_id = try!(cursor.read_u32::<BigEndian>());

    Ok(RtmpMessage::Abort {
        stream_id: stream_id
    })
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use byteorder::{BigEndian, WriteBytesExt};    

    use rtmp_message::RtmpMessage;

    #[test]
    fn can_serialize_message() {
        let id = 523;
        let mut cursor = Cursor::new(Vec::new());
        cursor.write_u32::<BigEndian>(id).unwrap();
        let expected = cursor.into_inner();

        let message = RtmpMessage::Abort { stream_id: id };
        let raw_message = message.serialize().unwrap();

        assert_eq!(raw_message.data, expected);
        assert_eq!(raw_message.type_id, 2);
    }

    #[test]
    fn can_deserialize_message() {
        let id = 532;
        let expected = RtmpMessage::Abort{stream_id: id};

        let mut cursor = Cursor::new(Vec::new());
        cursor.write_u32::<BigEndian>(id).unwrap();

        let result = RtmpMessage::deserialize(cursor.into_inner(), 2).unwrap();                
        assert_eq!(result, expected);
    }
}