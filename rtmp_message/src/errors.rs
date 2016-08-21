use std::io;

quick_error! {
    #[derive(Debug)]
    pub enum MessageDeserializationError {
        InvalidMessageFormaat {
            description("The message is not encoded as expected")
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

        Io(err: io::Error) {
            cause(err)
            description(err.description())
            from()
        }
    }
}