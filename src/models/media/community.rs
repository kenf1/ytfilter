use serde::Deserialize;

use super::star_rating::MediaStarRating;
use super::statistics::MediaStatistics;

#[derive(Debug, Deserialize)]
pub struct MediaCommunity {
    #[serde(rename = "starRating", alias = "media:starRating")]
    pub star_rating: MediaStarRating,

    #[serde(rename = "statistics", alias = "media:statistics")]
    pub statistics: MediaStatistics,
}
