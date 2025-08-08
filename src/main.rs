use dotenvy::dotenv;

mod configs;
mod data_layer;
mod filter;
mod models;

use crate::filter::title::filter_by_title;
use crate::models::{channel_id::ChannelID, video_entry::VideoEntry};
use crate::{
    configs::json::load_channels_json,
    data_layer::response::video_entries_wrapper,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let channels: Vec<ChannelID> =
        load_channels_json("./data/channels_example.json")?;
    println!(
        "Name: {}\nId: {}",
        channels[0].channel_name, channels[0].channel_id
    );

    let all_entries: Vec<VideoEntry> =
        video_entries_wrapper(channels[0].channel_id.to_string()).await?;

    let queries = [
        "FULL MATCH".to_string(),
        "Top Points".to_string(),
        "MS QF".to_string(),
        "MS SF".to_string(),
        "MS Final".to_string(),
    ];
    let filtered_entries: Vec<&VideoEntry> =
        filter_by_title(&all_entries, &queries);

    println!("{:?}", &filtered_entries);

    Ok(())
}
