-- Add up migration script here
CREATE SCHEMA IF NOT EXISTS pump;

CREATE TABLE IF NOT EXISTS pump.assess (
    id BIGSERIAL PRIMARY KEY,
    symbol VARCHAR(255) NOT NULL, -- 符号
    coin_name VARCHAR(255) NOT NULL, -- 名称
    chain VARCHAR(255) NOT NULL, -- 链
    contract_address VARCHAR(255) NOT NULL, -- 合约地址
    contract_status VARCHAR(255) NOT NULL, -- 合约状态
    mint_renounced INT NOT NULL DEFAULT 0, -- 是否放弃铸造
    top_10_holder_rate FLOAT NOT NULL DEFAULT 0, -- 前10持有者比例
    renounced_freeze_account INT NOT NULL DEFAULT 0, -- 放弃冻结账户,即黑名单
    burn_ratio VARCHAR(255) NOT NULL, -- 燃烧比例
    burn_status VARCHAR(255) NOT NULL, -- 燃烧状态
    rug_ratio FLOAT NOT NULL DEFAULT 0, -- 跑路比例
    creator_address VARCHAR(255) NOT NULL, -- 创建者地址
    creator_balance FLOAT NOT NULL DEFAULT 0, -- 创建者余额
    pool_creation_timestamp TIMESTAMP NOT NULL, -- 池子创建时间
    gmgn_link VARCHAR(255) NOT NULL, -- GMGN链接
    tip TEXT DEFAULT NULL, -- 额外信息
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    deleted INT NOT NULL DEFAULT 0 -- 是否删除 0:未删除 1:已删除
);

CREATE INDEX IF NOT EXISTS idx_created_at ON pump.assess(created_at);
