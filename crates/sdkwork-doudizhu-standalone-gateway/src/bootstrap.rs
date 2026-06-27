use std::sync::Arc;

use axum::Router;
use sdkwork_doudizhu_database_host::bootstrap_doudizhu_database_from_env;
use sdkwork_game_match_repository_sqlx::{
    memory_match_repository, GameMatchRepositoryKind, InMemoryGameMatchRepository,
    SqlxGameMatchRepository,
};
use sdkwork_game_match_service::GameMatchService;
use sdkwork_routes_health_app_api::build_health_router;
use sdkwork_routes_match_app_api::{build_match_app_router, MatchStore};
use sdkwork_routes_match_backend_api::build_match_backend_router;
use tower_http::cors::CorsLayer;

use crate::{with_doudizhu_app_request_context, with_doudizhu_backend_request_context};

pub type SharedMatchService = MatchStore<GameMatchRepositoryKind>;

pub async fn build_match_service() -> Result<SharedMatchService, String> {
    let mode = std::env::var("DOUDIZHU_REPOSITORY_MODE").unwrap_or_else(|_| "sqlx".into());
    if mode == "memory" {
        return Ok(Arc::new(GameMatchService::new(memory_match_repository())));
    }

    let host = bootstrap_doudizhu_database_from_env().await?;
    let repository =
        GameMatchRepositoryKind::Sqlx(Box::new(SqlxGameMatchRepository::new(host.pool().clone())));
    Ok(Arc::new(GameMatchService::new(repository)))
}

pub fn build_memory_match_service() -> SharedMatchService {
    Arc::new(GameMatchService::new(GameMatchRepositoryKind::Memory(
        InMemoryGameMatchRepository::with_seed(vec![]),
    )))
}

pub fn build_router(store: SharedMatchService) -> Router {
    let app_routes = Router::new()
        .merge(with_doudizhu_app_request_context(build_health_router()))
        .merge(with_doudizhu_app_request_context(build_match_app_router(
            store.clone(),
        )));

    let backend_routes = with_doudizhu_backend_request_context(build_match_backend_router(store));

    Router::new()
        .merge(app_routes)
        .merge(backend_routes)
        .layer(CorsLayer::permissive())
}
