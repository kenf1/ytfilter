use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct VideoEntry {
    pub video_id: String,
    pub published: String,
    pub updated: String,

    pub title: String,
    pub content_url: String,
    pub description: String,

    pub star_rating_count: u32,
    pub star_rating_average: f32,
    pub star_rating_min: u8,
    pub star_rating_max: u8,

    pub views: u32,
}
