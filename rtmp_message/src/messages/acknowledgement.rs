use std::io::Cursor;
use byteorder::{BigEndian, WriteBytesExt, ReadBytesExt};
use rtmp_time::RtmpTimestamp;

use errors::MessageDeserializationError;
use errors::MessageSerializationError;
use MessagePayload;

/// Sent when the peer has received bytes equal to the window size
#[derive(Eq, PartialEq, Debug)]
pub struct AcknowledgementMessage {
    pub sequence_number: u32
}

impl AcknowledgementMessage {
    pub fn from_payload(payload: &MessagePayload) -> Result<Self, MessageDeserializationError> {
        let mut cursor = Cursor::new(&payload.data);
        let sequence_number = try!(cursor.read_u32::<BigEndian>());

        Ok(AcknowledgementMessage{
            sequence_number: sequence_number
        })
    }

    pub fn to_payload(&self, timestamp: RtmpTimestamp, message_stream_id: u32) -> Result<MessagePayload, MessageSerializationError> {
        let mut cursor = Cursor::new(Vec::new());
        try!(cursor.write_u32::<BigEndian>(self.sequence_number));

        Ok(MessagePayload {
            timestamp: timestamp,
            stream_id: message_stream_id,
            type_id: 3,
            data: cursor.into_inner()
        })
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use byteorder::{BigEndian, WriteBytesExt};    
    use rtmp_time::RtmpTimestamp;

    use super::AcknowledgementMessage;
    use MessagePayload;

    #[test]
    fn can_serialize_message() {
        let id = 523;
        let timestamp = RtmpTimestamp::new(23);
        let stream_id = 55;
        let message = AcknowledgementMessage { sequence_number: id };
        let result = message.to_payload(timestamp, stream_id).unwrap();

        let mut cursor = Cursor::new(Vec::new());
        cursor.write_u32::<BigEndian>(id).unwrap();

        let expected = MessagePayload {
            timestamp: timestamp,
            stream_id: stream_id,
            type_id: 3,
            data: cursor.into_inner()
        };

        assert_eq!(expected, result);
    }

    #[test]
    fn can_deserialize_abort_payload() {
        let id = 532;
        let mut cursor = Cursor::new(Vec::new());
        cursor.write_u32::<BigEndian>(id).unwrap();
        
        let message = MessagePayload {
            timestamp: RtmpTimestamp::new(0),
            stream_id: 22,
            type_id: 3,
            data: cursor.into_inner()
        };

        let result = AcknowledgementMessage::from_payload(&message).unwrap();        
        let expected = AcknowledgementMessage{sequence_number: id};
        assert_eq!(expected, result);
    }
}