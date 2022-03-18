use std::time::Duration;

use arc_swap::access::Access;
use poem::listener::TcpListener;
use poem::Server;

use crate::config::Config;
use crate::GLOBAL_CONFIG;

mod api;
mod routes;

pub(crate) async fn start() {
    let routes = match routes::routes().await {
        Ok(routes) => routes,
        Err(e) => {
            tracing::error!("Failed to load routes: {}", e);
            return;
        }
    };

    let address = GLOBAL_CONFIG.map(|cfg: &Config| &cfg.web.address);
    let res = Server::new(TcpListener::bind(&*address.load()))
        .run_with_graceful_shutdown(routes, ctrl_c(), Some(Duration::from_secs(10)))
        .await;

    if let Err(e) = res {
        tracing::error!("服务异常: {}", e);
    }
}

async fn ctrl_c() {
    if let Err(e) = tokio::signal::ctrl_c().await {
        tracing::error!("退出信号异常: {}", e);
    }
}
