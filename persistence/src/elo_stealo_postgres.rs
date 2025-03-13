use crate::elo_stealo_repository::EloStealoRepository;
use crate::game_info::GameInfo;
use crate::stealo_rule::StealoRule;
use domain::chessgame::ChessGame;
use sqlx::PgPool;

pub struct EloStealoPostgresStore {
    pool: PgPool,
}

impl EloStealoPostgresStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl EloStealoRepository for EloStealoPostgresStore {
    async fn save_game(&self, id: String, new_game: ChessGame) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO games (id, white, black, game, elo_white, elo_black, filter_id_white, filter_id_black)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
            new_game.id,
            new_game.white,
            new_game.black,
            new_game.game as _,
            new_game.elo_white,
            new_game.elo_black,
            new_game.filter_id_white,
            new_game.filter_id_black
        ).execute(&self.pool).await?;
    }

    async fn get_game(&self, id: String) -> anyhow::Result<Option<ChessGame>> {
        todo!()
    }

    async fn update_game(&self, id: String, game: ChessGame) -> anyhow::Result<()> {
        todo!()
    }

    async fn load_game_info(
        &self,
        id: String,
        room_code: String,
        color: String,
    ) -> anyhow::Result<Option<GameInfo>> {
        todo!()
    }

    async fn get_stealo_rules(&self) -> anyhow::Result<Vec<StealoRule>> {
        todo!()
    }
}
