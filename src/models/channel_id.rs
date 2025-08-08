use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ChannelID {
    pub channel_name: String,
    pub channel_id: String,
}
