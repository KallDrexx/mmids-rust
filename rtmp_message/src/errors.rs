use std::io;
use amf0::{Amf0DeserializationError, Amf0SerializationError};

quick_error! {
    #[derive(Debug)]
    pub enum MessageDeserializationError {
        InvalidMessageFormat {
            description("The message is not encoded as expected")
        }

        Amf0DeserializationError(err: Amf0DeserializationError) {
            cause(err)
            description(err.description())
            from()
        }
        
        Io(err: io::Error) {
            cause(err)
            description(err.description())
            from()
        }
    }
}

quick_error! {
    #[derive(Debug)]
    pub enum MessageSerializationError {
        InvalidChunkSize { 
            description("Cannot serialize a SetChunkSize message with a size of 2147483648 or greater")
        }

        Amf0SerializationError(err: Amf0SerializationError) {
            cause(err)
            description(err.description())
            from()
        }

        Io(err: io::Error) {
            cause(err)
            description(err.description())
            from()
        }
    }
}