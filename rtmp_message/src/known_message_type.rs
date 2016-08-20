/// Represents message RTMP types that we can work with
#[derive(Eq, PartialEq, Debug)]
pub enum KnownMessageType {
    Abort,
    Acknowledgement,
    Amf0Command,
    Amf0Data,
    AudioData,
    SetChunkSize,
    SetPeerBandwidth,
    UserControl,
    VideoData,
    WindowAcknowledgement
}