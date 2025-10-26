use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use anyhow::{anyhow, Error};
use moka::future::Cache;
use moka::policy::EvictionPolicy;
use tracing::log;
use domain::chessgame::ChessGame;
use persistence::elo_stealo_postgres::EloStealoPostgresStore;
use persistence::game_id::GameId;
use persistence::game_model::chess_game_to_model;
use persistence::stealo_rule::StealoRule;
use crate::game_room::GameRoom;

#[derive(Clone)]
pub struct GameStore {
    cache: Cache<GameId, GameRoom>,
    database: EloStealoPostgresStore
}

impl GameStore {
    pub fn new(capacity: u64, database:EloStealoPostgresStore) -> Self {
        let write_to_database = |key: Arc<GameId>, game_room: GameRoom, _cause| -> Pin<Box<dyn Future<Output = ()> + Send>> {
            let db = database.clone();
            Box::pin(async move {
                let id = (*key).clone();
                log::info!("Writing game {:?} to database upon eviction", key);
                let game = game_room.get_game();
                let guard = game.lock().await;
                let game_model = chess_game_to_model(&guard);
                drop(guard);

                if let Err(e) = db.update_game_from_model(id, game_model).await {
                    log::error!("Failed to write game {:?} to database: {:?}", key, e);
                }
            })
        };

        let cache = Cache::builder()
            .max_capacity(capacity)
            .eviction_policy(EvictionPolicy::lru())
            .async_eviction_listener(write_to_database)
            .build();

        Self {
            cache,
            database
        }
    }

    pub async fn new_game(&self, game: ChessGame) -> anyhow::Result<GameId> {
        let id = GameId::new();
        self.database.save_game(&id, &game).await?;
        self.cache.insert(id.clone(), GameRoom::new(game)).await;
        Ok(id)
    }

    pub async fn try_get_game(&self, room_id: GameId) -> anyhow::Result<GameRoom> {
        let game_room = self.cache.try_get_with::<_, Error>(
            room_id.clone(),
            async { // upon cache miss, retrieve game from the database
                let game = self.database.get_game(room_id).await?;
                Ok(GameRoom::new(game))
            },
        ).await.map_err(|arc_err| anyhow!(arc_err.to_string()))?;
        Ok(game_room)
    }

    pub async fn get_stealo_rules(&self) -> anyhow::Result<Vec<StealoRule>> {
        self.database.get_stealo_rules().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::{PgPool};
    use persistence::elo_stealo_postgres::EloStealoPostgresStore;
    use domain::chessgame::ChessGame;

    #[sqlx::test(migrations = "../persistence/migrations")]
    async fn new_game_stores_in_memory_and_db(pool: PgPool) {
        let database = EloStealoPostgresStore::new(pool.clone()).await.unwrap();
        let game_store = GameStore::new(100, database);

        let chess_game = ChessGame::new_game(None, None, None, None, 0, 0, 0, 0);

        let game_id = game_store.new_game(chess_game).await.unwrap();

        game_store.cache.run_pending_tasks().await;
        assert_eq!(1, game_store.cache.entry_count());
        assert!(game_store.database.get_game(game_id).await.is_ok());
    }
}