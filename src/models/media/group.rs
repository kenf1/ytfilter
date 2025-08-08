use serde::Deserialize;

use super::community::MediaCommunity;
use super::content::MediaContent;

#[derive(Debug, Deserialize)]
pub struct MediaGroup {
    #[serde(rename = "title", alias = "media:title")]
    pub title: String,

    #[serde(rename = "content", alias = "media:content")]
    pub content: MediaContent,

    #[serde(rename = "description", alias = "media:description")]
    pub description: String,

    #[serde(rename = "community", alias = "media:community")]
    pub community: MediaCommunity,
}
