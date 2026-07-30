//! Gateway bootstrap for sdkwork-doudizhu.

use axum::Router;
use sdkwork_database_sqlx::DatabasePool;
use sdkwork_doudizhu_service_host::{DoudizhuServiceHost, SharedMatchService};
use sdkwork_web_bootstrap::{
    ApiAssemblyContribution, HttpRouteManifest, PgPoolReadinessCheck, ReadinessCheck,
};
use std::sync::Arc;

pub type ApiAssembly = ApiAssemblyContribution;

pub async fn assemble_api_router() -> Result<ApiAssembly, String> {
    assemble_api_router_with_host(DoudizhuServiceHost::from_env().await?).await
}

pub async fn assemble_api_router_with_pool(pool: DatabasePool) -> Result<ApiAssembly, String> {
    assemble_api_router_with_host(DoudizhuServiceHost::from_pool(pool).await?).await
}

async fn assemble_api_router_with_host(host: DoudizhuServiceHost) -> Result<ApiAssembly, String> {
    let readiness_pool = host
        .database_pool()
        .as_postgres()
        .ok_or_else(|| "doudizhu authoritative server requires a PostgreSQL pool".to_owned())?
        .clone();
    assemble_api_router_with_service(
        host.service(),
        Arc::new(PgPoolReadinessCheck::new(readiness_pool)),
    )
    .await
}

pub async fn assemble_api_router_with_service(
    service: SharedMatchService,
    readiness_check: Arc<dyn ReadinessCheck>,
) -> Result<ApiAssembly, String> {
    let router = Router::new()
        .merge(sdkwork_routes_doudizhu_app_api::gateway_mount(service.clone()).await)
        .merge(sdkwork_routes_doudizhu_backend_api::gateway_mount(service).await);
    let mut routes = Vec::new();
    routes.extend_from_slice(sdkwork_routes_doudizhu_app_api::gateway_route_manifest().routes());
    routes
        .extend_from_slice(sdkwork_routes_doudizhu_backend_api::gateway_route_manifest().routes());
    ApiAssemblyContribution::from_manifest(
        "sdkwork-doudizhu",
        "SDKWork Doudizhu API",
        router,
        HttpRouteManifest::from_owned_routes(routes),
        Vec::new(),
        readiness_check,
    )
}

pub async fn assemble_business_routes() -> Result<ApiAssembly, String> {
    assemble_api_router().await
}
