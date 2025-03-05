use serde::{Deserialize, Serialize};
use domain::chessgame::ChessGame;

pub trait GameRepository {
    async fn save_game(&self, id: String, new_game: ChessGame) -> anyhow::Result<()>;
    async fn get_game(&self, id: String) -> anyhow::Result<Option<ChessGame>>;
    async fn update_game(&self, id: String, game: ChessGame) -> anyhow::Result<()>;
    async fn load_game_info(&self, id: String, room_code: String, color: String) -> anyhow::Result<Option<GameInfo>>;
}

#[derive(Serialize, Deserialize)]
pub struct GameInfo {
    pub white: String,
    pub black: String,
    pub white_elo: i32,
    pub black_elo: i32,
    pub white_stealo: i32,
    pub black_stealo: i32,
}