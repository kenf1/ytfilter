use crate::configs::json::load_channels_json;
use crate::data_layer::response::video_entries_wrapper;
use crate::logic_layer::filter::title::filter_by_title;
use crate::models::{channel_id::ChannelID, video_entry::VideoEntry};

//abstracted away to cleanup main.rs
pub async fn query_wrapper(
    json_path: &str,
    filter_keep: &[String],
    filter_drop: &[String],
) -> Result<Vec<VideoEntry>, Box<dyn std::error::Error>> {
    let channels: Vec<ChannelID> = load_channels_json(json_path)?;

    let mut all_entries: Vec<VideoEntry> = Vec::new();

    for channel in &channels {
        let entries =
            video_entries_wrapper(channel.channel_id.to_string()).await?;

        let filtered = filter_by_title(&entries, filter_keep, filter_drop);
        all_entries.extend(filtered.into_iter().cloned());
    }

    Ok(all_entries)
}
