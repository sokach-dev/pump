pub mod pump_manager;

use crate::{pump::Assess,utils::get_global_config};
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::sync::Arc;
use tokio::sync::OnceCell;
use anyhow::Result;


#[derive(Debug, Clone)]
pub struct ModelsManager {
    pool: PgPool,
}

impl ModelsManager {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

pub static GLOBAL_MANAGER: OnceCell<Arc<ModelsManager>> = OnceCell::const_new();

pub async fn get_global_manager() -> &'static Arc<ModelsManager> {
    GLOBAL_MANAGER
    .get_or_init(|| async {
        let config = get_global_config().await;
        let pool = PgPoolOptions::default()
            .max_connections(100)
            .connect(&config.database_url)
            .await
            .unwrap();
        Arc::new(ModelsManager::new(pool))
    }).await
}

#[async_trait::async_trait]
pub trait PumpAssessor {
    // add assess
    async fn add_assess(&self, assess: Assess) -> Result<()>;

    // get assess by token
    async fn get_assess_by_contract_address(&self, token: &str) -> Result<Assess>;
    
    // get assess by limit
    async fn get_assess_by_limit(&self, limit: usize) -> Result<Vec<Assess>>;
}