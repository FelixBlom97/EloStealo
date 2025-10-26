use crate::game_model::{chess_game_to_model, model_to_chess_game, GameModel};
use crate::stealo_rule::StealoRule;
use domain::chessgame::ChessGame;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use crate::game_id::GameId;

#[derive(Clone, Debug)]
pub struct EloStealoPostgresStore {
    pool: PgPool,
}

impl EloStealoPostgresStore {
    pub async fn new(pool: PgPool) -> Result<EloStealoPostgresStore, anyhow::Error> {

        sqlx::migrate!("./migrations").run(&pool).await?;
        Ok(EloStealoPostgresStore { pool })
    }

    pub async fn save_game(&self, id: &GameId, new_game: &ChessGame) -> anyhow::Result<()> {
        let game_model = chess_game_to_model(&new_game);
        sqlx::query!(
            r#"INSERT INTO games
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)"#,
            id.as_str(),
            game_model.game,
            game_model.white,
            game_model.black,
            game_model.white_id,
            game_model.black_id,
            game_model.elo_white,
            game_model.elo_black,
            game_model.rule_id_white,
            game_model.rule_id_black,
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_game(&self, id: GameId) -> anyhow::Result<ChessGame> {
        let game_model = sqlx::query_as!(
            GameModel,
            r#"SELECT white, black, white_id, black_id, game, elo_white, elo_black, rule_id_white, rule_id_black
            FROM games WHERE id = $1"#,
            id.as_str()
        )
        .fetch_one(&self.pool)
        .await?;
        let chess_game = model_to_chess_game(game_model);
        Ok(chess_game)
    }

    pub async fn update_game(&self, id: GameId, game: &ChessGame) -> anyhow::Result<()> {
        let game_model = chess_game_to_model(game);
        sqlx::query!(
            r#"UPDATE games
            SET game = $1
            WHERE id = $2"#,
            game_model.game,
            id.as_str()
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn update_game_from_model(&self, id: GameId, game_model: GameModel) -> anyhow::Result<()> {
        sqlx::query!(
            r#"UPDATE games
            SET game = $1
            WHERE id = $2"#,
            game_model.game,
            id.as_str()
        )
        .execute(&self.pool)
        .await?;
        Ok(())
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
}
