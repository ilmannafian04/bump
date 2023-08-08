use std::process::exit;

use log::error;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AppConfig {
    #[serde(default = "default_host", alias = "HOST")]
    pub host: String,
    #[serde(default = "default_port", alias = "PORT")]
    pub port: u16,
}

macro_rules! config_default {
    ($name:ident, $return_type:ty, $return_value:expr) => {
        paste::item! {
            fn [< default_ $name >] () -> $return_type {
                $return_value
            }
        }
    };
}

config_default!(host, String, "127.0.0.1".to_owned());
config_default!(port, u16, 8080);

impl AppConfig {
    pub fn new() -> Self {
        let from_env = Self::from_env();

        from_env
    }

    pub fn from_env() -> Self {
        match envy::from_env::<AppConfig>() {
            Ok(config) => config,
            Err(err) => {
                error!("failed to build config from env: {}", err);
                exit(1)
            }
        }
    }
}
