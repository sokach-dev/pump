use crate::utils;
use anyhow::Result;
use serde::Deserialize;
use tap::prelude::*;
use tracing::debug;

#[derive(Debug, Deserialize)]
pub struct CheckContractResponse {
    pub score: i32,
}

/*  check contract status
https://api.rugcheck.xyz/v1/tokens/7gKCkfWmkqcV7Y2vXZTpWZmbh1ikeSZcpJQvSSbic4Ue/report
{
    ...
    score: 62105,
    ...
}
*/
pub async fn query_contract_status(chain: &str, address: &str) -> Result<String> {
    let config = utils::get_global_config().await;
    let rugcheck_url = &config.rug_check_url;
    if chain == "sol" {
        let url = format!("{}/{}/report", rugcheck_url, address);
        debug!("query contract status url: {}", url);
        let resp = reqwest::get(&url)
            .await?
            .json::<CheckContractResponse>()
            .await?
            .tap(|resp| debug!("query contract status response: {:?}, url: {}", resp, url));

        if resp.score < 500 {
            return Ok("Good".to_string());
        } else if resp.score < 1000 {
            return Ok("Warn".to_string());
        } else {
            return Ok("Danger".to_string());
        }
    } else if chain == "eth" {
        // todo
    }
    return Ok("unknown".to_string());
}
