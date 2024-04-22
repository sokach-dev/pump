use crate::models::{self, PumpAssessor};
use anyhow::Result;
use serde::Deserialize;
use tracing::{debug, error, info};

/*  query new pool
https://gmgn.ai/defi/quotation/v1/pairs/sol/new_pairs?limit=10&orderby=open_timestamp&direction=desc&filters[]=not_honeypot
{
    code: 0,
    msg: "success",
    data: {
        pairs: [
            {
                ...
                base_address: "6Ug3SqWN8pkg6RHKNE7yaszYjxSja3UaUr9tg2B4Xzn6",
                }
            ]
        }
    }
*/

#[derive(Debug, Deserialize)]
pub struct NewPairsResponse {
    pub code: i32,
    pub msg: String,
    pub data: NewPairsData,
}

#[derive(Debug, Deserialize)]
pub struct NewPairsData {
    pub pairs: Vec<NewPair>,
}

#[derive(Debug, Deserialize)]
pub struct NewPair {
    pub base_address: String,
}
pub async fn query_new_pairs() -> Result<Vec<String>> {
    let config = crate::utils::get_global_config().await;
    let resp = reqwest::get(&config.gmgn_get_new_pairs_url)
        .await?
        .json::<NewPairsResponse>()
        .await?;
    if resp.code != 0 {
        return Err(anyhow::anyhow!("query new pairs failed: {}", resp.msg));
    }
    // check new pairs, only need not record pair
    let pairs = resp
        .data
        .pairs
        .iter()
        .map(|p| p.base_address.clone())
        .collect::<Vec<String>>();

    let mut new_pairs = Vec::new();
    for pair in pairs {
        if !check_new_pairs(&pair).await {
            info!("new pair exist: {}", pair);
            new_pairs.push(pair);
        }
    }

    debug!("query new pairs: {:?}", new_pairs);
    Ok(new_pairs)
}

// check new pairs if exist
pub async fn check_new_pairs(address: &str) -> bool {
    let manager = models::get_global_manager().await;
    match manager.judge_assess_contract_address_exist(address).await {
        Ok(exist) => exist,
        Err(e) => {
            error!("check new pairs failed: {}", e);
            false
        }
    }
}
