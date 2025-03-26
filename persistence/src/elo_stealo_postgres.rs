use crate::elo_stealo_repository::EloStealoRepository;
use crate::game_info::GameInfo;
use crate::game_model::{chess_game_to_model, model_to_chess_game, GameModel};
use crate::stealo_rule::StealoRule;
use domain::chessgame::ChessGame;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct EloStealoPostgresStore {
    pool: PgPool,
}

impl EloStealoPostgresStore {
    pub async fn new(connection_string: String) -> Result<EloStealoPostgresStore, anyhow::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(8)
            .connect(&connection_string)
            .await
            .expect("Could not connect to postgres");
        sqlx::migrate!("../migrations").run(&pool).await?;
        Ok(EloStealoPostgresStore { pool })
    }

    pub async fn save_game(&self, id: Uuid, new_game: ChessGame) -> anyhow::Result<()> {
        let game_model = chess_game_to_model(&new_game);
        sqlx::query!(
            r#"INSERT INTO games
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"#,
            id,
            game_model.game,
            game_model.white,
            game_model.black,
            game_model.elo_white,
            game_model.elo_black,
            game_model.rule_id_white,
            game_model.rule_id_black,
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_game(&self, id: Uuid) -> anyhow::Result<ChessGame> {
        let game_model = sqlx::query_as!(
            GameModel,
            r#"SELECT white, black, game, elo_white, elo_black, rule_id_white, rule_id_black
            FROM games WHERE id = $1"#,
            id
        )
        .fetch_one(&self.pool)
        .await?;
        let chess_game = model_to_chess_game(game_model);
        Ok(chess_game)
    }

    pub async fn update_game(&self, id: Uuid, game: &ChessGame) -> anyhow::Result<()> {
        let game_model = chess_game_to_model(game);
        sqlx::query!(
            r#"UPDATE games
            SET game = $1
            WHERE id = $2"#,
            game_model.game,
            id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn load_game_info(
        &self,
        id: Uuid,
        color: String,
    ) -> anyhow::Result<GameInfo> {
        let game_model = sqlx::query_as!(
            GameModel,
            r#"SELECT white, black, game, elo_white, elo_black, rule_id_white, rule_id_black FROM games
            WHERE id = $1"#,
            id
        ).fetch_one(&self.pool).await?;
        let chess_game = model_to_chess_game(game_model);
        Ok(GameInfo::new(chess_game, color))
    }

    pub async fn get_stealo_rules(&self) -> anyhow::Result<Vec<StealoRule>> {
        let rules = sqlx::query_as!(
            StealoRule,
            r#"SELECT id, name, elo, description FROM rules"#
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rules)
    }

    pub async fn insert_stealo_rule(&self, rule: StealoRule) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO rules
            VALUES ($1, $2, $3, $4)"#,
            rule.id,
            rule.name,
            rule.elo,
            rule.description
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn delete_old_stealo_rules(&self) -> anyhow::Result<()> {
        sqlx::query!(r#"DELETE FROM rules"#)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
