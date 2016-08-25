use std::mem;
use rtmp_time::RtmpTimestamp;

use errors::{MessageDeserializationError, MessageSerializationError};
use MessagePayload;
use RtmpMessage;

#[derive(PartialEq, Debug)]
pub struct RtmpMessageDetails {
    pub rtmp_timestamp: RtmpTimestamp,
    pub stream_id: u32,
    pub message: RtmpMessage
}

impl RtmpMessageDetails {
    pub fn from_payload(mut payload: MessagePayload) -> Result<Self, MessageDeserializationError> {
        let data = mem::replace(&mut payload.data, vec![]);
        let message = try!(RtmpMessage::deserialize(data, payload.type_id));

        Ok(RtmpMessageDetails {
            rtmp_timestamp: payload.timestamp,
            stream_id: payload.stream_id,
            message: message
        })
    }

    pub fn to_payload(self) -> Result<MessagePayload, MessageSerializationError> {
        let timestamp = self.rtmp_timestamp;
        let stream_id = self.stream_id;
        let raw_message = try!(RtmpMessage::serialize(self.message));

        Ok(MessagePayload {
            timestamp: timestamp,
            stream_id: stream_id,
            type_id: raw_message.type_id,
            data: raw_message.data
        })
    }
}

#[cfg(test)]
mod tests {
    use rtmp_time::RtmpTimestamp;

    use super::*;
    use MessagePayload;
    use RtmpMessage;

    #[test]
    fn can_get_details_from_payload() {
        let stream_id = 12;

        let payload = MessagePayload {
            timestamp: RtmpTimestamp::new(5),
            stream_id: stream_id,
            type_id: 255,
            data: vec![1,2,3,4,5]
        };

        let expected = RtmpMessageDetails {
            rtmp_timestamp: RtmpTimestamp::new(5),
            stream_id: stream_id,
            message: RtmpMessage::Unknown { type_id: 255, data: vec![1,2,3,4,5] }
        };

        let result = RtmpMessageDetails::from_payload(payload).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn can_get_payload_from_details() {
        let stream_id = 12;

        let details = RtmpMessageDetails {
            rtmp_timestamp: RtmpTimestamp::new(5),
            stream_id: stream_id,
            message: RtmpMessage::Unknown { type_id: 255, data: vec![1,2,3,4,5] }
        };

        let expected = MessagePayload {
            timestamp: RtmpTimestamp::new(5),
            stream_id: stream_id,
            type_id: 255,
            data: vec![1,2,3,4,5]
        };

        let result = details.to_payload().unwrap();
        assert_eq!(result, expected);
    }
}