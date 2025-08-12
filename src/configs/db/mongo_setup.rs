use mongodb::{Client, Collection, bson::Document, options::ClientOptions};
use std::env;

use crate::models::db::mongo_connection::MongoConfig;

pub fn load_mongo_config() -> MongoConfig {
    let uri = env::var("MONGO_URI").expect("MONGO_URI not set");
    let db_name = env::var("MONGO_DB").expect("MONGO_DB not set");
    let coll_name = env::var("MONGO_COLL").expect("MONGO_COLL not set");

    MongoConfig {
        uri,
        db_name,
        coll_name,
    }
}

pub async fn get_client_and_collection(
    config: &MongoConfig,
) -> mongodb::error::Result<(Client, Collection<Document>)> {
    let client_options = ClientOptions::parse(&config.uri).await?;
    let client = Client::with_options(client_options)?;

    let db = client.database(&config.db_name);
    let coll = db.collection::<Document>(&config.coll_name);

    Ok((client, coll))
}
