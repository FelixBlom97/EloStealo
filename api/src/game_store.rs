use std::sync::Arc;
use anyhow::{anyhow, Error};
use moka::future::Cache;
use moka::notification::ListenerFuture;
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
        let db = Arc::new(database.clone());
        let write_to_database = move |key: Arc<GameId>, game_room: GameRoom, _cause| -> ListenerFuture {
            let db = db.clone();
            Box::pin(async move {
                let id = (*key).clone();
                log::info!("Writing game {:?} to database upon eviction", key);
                let game_model = {
                    let game_mutex = game_room.get_game();
                    let game_guard = game_mutex.lock().await;
                    chess_game_to_model(&game_guard)
                };
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
        let mut has_result = false;
        let game_room = self.cache.try_get_with::<_, Error>(
            room_id.clone(),
            async { // upon cache miss, retrieve game from the database
                let game = self.database.get_game(room_id.clone()).await?;
                has_result = game.get_moves().is_empty();
                Ok(GameRoom::new(game))
            },
        ).await.map_err(|arc_err| anyhow!(arc_err.to_string()))?;
        if !has_result { // add ongoing games back to the cache
            self.cache.insert(room_id, game_room.clone()).await;
        }
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
    use dotenv::dotenv;
    use uuid::Uuid;

    #[sqlx::test(migrations = "../persistence/migrations")]
    async fn new_game_stores_in_memory_and_db(pool: PgPool) {
        dotenv().ok();

        let database = EloStealoPostgresStore::new(pool.clone()).await.unwrap();
        let game_store = GameStore::new(100, database);

        let chess_game = ChessGame::new_game(None, None, None, None, 0, 0, 0, 0);

        let game_id = game_store.new_game(chess_game).await.unwrap();

        game_store.cache.run_pending_tasks().await;
        assert_eq!(1, game_store.cache.entry_count());
        assert!(game_store.database.get_game(game_id).await.is_ok());
    }

    #[sqlx::test(migrations = "../persistence/migrations")]
    async fn eviction_from_cache_saves_the_game_to_the_database(pool: PgPool) {
        dotenv().ok();

        let database = EloStealoPostgresStore::new(pool.clone()).await.unwrap();
        let game_store = GameStore::new(1, database);

        let uuid1 = Uuid::new_v4();
        let chess_game1 = ChessGame::new_game(None, None, Some(uuid1), None, 0, 0, 0, 0);
        let chess_game2 = ChessGame::new_game(None, None, None, None, 0, 0, 0, 0);

        let game_id1 = game_store.new_game(chess_game1).await.unwrap();
        let game_room1_result = game_store.try_get_game(game_id1.clone()).await;
        assert!(game_room1_result.is_ok());
        let game_room1 = game_room1_result.unwrap();
        game_room1.make_move("e2e4".to_string(), &uuid1).await;

        // Insert second game to trigger eviction of the first game
        let _game_id2 = game_store.new_game(chess_game2).await.unwrap();
        game_store.cache.run_pending_tasks().await;

        let game_room1_database_result = game_store.database.get_game(game_id1).await;
        assert!(game_room1_database_result.is_ok());
        let game_room1_database = game_room1_database_result.unwrap();
        assert_eq!(game_room1_database.get_position().to_string(), "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1");
    }

    #[sqlx::test(migrations = "../persistence/migrations")]
    async fn a_cache_miss_of_an_ongoing_game_should_put_the_game_back_in_the_cache(pool: PgPool) {
        dotenv().ok();

        let database = EloStealoPostgresStore::new(pool.clone()).await.unwrap();
        let game_store = GameStore::new(1, database);

        let uuid1 = Uuid::new_v4();
        let chess_game1 = ChessGame::new_game(None, None, Some(uuid1), None, 0, 0, 0, 0);

        let game_id1 = game_store.new_game(chess_game1).await.unwrap();

        // Insert second game to trigger eviction of the first game
        let chess_game2 = ChessGame::new_game(None, None, None, None, 0, 0, 0, 0);
        let _game_id2 = game_store.new_game(chess_game2).await.unwrap();

        // Now try to get the first game again, which should be a cache miss and load from DB
        let reload_game_room1_result = game_store.try_get_game(game_id1.clone()).await;

        game_store.cache.run_pending_tasks().await;

        assert!(reload_game_room1_result.is_ok());
        assert!(game_store.cache.contains_key(&game_id1));
    }
}