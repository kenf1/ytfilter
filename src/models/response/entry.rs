use serde::Deserialize;

use crate::models::media::group::MediaGroup;

#[derive(Debug, Deserialize)]
pub struct Entry {
    #[serde(rename = "videoId", alias = "yt:videoId")]
    pub video_id: String,

    pub published: String,
    pub updated: String,

    #[serde(rename = "group", alias = "media:group")]
    pub media_group: MediaGroup,
}
