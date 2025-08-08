pub fn create_xml_url(channel_id: String) -> String {
    format!("https://www.youtube.com/feeds/videos.xml?channel_id={channel_id}")
}
