use std::fs;
use toml;
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Rule {
    pub name: String,
    pub target_directory: String,
    pub extensions: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub rules: Vec<Rule>,
}

pub fn parse_config(file_path: &str) -> Config {
    // Read the contents of the TOML file into a string
    let toml_str = fs::read_to_string(file_path).expect("Error reading the TOML file");

    // Parse the TOML string into a toml::Value
    toml::de::from_str(&toml_str).expect("Error converting to Config")
}
