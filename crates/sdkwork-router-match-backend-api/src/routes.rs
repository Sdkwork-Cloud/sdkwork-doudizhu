use axum::extract::{Query, State};
use axum::response::Response;
use axum::routing::get;
use axum::Router;
use sdkwork_game_match_service::GameMatchRepository;
use sdkwork_router_match_app_api::{respond_list, MatchListQuery, MatchStore};
use sdkwork_web_axum::RequirePrincipal;

use crate::paths::BACKEND_MATCH_LIST_PATH;

pub fn build_match_backend_router<R>(store: MatchStore<R>) -> Router
where
    R: GameMatchRepository + Send + Sync + 'static,
{
    Router::new()
        .route(BACKEND_MATCH_LIST_PATH, get(list_matches::<R>))
        .with_state(store)
}

async fn list_matches<R>(
    RequirePrincipal(principal): RequirePrincipal,
    State(store): State<MatchStore<R>>,
    Query(query): Query<MatchListQuery>,
) -> Response
where
    R: GameMatchRepository + Send + Sync,
{
    respond_list(&store, principal.tenant_id(), query).await
}
