use dotenvy::dotenv;

mod configs;
mod data_layer;
mod filter;
mod models;

use crate::configs::json::load_channels_json;
use crate::data_layer::response::video_entries_wrapper;
use crate::filter::title::filter_by_title;
use crate::models::{channel_id::ChannelID, video_entry::VideoEntry};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let queries = [
        "FULL MATCH".to_string(),
        "Top Points".to_string(),
        "MS QF".to_string(),
        "MS SF".to_string(),
        "MS Final".to_string(),
    ];
    let filtered_entries: Vec<VideoEntry> =
        query_wrapper("./data/channels_example.json", &queries).await?;

    Ok(())
}

async fn query_wrapper(
    json_path: &str,
    queries: &[String],
) -> Result<Vec<VideoEntry>, Box<dyn std::error::Error>> {
    let channels: Vec<ChannelID> = load_channels_json(json_path)?;

    let all_entries: Vec<VideoEntry> =
        video_entries_wrapper(channels[0].channel_id.to_string()).await?;

    let filtered_entries: Vec<VideoEntry> =
        filter_by_title(&all_entries, queries).into_iter().collect();

    Ok(filtered_entries)
}
