use std::{fs::File, process::exit};

use log::error;
use serde::Deserialize;

use crate::cli;

#[derive(Clone, Debug, Deserialize)]
pub struct AppConfig {
    #[serde(default = "default_log_level", alias = "LOG_LEVEL")]
    pub log_level: String,

    #[serde(default = "default_api_host", alias = "API_HOST")]
    pub api_host: String,
    #[serde(default = "default_api_port", alias = "API_PORT")]
    pub api_port: u16,

    #[serde(default = "default_worker_interval", alias = "WORKER_INTERVAL")]
    pub worker_interval: u32,
}

impl AppConfig {
    pub fn new(args: &cli::Args) -> Self {
        let mut app_config = AppConfig::default();

        app_config.merge(Self::from_yml(&args.config));
        app_config.merge(Self::from_env());

        app_config
    }

    pub fn merge(&mut self, right: AppConfig) {
        macro_rules! merge_field {
            ($name:ident) => {
                paste::item! {
                    if right.$name != [< default_ $name >]() {
                        self.$name = right.$name
                    }
                }
            };
        }

        merge_field!(log_level);

        merge_field!(api_host);
        merge_field!(api_port);

        merge_field!(worker_interval);
    }

    fn from_yml(path: &str) -> Self {
        let reader = match File::open(path) {
            Ok(reader) => reader,
            Err(e) => {
                error!("failed to open config file: {}", e);
                exit(1)
            }
        };
        match serde_yaml::from_reader::<File, AppConfig>(reader) {
            Ok(app_config) => app_config,
            Err(e) => {
                error!("failed to read config file: {}", e);
                exit(1)
            }
        }
    }

    fn from_env() -> Self {
        match envy::from_env::<AppConfig>() {
            Ok(config) => config,
            Err(err) => {
                error!("failed to build config from env: {}", err);
                exit(1)
            }
        }
    }
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
config_default!(log_level, String, "info".to_owned());
config_default!(api_host, String, "127.0.0.1".to_owned());
config_default!(api_port, u16, 8080);
config_default!(worker_interval, u32, 60);

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            log_level: default_log_level(),
            api_host: default_api_host(),
            api_port: default_api_port(),
            worker_interval: default_worker_interval(),
        }
    }
}
