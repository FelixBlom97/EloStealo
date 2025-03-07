use sqlx::PgPool;
use domain::chessgame::ChessGame;
use crate::elo_stealo_repository::EloStealoRepository;
use crate::game_info::GameInfo;
use crate::stealo_rule::StealoRule;

pub struct EloStealoPostgresStore {
    pool: PgPool
}

impl EloStealoPostgresStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl EloStealoRepository for EloStealoPostgresStore {
    async fn save_game(&self, id: String, new_game: ChessGame) -> anyhow::Result<()> {
        todo!()
    }

    async fn get_game(&self, id: String) -> anyhow::Result<Option<ChessGame>> {
        todo!()
    }

    async fn update_game(&self, id: String, game: ChessGame) -> anyhow::Result<()> {
        todo!()
    }

    async fn load_game_info(&self, id: String, room_code: String, color: String) -> anyhow::Result<Option<GameInfo>> {
        todo!()
    }

    async fn get_stealo_rules(&self) -> anyhow::Result<Vec<StealoRule>> {
        todo!()
    }
}