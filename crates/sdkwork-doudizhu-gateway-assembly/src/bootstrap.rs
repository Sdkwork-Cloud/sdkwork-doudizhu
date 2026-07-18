//! Gateway bootstrap for sdkwork-doudizhu.

use axum::Router;
use sdkwork_doudizhu_service_host::{build_match_service, SharedMatchService};
use sdkwork_routes_match_app_api::build_match_app_router;
use sdkwork_routes_match_backend_api::build_match_backend_router;

use crate::web_bootstrap::{
    with_doudizhu_app_request_context, with_doudizhu_backend_request_context,
};

pub struct ApplicationAssembly {
    pub router: Router,
}

pub async fn assemble_application_business_router() -> Result<ApplicationAssembly, String> {
    let service = build_match_service().await?;
    Ok(assemble_application_business_router_with_service(service))
}

pub fn assemble_application_business_router_with_service(
    service: SharedMatchService,
) -> ApplicationAssembly {
    let app = with_doudizhu_app_request_context(build_match_app_router(service.clone()));
    let backend = with_doudizhu_backend_request_context(build_match_backend_router(service));
    ApplicationAssembly {
        router: Router::new().merge(app).merge(backend),
    }
}

pub async fn assemble_application_router() -> Result<ApplicationAssembly, String> {
    assemble_application_business_router().await
}
