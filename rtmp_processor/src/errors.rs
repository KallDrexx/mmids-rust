quick_error! {
    #[derive(Debug)]
    pub enum RtmpProcessorError {
        UnknownRequestId {
            description("Request id does not corrospond to an outstanding request")
        }

        AllRequestIdsInUse {
            description("All request ids values (u32) are currently marked as outstanding")
        }
    }
}
