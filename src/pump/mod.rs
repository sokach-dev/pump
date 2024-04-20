use chrono::NaiveDateTime;
use serde::{de, Serialize};
use anyhow::Result;
use derive_builder::Builder;
use sqlx::FromRow;



#[derive(Debug, Clone, Serialize, Builder, FromRow)]
pub struct Assess {
    pub id: i64,
    pub symbol: String, // 符号
    pub coin_name: String, // 币名
    pub chain: String, // 链
    pub contract_address: String, // 合约地址
    pub contract_status: String, // 合约状态
    pub mint_renounced: i64, // 是否放弃铸造
    pub top_10_holder_rate: f64, // 前十持有者比例
    pub renounced_freeze_account: i64, // 放弃冻结账户
    pub burn_ratio: String, // 燃烧比例
    pub burn_status: String, // 燃烧状态
    pub rug_ratio: f64, // 地毯比例
    pub creator_address: String, // 创建者地址
    pub creator_balance: f64, // 创建者余额
    pub pool_creation_timestamp: NaiveDateTime, // 池创建时间戳
    pub gmgn_link: String, // GMGN链接
    pub tip: String, // 提示
    pub created_at: NaiveDateTime, // 创建时间
    pub updated_at: NaiveDateTime, // 更新时间
    pub deleted: i32, // 是否删除
}