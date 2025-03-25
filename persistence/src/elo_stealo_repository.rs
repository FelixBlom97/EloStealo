use std::future::Future;
use crate::game_info::GameInfo;
use crate::stealo_rule::StealoRule;
use async_trait::async_trait;
use domain::chessgame::ChessGame;
use uuid::Uuid;

pub trait EloStealoRepository: Send + Sync + Clone + 'static {
    fn save_game(&self, id: Uuid, new_game: ChessGame) -> impl Future<Output = anyhow::Result<()>> + Send;
    fn get_game(&self, id: Uuid) -> impl Future<Output = anyhow::Result<ChessGame>> + Send;
    fn update_game(&self, id: Uuid, game: ChessGame) -> impl Future<Output = anyhow::Result<()>> + Send;
    fn load_game_info(
        &self,
        id: Uuid,
        color: String,
    ) -> impl Future<Output = anyhow::Result<GameInfo>> + Send;

    fn get_stealo_rules(&self) -> impl Future<Output = anyhow::Result<Vec<StealoRule>>> + Send;
    fn insert_stealo_rule(&self, rule: StealoRule) -> impl Future<Output = anyhow::Result<()>> + Send;
    fn delete_old_stealo_rules(&self) -> impl Future<Output = anyhow::Result<()>> + Send;
}
