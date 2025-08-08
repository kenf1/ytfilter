use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MediaContent {
    #[serde(rename = "@url")]
    pub url: String,

    #[serde(rename = "@type")]
    pub content_type: Option<String>,

    #[serde(rename = "@width")]
    pub width: Option<u32>,

    #[serde(rename = "@height")]
    pub height: Option<u32>,
}
