use std::sync::Arc;
use std::{env, fs};

use arc_swap::ArcSwap;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

pub(crate) static GLOBAL_CONFIG: Lazy<ArcSwap<Config>> =
    Lazy::new(|| ArcSwap::from_pointee(Config::default()));

/// 加载配置
pub(crate) fn reload() {
    let file = env::var("APP_ENV")
        .map(|e| format!("config.{e}.toml"))
        .unwrap_or("config.prd.toml".into());

    if let Ok(s) = fs::read_to_string(&file) {
        if let Ok(c) = toml::from_str(&s) {
            GLOBAL_CONFIG.store(Arc::new(c));
            return;
        };
    }

    if let Ok(c) = toml::to_string(&Config::default()) {
        fs::write(file, c).ok();
    }
}

#[derive(Deserialize, Serialize, Default)]
pub(crate) struct Config {
    pub(crate) log: LogCfg,
    pub(crate) web: WebCfg,
    pub(crate) mysql: MysqlCfg,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct LogCfg {
    pub(crate) directory: String,
    pub(crate) file_name: String,
    pub(crate) level: String,
}

impl Default for LogCfg {
    fn default() -> Self {
        LogCfg {
            directory: "./logs/".to_owned(),
            file_name: "log".to_owned(),
            level: "INFO".to_owned(),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub(crate) struct WebCfg {
    pub(crate) address: String,
}

impl Default for WebCfg {
    fn default() -> Self {
        WebCfg {
            address: "127.0.0.1:8080".to_owned(),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub(crate) struct MysqlCfg {
    pub(crate) url: String,
}

impl Default for MysqlCfg {
    fn default() -> Self {
        MysqlCfg {
            url: "mysql://zoe:123456@localhost:3306/crud".to_owned(),
        }
    }
}