
use super::{ModelsManager, PumpAssessor};

use crate::pump;
use anyhow::Result;

#[async_trait::async_trait]
impl PumpAssessor for ModelsManager {
    async fn add_assess(&self, assess: pump::Assess) -> Result<()> {
        let sql_str = format!(
            "INSERT INTO pump.assess (symbol, coin_name, chain, 
                contract_address, contract_status, mint_renounced,
                top_10_holder_rate, renounced_freeze_account, burn_ratio,
                burn_status, rug_ratio, creator_address, creator_balance,
                pool_creation_timestamp, gmgn_link, tip)
                VALUES ({}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {})",
                assess.symbol, assess.coin_name, assess.chain, assess.contract_address,
                assess.contract_status, assess.mint_renounced, assess.top_10_holder_rate,
                assess.renounced_freeze_account, assess.burn_ratio, assess.burn_status,
                assess.rug_ratio, assess.creator_address, assess.creator_balance,
                assess.pool_creation_timestamp, assess.gmgn_link, assess.tip,
        );

        sqlx::query(&sql_str)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn get_assess_by_contract_address(&self, address: &str) -> Result<pump::Assess> {
        let sql_str = format!(
            "SELECT * FROM pump.assess WHERE contract_address = {} AND deleted = 0",
            address 
        );
        let assess = sqlx::query_as(&sql_str)
            .fetch_one(&self.pool)
            .await?;

        Ok(assess)
    }

    async fn get_assess_by_limit(&self, limit: usize) -> Result<Vec<pump::Assess>> {
        let sql_str = format!(
            "SELECT * FROM pump.assess AND deleted = 0 ORDER BY id DESC LIMIT {}",
            limit
        );
        let assesses = sqlx::query_as(&sql_str)
            .fetch_all(&self.pool)
            .await?;

        Ok(assesses)
    }
}
