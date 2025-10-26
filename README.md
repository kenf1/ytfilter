## ytfilter: youtube rss feeds xml parser and filter

[![GitHub Tag](https://img.shields.io/github/v/tag/kenf1/ytfilter)](https://github.com/kenf1/ytfilter/tags) [![Cargo UnitTest](https://github.com/kenf1/ytfilter/actions/workflows/rust-tests.yml/badge.svg)](https://github.com/kenf1/ytfilter/actions/workflows/rust-tests.yml) [![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)

Avoid distractions, annoyances, deceptive titles. ytfilter is a youtube rss feeds xml parser and filter.

Prototype in C# [HighlightsParser](https://github.com/kenf1/ytfilter/tree/main/HighlightsParser), rewritten in Rust [src](https://github.com/kenf1/ytfilter/tree/main/src).

### Install crate

```shell
cargo add --git https://github.com/kenf1/ytfilter
```

### Example

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //import channels + filters json
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

    Ok(())
}
```

Environment variables (see data/example.env)

```shell
#required
MONGO_URI=
MONGO_DB=
MONGO_COLL=

#required, default="./data/channels_example.json"
CHANNELS_JSON=

#required, default="./data/filters_example.json"
FILTERS_JSON=

#optional, default="prod"
STATUS=
```
