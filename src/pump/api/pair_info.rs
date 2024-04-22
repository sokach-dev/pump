use crate::pump::{Assess, AssessBuilder};
use anyhow::Result;
use serde::Deserialize;
use tracing::debug;

use super::check_contract;

#[derive(Debug, Deserialize)]
pub struct NewPairInfoResponse {
    pub code: i32,
    pub msg: String,
    pub data: NewPairInfo,
}

#[derive(Debug, Deserialize)]
pub struct NewPairInfo {
    pub token: NewToken,
}

#[derive(Debug, Deserialize)]
pub struct NewToken {
    pub symbol: String,
    pub name: String,
    pub chain: String,
    pub open_timestamp: i64,
    pub address: String,
    pub link: Link,
    pub is_honeypot: Option<bool>,
    pub renounced: Option<bool>,
    pub top_10_holder_rate: f64,
    pub renounced_mint: i64,
    pub renounced_freeze_account: i64,
    pub burn_ratio: String,
    pub burn_status: String,
    pub launchpad: Option<String>,
    pub rug_ratio: Option<f64>,
    pub holder_rugged_num: i64,
    pub holder_token_num: i64,
    pub creator_address: String,
    pub creator_balance: f64,
}

#[derive(Debug, Deserialize)]
pub struct Link {
    pub gmgn: String,
}

/*
https://gmgn.ai/defi/quotation/v1/tokens/sol/BjmBDhSCfJwNMp7uFgJ9GCjCCmsP1nWwaDeRdYzUhnsM
{
    code: 0,
    msg: "success",
    data: {
        token: {
            symbol: "stones",
            name: "Stoned Stones",
            decimals: 6,
            price: 0.0001006860863773635,
            chain: "sol",
            creation_timestamp: null,
            open_timestamp: 1713590789,
            address: "BjmBDhSCfJwNMp7uFgJ9GCjCCmsP1nWwaDeRdYzUhnsM",
            ...
            link: {
                geckoterminal: "https://www.geckoterminal.com/solana/tokens/BjmBDhSCfJwNMp7uFgJ9GCjCCmsP1nWwaDeRdYzUhnsM",
                gmgn: "https://gmgn.ai/sol/token/BjmBDhSCfJwNMp7uFgJ9GCjCCmsP1nWwaDeRdYzUhnsM"
                ...
            },
            is_honeypot: null,
            renounced: null,
            top_10_holder_rate: 0.283207,
            renounced_mint: 1,
            renounced_freeze_account: 1,
            burn_ratio: "0.9998",
            burn_status: "burn",
            pool_info: {
                ...
                initial_quote_reserve: "79.00536041500000000000",
                creation_timestamp: 1713590789,
            },
            launchpad: "Pump.fun",
            rug_ratio: 0.29009531703273933,
            ...
            holder_rugged_num: 700,
            holder_token_num: 2413,
            creator_address: "Hnp3aHSw4S3NQgSWZUHNaR7XPTDaWMWkZBVM7TWAbdBY",
            creator_balance: 0.1692212
        }
    }
}
*/

// 查询新交易对信息
pub async fn query_new_pair_info(address: &str) -> Result<Assess> {
    let config = crate::utils::get_global_config().await;
    let url = format!("{}/{}", &config.gmgn_get_pair_info_url, address);
    debug!("query new pair info url: {}", url);
    let resp = reqwest::get(&url)
        .await?
        .json::<NewPairInfoResponse>()
        .await?;
    if resp.code != 0 {
        return Err(anyhow::anyhow!("query new pair info failed: {}", resp.msg));
    }
    let token = resp.data.token;
    // check contract status
    let contrac_status = check_contract::query_contract_status("sol", address).await?;
    debug!("contract status: {}, address: {}", contrac_status, address);

    let assess = AssessBuilder::default()
        .symbol(token.symbol)
        .coin_name(token.name)
        .chain(token.chain)
        .contract_address(token.address)
        .contract_status(contrac_status)
        .mint_renounced(token.renounced_mint)
        .top_10_holder_rate(token.top_10_holder_rate)
        .renounced_freeze_account(token.renounced_freeze_account)
        .burn_ratio(token.burn_ratio)
        .burn_status(token.burn_status)
        .rug_ratio(token.rug_ratio.unwrap_or(-1.0)) // -1 表示没有跑路记录,也就不知道是否跑路
        .creator_address(token.creator_address)
        .creator_balance(token.creator_balance)
        .pool_creation_timestamp(token.open_timestamp)
        .gmgn_link(token.link.gmgn)
        .pump_launch(token.launchpad.unwrap_or("".to_string()))
        .tip("".to_string())
        .deleted(0)
        .build()?;

    Ok(assess)
}
