use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Filters {
    pub keep: Vec<String>,
    pub drop: Vec<String>,
}
