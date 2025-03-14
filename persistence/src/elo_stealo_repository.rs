use crate::game_info::GameInfo;
use crate::stealo_rule::StealoRule;
use async_trait::async_trait;
use domain::chessgame::ChessGame;
use uuid::Uuid;

#[async_trait]
pub trait EloStealoRepository {
    async fn save_game(&self, id: Uuid, new_game: ChessGame) -> anyhow::Result<()>;
    async fn get_game(&self, id: Uuid) -> anyhow::Result<ChessGame>;
    async fn update_game(&self, id: Uuid, game: ChessGame) -> anyhow::Result<()>;
    async fn load_game_info(
        &self,
        id: Uuid,
        room_code: String,
        color: String,
    ) -> anyhow::Result<Option<GameInfo>>;

    async fn get_stealo_rules(&self) -> anyhow::Result<Vec<StealoRule>>;
    async fn insert_stealo_rule(&self, rule: StealoRule) -> anyhow::Result<()>;
    async fn delete_old_stealo_rules(&self) -> anyhow::Result<()>;
}
