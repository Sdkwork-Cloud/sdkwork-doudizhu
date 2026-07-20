use axum::Router;
use sdkwork_api_doudizhu_assembly::{
    assemble_business_router_with_service, with_doudizhu_app_request_context,
};
pub use sdkwork_api_doudizhu_assembly::{
    build_match_service, build_memory_match_service, SharedMatchService,
};
use sdkwork_routes_health_app_api::build_health_router;

pub fn build_router(store: SharedMatchService) -> Router {
    let business = assemble_business_router_with_service(store).router;
    build_router_from_business(business)
}

pub fn build_router_from_business(business: Router) -> Router {
    Router::new()
        .merge(with_doudizhu_app_request_context(build_health_router()))
        .merge(business)
        .layer(sdkwork_web_bootstrap::application_cors_layer_from_env(
            &["SDKWORK_DOUDIZHU_ENVIRONMENT"],
            &[
                "SDKWORK_DOUDIZHU_CORS_ALLOWED_ORIGINS",
                "SDKWORK_CORS_ALLOWED_ORIGINS",
            ],
        ))
}
