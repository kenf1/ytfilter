## ytfilter: youtube rss feeds xml parser and filter

[![GitHub Tag](https://img.shields.io/github/v/tag/kenf1/ytfilter)](https://github.com/kenf1/ytfilter/tags) [![Cargo UnitTest](https://github.com/kenf1/ytfilter/actions/workflows/rust-tests.yml/badge.svg)](https://github.com/kenf1/ytfilter/actions/workflows/rust-tests.yml) [![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)

Avoid distractions, annoyances, deceptive titles. ytfilter is a youtube rss feeds xml parser and filter.

Prototype in C# [HighlightsParser](https://github.com/kenf1/ytfilter/tree/main/HighlightsParser), rewritten in Rust [src](https://github.com/kenf1/ytfilter/tree/main/src).

### Example

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //import channel ids
    let channels: Vec<ChannelID> =
        load_channels_json("./data/channels_example.json")?;

    let all_entries: Vec<VideoEntry> =
        video_entries_wrapper(channels[0].channel_id.to_string()).await?;

    //query + filter
    let queries = ["FULL MATCH".to_string()];
    let filtered_entries: Vec<VideoEntry> =
        filter_by_title(&all_entries, &queries).to_owned();

    Ok(())
}
```
