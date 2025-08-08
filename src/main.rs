use dotenvy::dotenv;

mod configs;
mod data_layer;
mod models;

use crate::{
    configs::json::load_channels_json,
    data_layer::response::video_entries_wrapper,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let channels = load_channels_json("./data/channels_example.json")?;
    println!(
        "Name: {}\nId: {}",
        channels[0].channel_name, channels[0].channel_id
    );

    let all_entries =
        video_entries_wrapper(channels[0].channel_id.to_string()).await?;
    println!("{:?}", &all_entries[0..2]);

    Ok(())
}
