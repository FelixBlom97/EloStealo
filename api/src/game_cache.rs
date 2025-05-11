use std::sync::Arc;
use moka::future::{Cache};
use moka::policy::EvictionPolicy;
use tokio::sync::broadcast::{Receiver, Sender};
use tracing::log;
use crate::game_room::GameRoom;

#[derive(Clone)]
pub struct GameCache(Cache<String, Arc<GameRoom>>);

impl GameCache {
    pub fn new(capacity: u64) -> Self {
        let write_to_database = |key, _value, cause| {
            log::info!("Game {} evicted from GameCache, reason: {:?}", key, cause);
            // Save game in database
        };

        let cache = Cache::builder()
            .max_capacity(capacity)
            .eviction_policy(EvictionPolicy::lru())
            .eviction_listener(write_to_database)
            .build();

        Self(cache)
    }

    pub async fn get_or_create_transceivers(&self, room_id: &str) -> (Sender<String>, Receiver<String>) {
        let game_room = self.0.get_with(
            room_id.to_string(),
            async { Arc::new(GameRoom::new(room_id.to_string())) },
        ).await;
        let tx = game_room.tx.clone();
        let rx = game_room.tx.subscribe();
        (tx, rx)
    }

    pub async fn get(&self, id: &String) -> Option<Arc<GameRoom>> {
        self.0.get(id).await
    }
}
