use axum::Router;
use sdkwork_iam_web_adapter::{build_web_framework_layer, IamWebRequestContextResolver};
use sdkwork_web_axum::with_web_request_context;
use sdkwork_web_core::HttpRouteManifest;

include!(concat!(env!("OUT_DIR"), "/doudizhu_http_routes.rs"));

pub fn doudizhu_public_path_prefixes() -> Vec<String> {
    vec![
        "/app/v3/api/system/health".to_owned(),
        "/app/v3/api/system/ready".to_owned(),
    ]
}

fn default_resolver() -> IamWebRequestContextResolver {
    IamWebRequestContextResolver::new(None)
}

fn wrap_router_with_manifest(router: Router, route_manifest: HttpRouteManifest) -> Router {
    with_web_request_context(
        router,
        build_web_framework_layer(
            default_resolver(),
            route_manifest,
            doudizhu_public_path_prefixes(),
        ),
    )
}

pub fn with_doudizhu_app_request_context(router: Router) -> Router {
    wrap_router_with_manifest(router, HttpRouteManifest::new(DOUDIZHU_APP_HTTP_ROUTES))
}

pub fn with_doudizhu_backend_request_context(router: Router) -> Router {
    wrap_router_with_manifest(router, HttpRouteManifest::new(DOUDIZHU_BACKEND_HTTP_ROUTES))
}
