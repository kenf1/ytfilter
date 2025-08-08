use dotenvy::dotenv;

mod configs;
mod data_layer;
mod models;

use crate::configs::env::load_env_var;
use crate::data_layer::{request, response};
use crate::models::{response::feed::Feed, video_entry::VideoEntry};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    //todo: import channel id key value from json
    let channel_id = load_env_var("WTT");
    match channel_id {
        Ok(id) => {
            let feed: Feed = request::request_xml(id).await?;
            let video_entries: Vec<VideoEntry> =
                response::create_video_entries(feed);

            //todo: save data
            println!("{:?}", &video_entries[0..2]);
        }
        Err(e) => {
            eprintln!("Error: {e}")
        }
    }

    Ok(())
}
