use std::fs;

use crate::models::{channel_id::ChannelID, filters::Filters};

pub fn load_channels_json(
    path: &str,
) -> Result<Vec<ChannelID>, Box<dyn std::error::Error + Send + Sync + 'static>>
{
    let data = fs::read_to_string(path)?;
    let parsed = serde_json::from_str(&data)?;

    Ok(parsed)
}

pub fn load_filters_json(
    path: &str,
) -> Result<Filters, Box<dyn std::error::Error + Send + Sync + 'static>> {
    let data = fs::read_to_string(path)?;
    let parsed = serde_json::from_str(&data)?;

    Ok(parsed)
}
