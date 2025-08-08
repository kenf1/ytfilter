use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ChannelId {
    pub channel_name: String,
    pub channel_id: String,
}
