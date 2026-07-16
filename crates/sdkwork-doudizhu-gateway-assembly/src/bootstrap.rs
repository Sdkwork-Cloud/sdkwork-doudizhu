//! Gateway bootstrap for sdkwork-doudizhu.

use axum::Router;
use sdkwork_doudizhu_standalone_gateway::{
    build_match_service, with_doudizhu_app_request_context,
    with_doudizhu_backend_request_context,
};
use sdkwork_routes_match_app_api::build_match_app_router;
use sdkwork_routes_match_backend_api::build_match_backend_router;

pub struct ApplicationAssembly {
    pub router: Router,
}

pub async fn assemble_application_business_router() -> Result<ApplicationAssembly, String> {
    let service = build_match_service().await?;
    let app = with_doudizhu_app_request_context(build_match_app_router(service.clone()));
    let backend = with_doudizhu_backend_request_context(build_match_backend_router(service));
    Ok(ApplicationAssembly {
        router: Router::new().merge(app).merge(backend),
    })
}

pub async fn assemble_application_router() -> Result<ApplicationAssembly, String> {
    assemble_application_business_router().await
}
