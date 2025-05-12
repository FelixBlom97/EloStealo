use std::sync::Arc;
use moka::future::{Cache};
use moka::policy::EvictionPolicy;
use tracing::log;
use crate::game_room::GameRoom;

#[derive(Clone)]
pub struct GameCache(Cache<String, GameRoom>);

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

    pub async fn get_or_create_channel_and_game(&self, room_id: &str) -> GameRoom {
        let game_room = self.0.get_with(
            room_id.to_string(),
            async { GameRoom::new(room_id.to_string()) },
        ).await;
        game_room
    }

}
