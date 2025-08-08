use crate::configs::json::load_channels_json;
use crate::data_layer::response::video_entries_wrapper;
use crate::logic_layer::filter::title::filter_by_title;
use crate::models::{channel_id::ChannelID, video_entry::VideoEntry};

//abstracted away to cleanup main.rs
pub async fn query_wrapper(
    json_path: &str,
    queries: &[String],
) -> Result<Vec<VideoEntry>, Box<dyn std::error::Error>> {
    let channels: Vec<ChannelID> = load_channels_json(json_path)?;

    //todo: find channel by name
    let all_entries: Vec<VideoEntry> =
        video_entries_wrapper(channels[0].channel_id.to_string()).await?;

    let filtered_entries: Vec<VideoEntry> =
        filter_by_title(&all_entries, queries).into_iter().collect();

    Ok(filtered_entries)
}
