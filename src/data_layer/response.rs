use crate::data_layer::request;
use crate::models::{response::feed::Feed, video_entry::VideoEntry};

pub fn create_video_entries(feed: Feed) -> Vec<VideoEntry> {
    feed.entry
        .iter()
        .map(|entry| {
            let group = &entry.media_group;
            VideoEntry {
                video_id: entry.video_id.clone(),
                published: entry.published.clone(),
                updated: entry.updated.clone(),

                title: group.title.clone(),
                content_url: group.content.url.clone(),
                description: group.description.clone(),

                star_rating_count: group.community.star_rating.count,
                star_rating_average: group.community.star_rating.average,
                star_rating_min: group.community.star_rating.min,
                star_rating_max: group.community.star_rating.max,

                views: group.community.statistics.views,
            }
        })
        .collect()
}

pub async fn video_entries_wrapper(
    channel_id: String,
) -> Result<Vec<VideoEntry>, Box<dyn std::error::Error + Send + Sync + 'static>>
{
    let feed: Feed = request::request_xml(channel_id).await?;
    let video_entries: Vec<VideoEntry> = create_video_entries(feed);

    Ok(video_entries)
}
