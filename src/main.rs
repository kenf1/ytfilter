mod configs;
mod data;
mod models;

use crate::data::request;
use crate::models::feed::Feed;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //todo: read channel id from env/another file
    let feed: Feed = request::request_xml("UC9ckyA_A3MfXUa0ttxMoIZw").await?;

    //todo: save as struct
    for entry in &feed.entry {
        println!("videoId: {}", entry.video_id);
        println!("published: {}", entry.published);
        println!("updated: {}", entry.updated);

        let group: &models::media::group::MediaGroup = &entry.media_group;
        println!("media:title: {}", group.title);
        println!("media:content url: {}", group.content.url);
        println!("media:description: {}", group.description);
        println!(
            "media:starRating count={}, average={}, min={}, max={}",
            group.community.star_rating.count,
            group.community.star_rating.average,
            group.community.star_rating.min,
            group.community.star_rating.max
        );
        println!(
            "media:statistics views={}",
            group.community.statistics.views
        );
        println!("---------------------------------------------------");
    }

    Ok(())
}
