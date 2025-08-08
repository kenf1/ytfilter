use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ChannelID {
    #[allow(dead_code)]
    pub channel_name: String,
    pub channel_id: String,
}
