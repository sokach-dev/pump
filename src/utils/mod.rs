pub mod log;
pub mod version;

use crate::config::Config;
use chrono::{FixedOffset, Local, NaiveDateTime};
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

pub fn get_local_time_second(timezone: i32) -> NaiveDateTime {
    let offset = FixedOffset::east_opt(timezone * 60 * 60).unwrap();
    let now_with_offset = Local::now().with_timezone(&offset);

    now_with_offset.naive_local()
}

#[cfg(test)]
mod tests {

    use super::*;
    use chrono::{FixedOffset, Local, Utc};

    // will test how to get local time
    #[test]
    fn test_get_utc_time() {
        // 获取utc时间
        let now = Utc::now();
        println!("now: {:?}", now);
        println!("now: {:?}", now.timestamp());
        println!("now: {:?}", now.date_naive());

        // 自动获取本地时间
        let now_fixed_offset = Local::now().fixed_offset();
        println!("now_fixed_offset: {:?}", now_fixed_offset);

        // 指定时区的时间
        let offset = FixedOffset::east_opt(8 * 60 * 60).unwrap();
        let now_with_offset = Local::now().with_timezone(&offset);
        println!("now_with_offset: {:?}", now_with_offset);

        // Converting the local NaiveDateTime to DateTime<Utc>
        let now_utc = now_with_offset.naive_local().and_utc();
        println!("now_utc: {:?}", now_utc);
    }

    #[test]
    fn test_get_local_time_second() {
        let local_time = get_local_time_second(8);
        println!("local_time: {:?}", local_time);
    }
}
