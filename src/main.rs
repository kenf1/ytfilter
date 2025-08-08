use quick_xml::de::from_str;

mod models;
use crate::models::feed::Feed;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //todo: read from env/another file
    let channel_id = "UC9ckyA_A3MfXUa0ttxMoIZw";

    //todo: separate
    let url = format!(
        "https://www.youtube.com/feeds/videos.xml?channel_id={channel_id}",
    );

    let xml_data = reqwest::get(&url).await?.text().await?;

    let feed: Feed = from_str(&xml_data)?;

    //todo: save as struct
    for entry in &feed.entry {
        println!("videoId: {}", entry.video_id);
        println!("published: {}", entry.published);
        println!("updated: {}", entry.updated);
        let group = &entry.media_group;
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
