use async_trait::async_trait;
use crate::game_info::GameInfo;
use crate::stealo_rule::StealoRule;
use domain::chessgame::ChessGame;

#[async_trait]
pub trait EloStealoRepository {
    async fn save_game(&self, id: String, new_game: ChessGame) -> anyhow::Result<()>;
    async fn get_game(&self, id: String) -> anyhow::Result<Option<ChessGame>>;
    async fn update_game(&self, id: String, game: ChessGame) -> anyhow::Result<()>;
    async fn load_game_info(
        &self,
        id: String,
        room_code: String,
        color: String,
    ) -> anyhow::Result<Option<GameInfo>>;

    async fn get_stealo_rules(&self) -> anyhow::Result<Vec<StealoRule>>;
    async fn insert_stealo_rule(&self, rule: StealoRule) -> anyhow::Result<()>;
    async fn delete_old_stealo_rules(&self) -> anyhow::Result<()>;
}
