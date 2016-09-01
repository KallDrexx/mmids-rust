quick_error! {
    #[derive(Debug)]
    pub enum RtmpProcessorError {
        UnknownRequestId {
            description("Request id does not corrospond to an outstanding request")
        }
    }
}
