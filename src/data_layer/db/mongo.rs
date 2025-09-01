use futures::stream::StreamExt;
use mongodb::{
    Collection, Cursor,
    bson::{Document, doc},
};

use crate::models::video_entry::VideoEntry;

pub async fn write_wrapper(
    coll: &Collection<Document>,
    entries: Vec<VideoEntry>,
) -> mongodb::error::Result<()> {
    let docs = filter_duplicates(coll, entries).await?;
    if !docs.is_empty() {
        mongo_create(coll, docs).await?;
    }
    Ok(())
}

async fn mongo_create(
    coll: &Collection<Document>,
    docs: Vec<Document>,
) -> mongodb::error::Result<()> {
    coll.insert_many(docs).await?;
    Ok(())
}

fn ve_to_document(entry: &VideoEntry) -> Document {
    doc! {
        "video_id": &entry.video_id,
        "published": &entry.published,
        "updated": &entry.updated,
        "title": &entry.title,
        "content_url": &entry.content_url,
        "description": &entry.description,
        "star_rating_count": entry.star_rating_count,
        "star_rating_average": entry.star_rating_average,
        "star_rating_min": entry.star_rating_min as i32,
        "star_rating_max": entry.star_rating_max as i32,
        "views": entry.views,
    }
}

async fn filter_duplicates(
    coll: &Collection<Document>,
    entries: Vec<VideoEntry>,
) -> mongodb::error::Result<Vec<Document>> {
    //collect all video_ids, to upload
    let video_ids: Vec<_> = entries.iter().map(|e| &e.video_id).collect();
    let filter = doc! { "video_id": { "$in": video_ids.clone() } };

    //collect all video_ids, in db
    let mut cursor: Cursor<Document> = coll.find(filter).await?;
    let mut existing_ids: Vec<String> = Vec::new();

    while let Some(result_doc) = cursor.next().await {
        let doc = result_doc?;
        if let Ok(id) = doc.get_str("video_id") {
            existing_ids.push(id.to_string());
        }
    }

    //convert video_ids not in db to document
    let docs: Vec<Document> = entries
        .into_iter()
        .filter(|entry| !existing_ids.contains(&entry.video_id))
        .map(|entry| ve_to_document(&entry))
        .collect();

    Ok(docs)
}
