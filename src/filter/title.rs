use crate::models::video_entry::VideoEntry;

pub fn filter_by_title<'a>(
    entries: &'a [VideoEntry],
    queries: &[String],
) -> Vec<&'a VideoEntry> {
    entries
        .iter()
        .filter(|entry| queries.iter().any(|q| entry.title.contains(q)))
        .collect()
}
