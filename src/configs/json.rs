use std::fs;

use crate::models::channel_id::ChannelID;

pub fn load_channels_json(
    path: &str,
) -> Result<Vec<ChannelID>, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(path)?;
    let parsed = serde_json::from_str(&data)?;

    Ok(parsed)
}
