use mongodb::{
    Collection,
    bson::{Document, to_document},
};

use crate::models::video_entry::VideoEntry;

pub async fn write_wrapper(
    coll: &Collection<Document>,
    entries: Vec<VideoEntry>,
) -> mongodb::error::Result<()> {
    let docs: Vec<Document> = video_entries_to_documents(entries);
    mongo_create(coll, docs).await
}

async fn mongo_create(
    coll: &Collection<Document>,
    docs: Vec<Document>,
) -> mongodb::error::Result<()> {
    coll.insert_many(docs).await?;
    Ok(())
}

pub fn video_entries_to_documents(entries: Vec<VideoEntry>) -> Vec<Document> {
    entries
        .into_iter()
        .filter_map(|entry| to_document(&entry).ok())
        .collect()
}
