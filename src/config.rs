use std::fs::File;

use serde::Deserialize;

use crate::cli;

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub log_level: String,
    pub api: ApiConfig,
    pub worker: WorkerConfig,
}

#[derive(Clone, Debug)]
pub struct ApiConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Clone, Debug)]
pub struct WorkerConfig {
    pub interval: u32,
}

impl AppConfig {
    pub fn new(args: &cli::Args) -> Self {
        let mut app_config = AppConfig::default();

        app_config.merge(Self::from_yml(&args.config));
        app_config.merge(Self::from_env());

        app_config
    }

    fn from_yml(path: &str) -> RawAppConfig {
        let reader = match File::open(path) {
            Ok(reader) => reader,
            Err(e) => panic!("failed to open config file: {}", e),
        };
        match serde_yaml::from_reader::<File, RawAppConfig>(reader) {
            Ok(app_config) => app_config,
            Err(e) => panic!("failed to read config file: {}", e),
        }
    }

    fn from_env() -> RawAppConfig {
        let mut raw_app_config = match envy::from_env::<RawAppConfig>() {
            Ok(config) => config,
            Err(err) => panic!("failed to build config from env: {}", err),
        };

        let raw_api_config = match envy::from_env::<RawApiConfig>() {
            Ok(config) => config,
            Err(err) => panic!("failed to build config from env: {}", err),
        };

        let raw_worker_config = match envy::from_env::<RawWorkerConfig>() {
            Ok(config) => config,
            Err(err) => panic!("failed to build config from env: {}", err),
        };

        raw_app_config.api = raw_api_config;
        raw_app_config.worker = raw_worker_config;

        raw_app_config
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            log_level: "info".to_owned(),
            api: ApiConfig::default(),
            worker: WorkerConfig::default(),
        }
    }
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_owned(),
            port: 8080,
        }
    }
}

impl Default for WorkerConfig {
    fn default() -> Self {
        Self { interval: 60 }
    }
}

#[derive(Debug, Default, Deserialize)]
struct RawAppConfig {
    #[serde(default, alias = "log_level")]
    pub log_level: Option<String>,

    #[serde(default)]
    pub api: RawApiConfig,

    #[serde(default)]
    pub worker: RawWorkerConfig,
}

#[derive(Debug, Default, Deserialize)]
struct RawApiConfig {
    #[serde(default, alias = "api_host")]
    pub host: Option<String>,
    #[serde(default, alias = "api_port")]
    pub port: Option<u16>,
}

#[derive(Debug, Default, Deserialize)]
struct RawWorkerConfig {
    #[serde(default, alias = "worker_interval")]
    pub interval: Option<u32>,
}

trait Merge {
    type Right;
    fn merge(&mut self, right: Self::Right);
}

impl Merge for AppConfig {
    type Right = RawAppConfig;

    fn merge(&mut self, right: Self::Right) {
        if let Some(log_level) = right.log_level {
            self.log_level = log_level
        }
        self.api.merge(right.api);
        self.worker.merge(right.worker);
    }
}

impl Merge for ApiConfig {
    type Right = RawApiConfig;

    fn merge(&mut self, right: Self::Right) {
        if let Some(host) = right.host {
            self.host = host
        }
        if let Some(port) = right.port {
            self.port = port
        }
    }
}

impl Merge for WorkerConfig {
    type Right = RawWorkerConfig;

    fn merge(&mut self, right: Self::Right) {
        if let Some(interval) = right.interval {
            self.interval = interval
        }
    }
}
