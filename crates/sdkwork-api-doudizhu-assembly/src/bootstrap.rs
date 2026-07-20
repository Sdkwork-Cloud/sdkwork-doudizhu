//! Gateway bootstrap for sdkwork-doudizhu.

use axum::Router;
use sdkwork_doudizhu_service_host::{build_match_service, SharedMatchService};
use sdkwork_routes_match_app_api::build_match_app_router;
use sdkwork_routes_match_backend_api::build_match_backend_router;

use crate::web_bootstrap::{
    with_doudizhu_app_request_context, with_doudizhu_backend_request_context,
};

pub struct ApiAssembly {
    pub router: Router,
}

pub async fn assemble_business_routes() -> Result<ApiAssembly, String> {
    let service = build_match_service().await?;
    Ok(assemble_api_router_with_service(service))
}

pub fn assemble_api_router_with_service(
    service: SharedMatchService,
) -> ApiAssembly {
    let app = with_doudizhu_app_request_context(build_match_app_router(service.clone()));
    let backend = with_doudizhu_backend_request_context(build_match_backend_router(service));
    ApiAssembly {
        router: Router::new().merge(app).merge(backend),
    }
}

pub async fn assemble_business_routes() -> Result<ApiAssembly, String> {
    assemble_api_router().await
}
