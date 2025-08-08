use serde::Deserialize;
use tempfile::NamedTempFile;

#[derive(Debug, Deserialize, PartialEq)]
pub struct ChannelId {
    pub channel_name: String,
    pub channel_id: String,
}

pub fn load_channels_json(
    path: &str,
) -> Result<Vec<ChannelId>, Box<dyn std::error::Error>> {
    let data = std::fs::read_to_string(path)?;
    let parsed = serde_json::from_str(&data)?;
    Ok(parsed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_load_channels_json_success() {
        let json_data = r#"
        [
            {"channel_name": "ChannelA", "channel_id": "idA"},
            {"channel_name": "ChannelB", "channel_id": "idB"}
        ]
        "#;

        let mut tmpfile = NamedTempFile::new().unwrap();
        write!(tmpfile, "{}", json_data).unwrap();

        let path = tmpfile.path().to_str().unwrap();
        let channels = load_channels_json(path).unwrap();

        assert_eq!(
            channels,
            vec![
                ChannelId {
                    channel_name: "ChannelA".to_string(),
                    channel_id: "idA".to_string(),
                },
                ChannelId {
                    channel_name: "ChannelB".to_string(),
                    channel_id: "idB".to_string(),
                },
            ]
        );
    }

    #[test]
    fn test_load_channels_json_invalid() {
        let bad_json = r#" not valid json "#;

        let mut tmpfile = NamedTempFile::new().unwrap();
        write!(tmpfile, "{}", bad_json).unwrap();
        let path = tmpfile.path().to_str().unwrap();

        let result = load_channels_json(path);
        assert!(result.is_err(), "Should fail on malformed JSON");
    }

    #[test]
    fn test_load_channels_json_dne() {
        // Pass a non-existent file name
        let path = "/does/not/exist.json";
        let result = load_channels_json(path);
        assert!(result.is_err(), "Should fail when file does not exist");
    }
}
