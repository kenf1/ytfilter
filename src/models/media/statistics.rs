use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MediaStatistics {
    #[serde(rename = "@views")]
    pub views: u32,
}
