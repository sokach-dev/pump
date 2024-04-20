use serde::Deserialize;
use std::str::FromStr;
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct Config {
    #[validate(length(min = 1))]
    pub database_url: String,
    #[validate(length(min = 1))]
    pub rug_check_url: String, // https://api.rugcheck.xyz/v1/tokens/
    #[validate(length(min = 1))]
    pub gmgn_get_new_pairs_url: String, // https://gmgn.ai/defi/quotation/v1/pairs/sol/new_pairs?limit=10&orderby=open_timestamp&direction=desc&filters[]=not_honeypot
    #[validate(length(min = 1))]
    pub gmgn_get_pair_info_url: String, // https://gmgn.ai/defi/quotation/v1/tokens/sol/

    pub web: WebConfig,
    pub alter: AlterConfig,
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct WebConfig {
    #[validate(length(min = 1))]
    pub socket_addr: String,
    #[validate(range(min = 1))]
    pub timeout: u64,
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct AlterConfig {
    #[validate(length(min = 1))]
    pub ding_url: String,
    #[validate(length(min = 1))]
    pub title: String,
}

impl FromStr for Config {
    type Err = toml::de::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        toml::from_str(s)
    }
}
