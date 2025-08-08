use crate::models::video_entry::VideoEntry;

//todo: avoid clone
pub fn filter_by_title<'a>(
    entries: &'a [VideoEntry],
    queries: &[String],
) -> Vec<VideoEntry> {
    entries
        .iter()
        .filter(|entry| queries.iter().any(|q| entry.title.contains(q)))
        .cloned()
        .to_owned()
        .collect()
}
