use anyhow::Result;
use poem::middleware::{AddDataEndpoint, Tracing, TracingEndpoint};
use poem::{EndpointExt, Route};
use poem_openapi::OpenApiService;
use sqlx::{MySql, Pool};

use crate::web::api::Apis;
use crate::GLOBAL_CONFIG;

pub(crate) type DbPool = sqlx::MySqlPool;

pub(crate) async fn routes() -> Result<AddDataEndpoint<TracingEndpoint<Route>, Pool<MySql>>> {
    let cfg = GLOBAL_CONFIG.load();
    let pool = DbPool::connect(&*cfg.mysql.url).await?;
    let address = format!("http://{}/api", &*cfg.web.address);
    drop(cfg);

    let hero_service = OpenApiService::new(Apis, "Hero", "1.0.0").server(address);
    let swagger_ui = hero_service.swagger_ui();
    let spec = hero_service.spec();

    let route = Route::new()
        .nest("/api", hero_service)
        .nest("/swagger", swagger_ui)
        .at("/spec", poem::endpoint::make_sync(move |_| spec.clone()))
        .with(Tracing)
        .data(pool);

    Ok(route)
}
