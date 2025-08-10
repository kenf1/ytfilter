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
    dotenv().ok();
    let mongo_config: mongo_connection::MongoConfig = load_mongo_config();
    let coll: mongodb::Collection<mongodb::bson::Document> =
        get_collection(&mongo_config).await?;

    let status = env::var("STATUS").unwrap_or_else(|_| "prod".to_string());

    //query + filter
    let queries = [
        "FULL MATCH".to_string(),
        "Top Points".to_string(),
        "MS QF".to_string(),
        "MS SF".to_string(),
        "MS Final".to_string(),
    ];
    let filtered_entries: Vec<VideoEntry> =
        query_wrapper("./data/channels_example.json", &queries).await?;

    //write to collection
    let mongo_write_res = write_wrapper(&coll, filtered_entries).await;
    match mongo_write_res {
        Ok(res) => {
            if status != "prod" {
                println!("{res:?}")
            }
            println!("Success")
        }
        Err(e) => {
            if status != "prod" {
                eprintln!("{e}");
            }
            eprintln!("Error")
        }
    }

    Ok(())
}
