use dotenvy::dotenv;
use opentelemetry_appender_tracing::layer;
use std::env;
use tracing_subscriber::{EnvFilter, prelude::*};

mod configs;
mod data_layer;
mod logic_layer;
mod models;

use crate::configs::db::mongo_setup::{
    get_client_and_collection, load_mongo_config,
};
use crate::configs::{
    json::load_filters_json, observability::init_logger_provider,
};
use crate::data_layer::db::mongo::write_wrapper;
use crate::logic_layer::abstractions::wrapper::query_wrapper;
use crate::models::{
    db::mongo_connection, filters::Filters, video_entry::VideoEntry,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>
{
    dotenv().ok();
    let status = env::var("STATUS").unwrap_or_else(|_| "prod".to_string());

    //logs setup
    let dsn = std::env::var("OPENTELEMETRY_DSN")
        .expect("Error: OPENTELEMETRY_DSN not found");
    let provider = init_logger_provider(dsn)?;

    let filter_otel = EnvFilter::new("info")
        .add_directive("hyper=off".parse().unwrap())
        .add_directive("tonic=off".parse().unwrap())
        .add_directive("h2=off".parse().unwrap())
        .add_directive("reqwest=off".parse().unwrap());
    let otel_layer = layer::OpenTelemetryTracingBridge::new(&provider)
        .with_filter(filter_otel);

    let filter_fmt = EnvFilter::new("info")
        .add_directive("opentelemetry=debug".parse().unwrap());
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_thread_names(true)
        .with_filter(filter_fmt);

    tracing_subscriber::registry()
        .with(otel_layer)
        .with(fmt_layer)
        .init();

    //db setup
    let mongo_config: mongo_connection::MongoConfig = load_mongo_config();
    let (client, coll) = get_client_and_collection(&mongo_config).await?;

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
        tracing::debug!("No new entries");

        //must repeat for early-return
        provider.force_flush()?;
        provider.shutdown()?;

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
            tracing::debug!("Success");
        }
        Err(e) => {
            if status == "dev" {
                eprintln!("{e}");
            }
            tracing::error!("Error writing to mongodb")
        }
    }

    //usually not required, see: https://docs.rs/mongodb/latest/mongodb/struct.Client.html#method.shutdown
    client.shutdown().await;

    //ensure logs complete
    provider.force_flush()?;
    provider.shutdown()?;

    Ok(())
}
