use dotenvy::dotenv;
use std::env;

mod configs;
mod data_layer;
mod logic_layer;
mod models;

use crate::configs::db::mongo_setup::{get_collection, load_mongo_config};
use crate::data_layer::db::mongo::write_wrapper;
use crate::logic_layer::abstractions::wrapper::query_wrapper;
use crate::models::{db::mongo_connection, video_entry::VideoEntry};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //setup configs
    dotenv().ok();
    let mongo_config: mongo_connection::MongoConfig = load_mongo_config();
    let coll: mongodb::Collection<mongodb::bson::Document> =
        get_collection(&mongo_config).await?;

    let status = env::var("STATUS").unwrap_or_else(|_| "prod".to_string());

    //overwrite default by passing env var (see setup/example.env for template)
    let json_path = env::var("CHANNELS_JSON")
        .unwrap_or_else(|_| "./data/channels_example.json".to_string());

    //query + filter
    //todo: read from json or txt file
    let filter_keep = [
        "FULL MATCH".to_string(),
        "Top Points".to_string(),
        "MS QF".to_string(),
        "MS SF".to_string(),
        "MS Final".to_string(),
    ];
    let filter_drop = ["WTT Feeder".to_string()];

    let all_filtered_entries: Vec<VideoEntry> =
        query_wrapper(&json_path, &filter_keep, &filter_drop).await?;

    //dev debug: visual confirmation
    if status == "dev" {
        println!("{:#?}", &all_filtered_entries[..5])
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

    Ok(())
}
