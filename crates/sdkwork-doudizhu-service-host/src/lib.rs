use std::sync::Arc;

use sdkwork_doudizhu_database_host::bootstrap_doudizhu_database_from_env;
use sdkwork_game_match_repository_sqlx::{
    memory_match_repository, GameMatchRepositoryKind, InMemoryGameMatchRepository,
    SqlxGameMatchRepository,
};
use sdkwork_game_match_service::GameMatchService;

pub type SharedMatchService = Arc<GameMatchService<GameMatchRepositoryKind>>;

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
