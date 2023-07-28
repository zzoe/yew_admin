use std::time::Duration;

use poem::listener::TcpListener;
use poem::middleware::{
    AddData, CatchPanic, Compression, CookieJarManager, NormalizePath, Tracing, TrailingSlash,
};
use poem::{EndpointExt, Route, Server};
use poem_openapi::OpenApiService;

use crate::web::api::crud::CRUDApi;
use crate::web::api::menu::MenuApi;
use crate::GLOBAL_CONFIG;

mod api;

pub(crate) type DbPool = sqlx::MySqlPool;

//noinspection HttpUrlsUsage
pub(crate) async fn start() {
    let cfg = GLOBAL_CONFIG.get().unwrap().load();
    let pool = match DbPool::connect(&cfg.mysql.url).await {
        Ok(p) => p,
        Err(e) => {
            tracing::error!("数据库连接失败：{e}");
            return;
        }
    };

    let hero_service = OpenApiService::new((MenuApi, CRUDApi), "Hero", "1.0.0")
        .server(format!("http://{}/api", cfg.web.address));
    let swagger_ui = hero_service.swagger_ui();
    let spec = hero_service.spec();

    let route = Route::new()
        .nest("/api", hero_service)
        .nest("/swagger", swagger_ui)
        .at("/spec", poem::endpoint::make_sync(move |_| spec.clone()))
        .with(AddData::new(pool))
        .with(NormalizePath::new(TrailingSlash::Trim))
        .with(CookieJarManager::new())
        .with(Compression::new())
        .with(Tracing)
        .with(CatchPanic::new());

    let res = Server::new(TcpListener::bind(&cfg.web.address))
        .run_with_graceful_shutdown(route, ctrl_c(), Some(Duration::from_secs(10)))
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
