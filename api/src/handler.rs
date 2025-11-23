use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use futures_util::TryFutureExt;
use tower_sessions::{Session};
use tracing::log;
use uuid::Uuid;
use domain::chessgame::ChessGame;
use persistence::stealo_rule::StealoRule;
use crate::AppState;
use crate::dtos::new_game_dto::{NewLocalGameDTO};

const UUID_SESSION_KEY: &str = "user_uuid";

pub async fn start_local_game(
    State(state): State<AppState>,
    session: Session,
    Json(new_game): Json<NewLocalGameDTO>,
) -> Result<String, StatusCode> {
    let user_uuid: Uuid = get_or_create_user_uuid(session)
        .map_err(|_| return StatusCode::INTERNAL_SERVER_ERROR)
        .await?;
    let p2 = Some(new_game.player2);
    let p1 = Some(new_game.player1);
    let id_1 = Some(user_uuid.clone());
    let id_2 = Some(user_uuid);
    let elo1 = new_game.elo1;
    let elo2 = new_game.elo2;
    let stealo1 = new_game.stealo1;
    let stealo2 = new_game.stealo2;
    let new_game = ChessGame::new_game(p1, p2, id_1, id_2, elo1, elo2, stealo1, stealo2);
    let game_id = state.game_store.new_game(new_game)
        .map_err(|_| return StatusCode::INTERNAL_SERVER_ERROR)
        .await?;

    log::info!("Game {:?} created", game_id);
    Ok(game_id.into_string())
}

pub async fn get_or_create_user_uuid(session: Session) -> anyhow::Result<Uuid> {
    let user_uuid_option: Option<String> = session.get(UUID_SESSION_KEY).await?;
    match user_uuid_option {
        Some(user_uuid) => {
            Ok(Uuid::parse_str(&user_uuid)?)
        },
        None => {
            let new_uuid = Uuid::new_v4().to_string();
            session.insert(UUID_SESSION_KEY, &new_uuid).await?;
            Ok(Uuid::parse_str(&new_uuid)?)
        }
    }
}

pub async fn stealo_rules(State(state): State<AppState>) -> Json<Vec<StealoRule>> {
    let rules = state.game_store.get_stealo_rules().await.unwrap_or(Vec::new());
    Json(rules)
}