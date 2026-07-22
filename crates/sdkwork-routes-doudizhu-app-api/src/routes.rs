use axum::extract::{Path, Query, State};
use axum::response::Response;
use axum::routing::get;
use axum::Router;
use sdkwork_doudizhu_match_service::{GameMatchQuery, GameMatchRepository, GameMatchService};
use sdkwork_web_axum::{extractors::WebRequestContext, RequirePrincipal};
use std::sync::Arc;

use crate::error::{map_match_error, ok_page_envelope, ok_resource_envelope};
use crate::paths::{MATCH_DETAIL_PATH, MATCH_LIST_PATH};

pub type MatchStore<R> = Arc<GameMatchService<R>>;

#[derive(Debug, serde::Deserialize, Default)]
pub struct MatchListQuery {
    page: Option<u32>,
    page_size: Option<u32>,
    status: Option<String>,
}

pub fn build_match_app_router<R>(store: MatchStore<R>) -> Router
where
    R: GameMatchRepository + Send + Sync + 'static,
{
    Router::new()
        .route(MATCH_LIST_PATH, get(list_matches::<R>))
        .route(MATCH_DETAIL_PATH, get(get_match::<R>))
        .with_state(store)
}

async fn list_matches<R>(
    ctx: WebRequestContext,
    RequirePrincipal(principal): RequirePrincipal,
    State(store): State<MatchStore<R>>,
    Query(query): Query<MatchListQuery>,
) -> Response
where
    R: GameMatchRepository + Send + Sync,
{
    respond_list(&ctx, &store, principal.tenant_id(), query).await
}

async fn get_match<R>(
    ctx: WebRequestContext,
    RequirePrincipal(principal): RequirePrincipal,
    State(store): State<MatchStore<R>>,
    Path(match_id): Path<String>,
) -> Response
where
    R: GameMatchRepository + Send + Sync,
{
    let tenant_id = principal.tenant_id();
    match store.get_match(tenant_id, &match_id).await {
        Ok(item) => ok_resource_envelope(&ctx, item),
        Err(error) => map_match_error(error, &ctx),
    }
}

pub async fn respond_list<R>(
    ctx: &WebRequestContext,
    store: &GameMatchService<R>,
    tenant_id: &str,
    query: MatchListQuery,
) -> Response
where
    R: GameMatchRepository + Send + Sync,
{
    let match_query = GameMatchQuery {
        page: query.page,
        page_size: query.page_size,
        status: query.status,
    };

    match store.list_matches(tenant_id, match_query).await {
        Ok(page) => ok_page_envelope(ctx, page),
        Err(error) => map_match_error(error, ctx),
    }
}
