use serde::Deserialize;
use std::str::FromStr;
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct Config {
    #[validate(length(min = 1))]
    pub database_url: String,
    pub web: WebConfig,
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct WebConfig {
    #[validate(length(min = 1))]
    pub socket_addr: String,
    #[validate(range(min = 1))]
    pub timeout: u64,
}

impl FromStr for Config {
    type Err = toml::de::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        toml::from_str(s)
    }
}