use chrono::Utc;
use log::info;
use tokio_schedule::{every, Job};

use crate::config::AppConfig;

pub async fn run(app_config: &AppConfig) {
    let every_second = every(app_config.worker.interval)
        .seconds()
        .in_timezone(&Utc)
        .perform(|| async { info!("run scheduled {}", Utc::now()) });
    every_second.await;
}
