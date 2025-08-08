use serde::Deserialize;

//content_type, width, height not used for now, reconsider later
#[derive(Debug, Deserialize)]
pub struct MediaContent {
    #[serde(rename = "@url")]
    pub url: String,

    #[allow(dead_code)]
    #[serde(rename = "@type")]
    pub content_type: Option<String>,

    #[allow(dead_code)]
    #[serde(rename = "@width")]
    pub width: Option<u32>,

    #[allow(dead_code)]
    #[serde(rename = "@height")]
    pub height: Option<u32>,
}
