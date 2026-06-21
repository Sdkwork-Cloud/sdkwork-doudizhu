use async_trait::async_trait;
use sdkwork_game_match_service::{
    GameMatchItem, GameMatchPage, GameMatchQuery, GameMatchRepository, GameResult,
};

use crate::memory::InMemoryGameMatchRepository;
use crate::sqlx::SqlxGameMatchRepository;

#[derive(Clone)]
pub enum GameMatchRepositoryKind {
    Memory(InMemoryGameMatchRepository),
    Sqlx(Box<SqlxGameMatchRepository>),
}

#[async_trait]
impl GameMatchRepository for GameMatchRepositoryKind {
    async fn list_matches(
        &self,
        tenant_id: &str,
        query: &GameMatchQuery,
    ) -> GameResult<GameMatchPage> {
        match self {
            Self::Memory(repo) => repo.list_matches(tenant_id, query).await,
            Self::Sqlx(repo) => repo.list_matches(tenant_id, query).await,
        }
    }

    async fn get_match_item(&self, tenant_id: &str, match_id: &str) -> GameResult<GameMatchItem> {
        match self {
            Self::Memory(repo) => repo.get_match_item(tenant_id, match_id).await,
            Self::Sqlx(repo) => repo.get_match_item(tenant_id, match_id).await,
        }
    }
}
