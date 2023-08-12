use chrono::{Local, Utc};
use log::info;
use tokio_schedule::{every, Job};

pub async fn run() {
    let every_second = every(1)
        .seconds()
        .in_timezone(&Utc)
        .perform(|| async { info!("run scheduled {}", Local::now()) });
    every_second.await;
}
