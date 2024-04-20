pub mod api;

use crate::models::{self, PumpAssessor};
use anyhow::Result;
use api::new_pairs;
use chrono::NaiveDateTime;
use derive_builder::Builder;
use serde::Serialize;
use sqlx::FromRow;
use tracing::info;

#[derive(Debug, Clone, Serialize, Builder, FromRow)]
pub struct Assess {
    pub id: i64,
    pub symbol: String,                         // 符号
    pub coin_name: String,                      // 币名
    pub chain: String,                          // 链
    pub contract_address: String,               // 合约地址
    pub contract_status: String,                // 合约状态
    pub mint_renounced: i64,                    // 是否放弃铸造
    pub top_10_holder_rate: f64,                // 前十持有者比例
    pub renounced_freeze_account: i64,          // 放弃冻结账户
    pub burn_ratio: String,                     // 燃烧比例
    pub burn_status: String,                    // 燃烧状态
    pub rug_ratio: f64,                         // 地毯比例
    pub creator_address: String,                // 创建者地址
    pub creator_balance: f64,                   // 创建者余额
    pub pool_creation_timestamp: NaiveDateTime, // 池创建时间戳
    pub gmgn_link: String,                      // GMGN链接
    pub pump_launch: String,                    // 启动平台
    pub tip: String,                            // 提示
    pub created_at: NaiveDateTime,              // 创建时间
    pub updated_at: NaiveDateTime,              // 更新时间
    pub deleted: i32,                           // 是否删除
}

pub async fn pump_new_pairs() -> Result<()> {
    // 1. query new pairs
    let new_pairs = new_pairs::query_new_pairs().await?;
    // 2. add new pairs
    if new_pairs.is_empty() {
        info!("no new pairs");
        return Ok(());
    }
    // add new pairs
    for pair in new_pairs {
        let assess = AssessBuilder::default()
            .symbol("GMGN".to_string())
            .coin_name("GMGN".to_string())
            .chain("SOL".to_string())
            .contract_address(pair)
            .contract_status("normal".to_string())
            .mint_renounced(1)
            .top_10_holder_rate(0.0)
            .renounced_freeze_account(1)
            .burn_ratio("0".to_string())
            .burn_status("normal".to_string())
            .rug_ratio(0.0)
            .creator_address("GMGN".to_string())
            .creator_balance(0.0)
            .pool_creation_timestamp(chrono::Utc::now().naive_utc())
            .gmgn_link("https://gmgn.ai".to_string())
            .pump_launch("GMGN".to_string())
            .tip("GMGN".to_string())
            .build()
            .unwrap();
        let manager = models::get_global_manager().await;
        manager.add_assess(assess).await?;
    }

    // 3. alert new pairs
    Ok(())
}
