#[derive(PartialEq, Debug)]
pub struct StreamMetadata {
    pub video_width: Option<u32>,
    pub video_height: Option<u32>,
    pub video_codec: Option<String>,
    pub video_frame_rate: Option<f64>,
    pub video_bitrate_kbps: Option<u32>,
    pub audio_codec: Option<String>,
    pub audio_bitrate_kbps: Option<u32>,
    pub audio_sample_rate: Option<u32>,
    pub audio_channels: Option<u32>,
    pub audo_is_stereo: Option<bool>,
    pub encoder: Option<String>
}