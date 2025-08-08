use std::fs;

use crate::models::channel_id::ChannelId;

pub fn load_channels_json(
    path: &str,
) -> Result<Vec<ChannelId>, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(path)?;
    let parsed = serde_json::from_str(&data)?;

    Ok(parsed)
}
