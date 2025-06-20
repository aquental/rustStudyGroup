use anyhow::{Context, Result};
use serde::Deserialize;
use std::fs;

// Define structs to match the JSON structure
#[derive(Deserialize, Debug)]
struct ApiKey {
    name: String,
    bytes: Vec<u8>,
}

#[derive(Deserialize, Debug)]
struct Keys {
    keys: Vec<ApiKey>,
}

/// Load API keys from a JSON string.
///
/// This function takes a JSON string as input, expected to be in the format returned by the
/// `api-key` crate. It deserializes this string into a `Keys` struct, and then iterates over the
/// `keys` field of the struct and prints out the `name` and `bytes` fields of each element.
///
/// # Errors
///
/// This function returns an error if the JSON string cannot be deserialized into a `Keys`
/// struct.
pub fn load_api_key_from_str(json_str: &str) -> Result<()> {
    // Deserialize JSON string into Keys struct
    let keys_data: Keys = serde_json::from_str(json_str)?;

    // Iterate and print each key/value pair
    for key in keys_data.keys {
        println!("Key: {}", key.name);
        println!("Bytes: {:?}", key.bytes);
        println!();
    }

    Ok(())
}

/// Load API keys from a JSON file.
///
/// This function reads a JSON file from the specified path and then processes the contents
/// using the `load_api_key_from_str` function.
///
/// # Errors
///
/// This function returns an error if the file cannot be read or if the JSON content
/// cannot be deserialized.
pub fn load_api_key_from_file(file_path: &str) -> Result<()> {
    // Read JSON string from file
    let json_str = fs::read_to_string(file_path).context("Failed to read file")?;
    // Process the JSON string
    load_api_key_from_str(&json_str)
}

fn main() -> Result<()> {
    let file_path = "apikey.json";
    load_api_key_from_file(file_path)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_api_key() {
        let json = r#"
        {
            "keys": [
                {
                    "name": "google_api_key",
                    "bytes": [
                        74, 69, 248, 57, 214, 108, 36, 71, 167, 181, 223, 196, 113, 171,
                        78, 97, 176, 193, 170, 61, 132, 192, 152, 117, 221, 96, 162, 114,
                        129, 185, 156, 250, 232, 39, 204, 142, 215, 67, 195, 102, 206, 190,
                        208, 246, 113
                    ]
                },
                {
                    "name": "infura_api_key",
                    "bytes": [71, 114, 111, 107, 32, 51]
                }
            ]
        }
        "#;

        assert!(load_api_key_from_str(json).is_ok());
    }
}
