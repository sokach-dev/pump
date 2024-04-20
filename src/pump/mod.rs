pub mod api;

use crate::alter;
use crate::models::{self, PumpAssessor};
use anyhow::Result;
use api::new_pairs;
use api::pair_info;
use chrono::NaiveDateTime;
use derive_builder::Builder;
use serde::Serialize;
use sqlx::FromRow;
use tracing::{info, warn};

#[derive(Debug, Clone, Serialize, Builder, FromRow)]
pub struct Assess {
    pub id: i64,
    pub symbol: String,                // 符号
    pub coin_name: String,             // 币名
    pub chain: String,                 // 链
    pub contract_address: String,      // 合约地址
    pub contract_status: String,       // 合约状态
    pub mint_renounced: i64,           // 是否放弃铸造
    pub top_10_holder_rate: f64,       // 前十持有者比例
    pub renounced_freeze_account: i64, // 放弃冻结账户
    pub burn_ratio: String,            // 燃烧比例
    pub burn_status: String,           // 燃烧状态
    pub rug_ratio: f64,                // 地毯比例
    pub creator_address: String,       // 创建者地址
    pub creator_balance: f64,          // 创建者余额
    pub pool_creation_timestamp: i64,  // 池创建时间戳
    pub gmgn_link: String,             // GMGN链接
    pub pump_launch: String,           // 启动平台
    pub tip: String,                   // 提示
    pub created_at: NaiveDateTime,     // 创建时间
    pub updated_at: NaiveDateTime,     // 更新时间
    pub deleted: i32,                  // 是否删除
}

pub struct AlertMessage {
    date: String,
    symbol: String,
    coin_name: String,
    address: String,
    gmgn_link: String,
    contract_check_link: String,
}

impl ToString for AlertMessage {
    fn to_string(&self) -> String {
        format!(
            "* date: {}\n* symbol: {}\n* coin_name: {}\n* address: {}\n* gmgn_link: {}\n* contract_check_link: {}\n",
            self.date, self.symbol, self.coin_name, self.address, self.gmgn_link, self.contract_check_link
        )
    }
}

pub async fn pump_new_pairs() -> Result<()> {
    let config = crate::utils::get_global_config().await;
    // 1. query new pairs
    let new_pairs = new_pairs::query_new_pairs().await?;
    // 2. add new pairs
    if new_pairs.is_empty() {
        info!("no new pairs");
        return Ok(());
    }
    let mut alert_assesses = Vec::new();
    // add new pairs
    for pair in new_pairs {
        match pair_info::query_new_pair_info(&pair).await {
            Ok(assess) => {
                let manager = models::get_global_manager().await;
                manager.add_assess(assess.clone()).await?;
                info!("success add new pair: {:?}", pair);
                alert_assesses.push(assess);
            }
            Err(e) => {
                info!("fail add new pair: {}, error: {}", pair, e);
            }
        }
    }

    // 3. alert new pairs
    for a in alert_assesses {
        // 合约状态为Danger的不发送警告
        if a.contract_status == "Danger" {
            warn!(
                "contract status is Danger, no need to alert: {}",
                a.contract_address
            );
            continue;
        }
        // 不燃烧的不发送警告
        if a.burn_status != "burn" {
            warn!(
                "burn status is not burn, no need to alert: {}",
                a.contract_address
            );
            continue;
        }
        // Mint未放弃的不发送警告
        if a.mint_renounced != 1 {
            warn!(
                "mint not renounced, no need to alert: {}",
                a.contract_address
            );
            continue;
        }
        // 冻结账户未放弃的不发送警告
        if a.renounced_freeze_account != 1 {
            warn!(
                "renounced freeze account not renounced, no need to alert: {}",
                a.contract_address
            );
            continue;
        }
        // top 10 持有者比例大于0.5的不发送警告
        if a.top_10_holder_rate > 0.3 {
            warn!(
                "top 10 holder rate > 0.3, no need to alert: {}",
                a.contract_address
            );
            continue;
        }
        let alter_msg = AlertMessage {
            date: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            symbol: a.symbol.clone(),
            coin_name: a.coin_name.clone(),
            address: a.contract_address.clone(),
            gmgn_link: a.gmgn_link.clone(),
            contract_check_link: format!(
                "{}/{}",
                "https://rugcheck.xyz/tokens", a.contract_address
            ),
        };
        match alter::ding::ding_markdown(
            alter_msg.to_string(),
            &config.alter.title,
            &config.alter.ding_url,
        )
        .await
        {
            Ok(_) => {
                info!("success send alter message: {}", alter_msg.to_string());
            }
            Err(e) => {
                info!("fail send alter message: {}", e);
            }
        }
    }
    Ok(())
}
