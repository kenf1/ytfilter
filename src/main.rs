mod configs;
mod data;
mod models;

use crate::data::{request, response};
use crate::models::{response::feed::Feed, video_entry::VideoEntry};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //todo: read channel id from env/another file
    let feed: Feed = request::request_xml("UC9ckyA_A3MfXUa0ttxMoIZw").await?;

    let video_entries: Vec<VideoEntry> = response::create_video_entries(feed);
    println!("{:?}", &video_entries[0..2]);

    Ok(())
}
