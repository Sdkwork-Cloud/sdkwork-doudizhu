use axum::Router;
use sdkwork_api_doudizhu_assembly::assemble_api_router_with_service;
pub use sdkwork_api_doudizhu_assembly::{
    build_match_service, build_memory_match_service, SharedMatchService,
};
use sdkwork_web_bootstrap::{service_router, ServiceRouterConfig};

pub fn build_router(store: SharedMatchService) -> Router {
    let business = assemble_api_router_with_service(store).router;
    build_router_from_business(business)
}

pub fn build_router_from_business(business: Router) -> Router {
    service_router(business, ServiceRouterConfig::default().with_always_ready()).layer(
        sdkwork_web_bootstrap::application_cors_layer_from_env(
            &["SDKWORK_DOUDIZHU_ENVIRONMENT"],
            &[
                "SDKWORK_DOUDIZHU_CORS_ALLOWED_ORIGINS",
                "SDKWORK_CORS_ALLOWED_ORIGINS",
            ],
        ),
    )
}
