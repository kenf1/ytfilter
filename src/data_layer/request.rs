use quick_xml::de::from_str;

use crate::configs::channel_setup::create_xml_url;
use crate::models::response::feed::Feed;

pub async fn request_xml(
    channel_id: String,
) -> Result<Feed, Box<dyn std::error::Error>> {
    let url = create_xml_url(channel_id.to_string());

    let xml_data = reqwest::get(&url).await?.text().await?;
    let feed: Feed = from_str(&xml_data)?;

    Ok(feed)
}
