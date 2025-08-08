use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MediaStarRating {
    #[serde(rename = "@count")]
    pub count: u32,

    #[serde(rename = "@average")]
    pub average: f32,

    #[serde(rename = "@min")]
    pub min: u8,

    #[serde(rename = "@max")]
    pub max: u8,
}
