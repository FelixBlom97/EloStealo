use anyhow::{anyhow, Error};
use moka::future::Cache;
use moka::policy::EvictionPolicy;
use tracing::log;
use domain::chessgame::ChessGame;
use persistence::elo_stealo_postgres::EloStealoPostgresStore;
use persistence::game_id::GameId;
use crate::game_room::GameRoom;

#[derive(Clone)]
pub struct GameStore {
    cache: Cache<GameId, GameRoom>,
    database: EloStealoPostgresStore
}

impl GameStore {
    pub fn new(capacity: u64, database:EloStealoPostgresStore) -> Self {
        let write_to_database = async |key, value, cause| {
            log::info!("Game {:?} evicted from GameCache, reason: {:?}", key, cause);
            database.update_or_create_game(GameId::from(key), value).await
                .unwrap_or_else(|e| log::error!("Failed to write game {:?} to database: {:?}", key, e));
        };

        let cache = Cache::builder()
            .max_capacity(capacity)
            .eviction_policy(EvictionPolicy::lru())
            .eviction_listener(write_to_database)
            .build();

        Self {
            cache,
            database
        }
    }

    pub async fn new_game(&self, game: ChessGame) -> GameId {
        let id = GameId::new();
        self.database.save_game(&id, &game).await
            .unwrap_or_else(|e| log::error!("Failed to save new game {:?} to database: {:?}", id, e));
        self.cache.insert(id.clone(), GameRoom::new(game)).await;
        id
    }

    pub async fn try_get(&self, room_id: GameId) -> anyhow::Result<GameRoom> {
        let game_room = self.cache.try_get_with::<_, Error>(
            room_id.clone(),
            async { // upon cache miss, retrieve game from the database
                let game = self.database.get_game(room_id).await?;
                Ok(GameRoom::new(game))
            },
        ).await.map_err(|arc_err| anyhow!(arc_err.to_string()))?;
        Ok(game_room)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::{PgPool, Executor};
    use persistence::elo_stealo_postgres::EloStealoPostgresStore;
    use domain::chessgame::ChessGame;

    #[sqlx::test(migrations = "../persistence/migrations")]
    async fn test_insert_and_get_game(pool: PgPool) {
        // Set up the database store and GameStore
        let store = EloStealoPostgresStore::new(pool.clone());
        let game_store = GameStore::new(100, store.clone());

        // Create a test game and id
        let game_id = GameId::new();
        let chess_game = ChessGame::default();

        // Insert into cache (which will store in memory)
        game_store.insert_into_cache(game_id.clone(), chess_game.clone()).await;

        // Try to get from cache (should hit cache)
        let room = game_store.try_get(game_id.clone(), &store).await.unwrap();
        assert_eq!(room.game, chess_game);

        // Optionally, test eviction and DB persistence
    }
}