pub mod log;
pub mod version;

use crate::config::Config;
use std::{env, sync::Arc};
use tokio::{fs, sync::OnceCell};

pub static GLOBAL_CONFIG: OnceCell<Arc<Config>> = OnceCell::const_new();

pub async fn get_global_config() -> &'static Arc<Config> {
    let config_url = env::var("PUMP_CONFIG").expect("config url not found, check env PUMP_CONFIG");

    GLOBAL_CONFIG
        .get_or_init(|| async {
            Arc::new(
                fs::read_to_string(config_url)
                    .await
                    .unwrap()
                    .parse::<Config>()
                    .unwrap(),
            )
        })
        .await
}
