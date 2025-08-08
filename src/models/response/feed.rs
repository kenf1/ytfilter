use serde::Deserialize;

use super::entry::Entry;

#[derive(Debug, Deserialize)]
pub struct Feed {
    #[serde(rename = "entry")]
    pub entry: Vec<Entry>,
}
