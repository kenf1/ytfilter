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

    //query + filter
    let queries = [
        "FULL MATCH".to_string(),
        "Top Points".to_string(),
        "MS QF".to_string(),
        "MS SF".to_string(),
        "MS Final".to_string(),
    ];

    let all_filtered_entries: Vec<VideoEntry> =
        query_wrapper(&json_path, &queries).await?;

    Ok(())
}
```

Environment variables (see data/example.env)

```shell
#required
MONGO_URI=
MONGO_DB=
MONGO_COLL=

#required, default: "./data/channels_example.json"
CHANNELS_JSON=

#optional, default: "prod"
STATUS=
```
