pub mod kind;
pub mod memory;
pub mod sqlx;

pub use kind::GameMatchRepositoryKind;
pub use memory::InMemoryGameMatchRepository;
pub use sqlx::SqlxGameMatchRepository;

pub fn memory_match_repository() -> GameMatchRepositoryKind {
    GameMatchRepositoryKind::Memory(InMemoryGameMatchRepository::with_seed(vec![]))
}
