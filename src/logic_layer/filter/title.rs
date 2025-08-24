use crate::models::video_entry::VideoEntry;

pub fn filter_by_title<'a>(
    entries: &'a [VideoEntry],
    filter_keep: &[String],
    filter_drop: &[String],
) -> Vec<&'a VideoEntry> {
    entries
        .iter()
        .filter(|entry| {
            //to keep
            let keep_match = filter_keep.is_empty()
                || filter_keep.iter().any(|q| entry.title.contains(q));

            //to drop
            let drop_match =
                filter_drop.iter().any(|q| entry.title.contains(q));
            keep_match && !drop_match
        })
        .collect()
}
