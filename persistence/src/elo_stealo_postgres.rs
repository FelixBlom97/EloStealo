use async_trait::async_trait;
use crate::elo_stealo_repository::EloStealoRepository;
use crate::game_info::GameInfo;
use crate::stealo_rule::StealoRule;
use domain::chessgame::ChessGame;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

pub struct EloStealoPostgresStore {
    pool: PgPool,
}

impl EloStealoPostgresStore {
    pub async fn new(connection_string: String) -> Box<dyn EloStealoRepository> {
        let pool = PgPoolOptions::new().connect(&connection_string).await.expect("Could not connect to postgres");
        Box::new(EloStealoPostgresStore { pool })
    }
}

#[async_trait]
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

    async fn load_game_info(
        &self,
        id: String,
        room_code: String,
        color: String,
    ) -> anyhow::Result<Option<GameInfo>> {
        todo!()
    }

    async fn get_stealo_rules(&self) -> anyhow::Result<Vec<StealoRule>> {
        let rules = sqlx::query_as!(
            StealoRule,
            r#"SELECT id, name, elo, description FROM rules"#
        ).fetch_all(&self.pool).await?;
        Ok(rules)
    }

    async fn insert_stealo_rule(&self, rule: StealoRule) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO rules
            VALUES ($1, $2, $3, $4)"#,
            rule.id,
            rule.name,
            rule.elo,
            rule.description
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn delete_old_stealo_rules(&self) -> anyhow::Result<()> {
        sqlx::query!(
            r#"DELETE FROM rules"#
        ).execute(&self.pool).await?;
        Ok(())
    }
}
