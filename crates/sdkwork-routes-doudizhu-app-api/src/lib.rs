mod error;
mod http_route_manifest;
mod paths;
mod routes;

pub use error::{map_match_error, ok_page_envelope, ok_resource_envelope};
pub use http_route_manifest::gateway_route_manifest;
pub use paths::{MATCH_DETAIL_PATH, MATCH_LIST_PATH};
pub use routes::{build_match_app_router, respond_list, MatchListQuery, MatchStore};

pub async fn gateway_mount<R>(store: MatchStore<R>) -> axum::Router
where
    R: sdkwork_doudizhu_match_service::GameMatchRepository + 'static,
{
    build_match_app_router(store)
}
