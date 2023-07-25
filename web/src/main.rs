use arc_swap::access::Access;
use time::format_description::well_known::Rfc3339;
use time::UtcOffset;
use tracing::Level;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::fmt::time::OffsetTime;

use crate::config::{Config, GLOBAL_CONFIG};

mod config;
pub(crate) mod error;
mod web;

fn main() {
    let _guard = init_log();
    let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
    rt.block_on(web::start())
}

fn init_log() -> WorkerGuard {
    //加载配置
    config::reload();
    let cfg = GLOBAL_CONFIG.map(|cfg: &Config| &cfg.log).load();

    let file_appender = tracing_appender::rolling::daily(&*cfg.directory, &*cfg.file_name);
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::fmt()
        .with_ansi(false)
        .with_thread_ids(true)
        .with_max_level(cfg.level.parse::<Level>().expect("日志级别配置错误"))
        .with_timer(OffsetTime::new(
            UtcOffset::from_hms(8, 0, 0).unwrap(),
            Rfc3339,
        ))
        .with_writer(non_blocking)
        .init();

    guard
}
