mod http_route_manifest;
mod paths;
mod routes;

pub use http_route_manifest::gateway_route_manifest;
pub use paths::BACKEND_MATCH_LIST_PATH;
pub use routes::build_match_backend_router;

pub async fn gateway_mount<R>(store: sdkwork_routes_doudizhu_app_api::MatchStore<R>) -> axum::Router
where
    R: sdkwork_doudizhu_match_service::GameMatchRepository + 'static,
{
    build_match_backend_router(store)
}
