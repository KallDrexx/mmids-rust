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
        Io(err: io::Error) {
            cause(err)
            description(err.description())
            from()
        }
    }
}