use std::mem;
use rtmp_time::RtmpTimestamp;

use errors::{MessageDeserializationError, MessageSerializationError};
use MessagePayload;

pub trait RtmpMessage : Sized {
    fn deserialize(Vec<u8>) -> Result<Self, MessageDeserializationError>;
    fn serialize(self) -> Result<Vec<u8>, MessageSerializationError>;
    fn get_type_id() -> u8;
}

#[derive(Eq, PartialEq, Debug)]
pub struct RtmpMessageDetails<T: RtmpMessage> {
    pub rtmp_timestamp: RtmpTimestamp,
    pub stream_id: u32,
    pub message: T
}

impl<T: RtmpMessage> RtmpMessageDetails<T> {
    pub fn from_payload(mut payload: MessagePayload) -> Result<Self, MessageDeserializationError> {
        let data = mem::replace(&mut payload.data, vec![]);
        let message = try!(T::deserialize(data));

        Ok(RtmpMessageDetails {
            rtmp_timestamp: payload.timestamp,
            stream_id: payload.stream_id,
            message: message
        })
    }

    pub fn to_payload(self) -> Result<MessagePayload, MessageSerializationError> {
        let timestamp = self.rtmp_timestamp;
        let stream_id = self.stream_id;
        let data = try!(T::serialize(self.message));
        let type_id = T::get_type_id();

        Ok(MessagePayload {
            timestamp: timestamp,
            stream_id: stream_id,
            type_id: type_id,
            data: data
        })
    }
}

#[cfg(test)]
mod tests {
    use rtmp_time::RtmpTimestamp;

    use errors::{MessageDeserializationError, MessageSerializationError};
    use MessagePayload;
    use super::*;

    const TYPE_ID: u8 = 254;

    #[derive(Eq, PartialEq, Debug)]
    struct TestMessage {
        data: Vec<u8>
    }

    impl RtmpMessage for TestMessage {
        fn deserialize(data: Vec<u8>) -> Result<Self, MessageDeserializationError> {
            Ok(TestMessage { data: data })
        }

        fn serialize(self) -> Result<Vec<u8>, MessageSerializationError> {
            Ok(self.data)
        }

        fn get_type_id() -> u8 {
            TYPE_ID
        }
    }

    #[test]
    fn can_get_details_from_payload() {
        let stream_id = 12;

        let payload = MessagePayload {
            timestamp: RtmpTimestamp::new(5),
            stream_id: stream_id,
            type_id: TYPE_ID,
            data: vec![1,2,3,4,5]
        };

        let expected = RtmpMessageDetails {
            rtmp_timestamp: RtmpTimestamp::new(5),
            stream_id: stream_id,
            message: TestMessage { data: vec![1,2,3,4,5] }
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
            message: TestMessage { data: vec![1,2,3,4,5] }
        };

        let expected = MessagePayload {
            timestamp: RtmpTimestamp::new(5),
            stream_id: stream_id,
            type_id: TYPE_ID,
            data: vec![1,2,3,4,5]
        };

        let result = details.to_payload().unwrap();
        assert_eq!(result, expected);
    }
}