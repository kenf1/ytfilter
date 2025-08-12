use dotenvy::dotenv;
use std::env;

mod configs;
mod data_layer;
mod logic_layer;
mod models;

use crate::configs::db::mongo_setup::{
    get_client_and_collection, load_mongo_config,
};
use crate::configs::json::load_filters_json;
use crate::data_layer::db::mongo::write_wrapper;
use crate::logic_layer::abstractions::wrapper::query_wrapper;
use crate::models::{
    db::mongo_connection, filters::Filters, video_entry::VideoEntry,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let mongo_config: mongo_connection::MongoConfig = load_mongo_config();
    let (client, coll) = get_client_and_collection(&mongo_config).await?;

    let status = env::var("STATUS").unwrap_or_else(|_| "prod".to_string());

    //overwrite default by passing env var (see setup/example.env for template)
    let channels_json_path = env::var("CHANNELS_JSON")
        .unwrap_or_else(|_| "./data/channels_example.json".to_string());
    let filters_json_path = env::var("FILTERS_JSON")
        .unwrap_or_else(|_| "./data/filters_example.json".to_string());

    //query + filter
    let all_filters: Filters = load_filters_json(&filters_json_path)?;
    let all_filtered_entries: Vec<VideoEntry> = query_wrapper(
        &channels_json_path,
        &all_filters.keep,
        &all_filters.drop,
    )
    .await?;

    //skip db if no new entries
    if all_filtered_entries.is_empty() {
        return Ok(());
    }

    //dev debug: visual confirmation
    if status == "dev" && all_filtered_entries.len() >= 5 {
        println!("{:#?}", &all_filtered_entries[..5]);
    } else {
        println!("{:#?}", &all_filtered_entries[0]);
    }

    //write to collection
    let mongo_write_res = write_wrapper(&coll, all_filtered_entries).await;
    match mongo_write_res {
        Ok(res) => {
            if status == "dev" {
                println!("{res:?}")
            }
            println!("Success")
        }
        Err(e) => {
            if status == "dev" {
                eprintln!("{e}");
            }
            eprintln!("Error")
        }
    }

    //usually not required, see: https://docs.rs/mongodb/latest/mongodb/struct.Client.html#method.shutdown
    client.shutdown().await;

    Ok(())
}
