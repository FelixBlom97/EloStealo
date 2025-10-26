use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use futures_util::TryFutureExt;
use tower_sessions::{Session};
use tracing::log;
use persistence::game_id::GameId;
use uuid::Uuid;
use domain::chessgame::ChessGame;
use crate::AppState;
use crate::DTOs::game_info_dto::GameInfoDTO;
use crate::DTOs::new_game_dto::{NewLocalGameDTO, NewOnlineGameDTO};

const UUID_SESSION_KEY: &str = "user_uuid";

pub async fn start_local_game(
    State(state): State<AppState>,
    session: Session,
    Json(new_game): Json<NewLocalGameDTO>,
) -> Result<String, StatusCode> {
    let user_uuid: Uuid = get_or_create_user_uuid(session)
        .map_err(|_| return StatusCode::INTERNAL_SERVER_ERROR)
        .await?;
    let p1 = new_game.player1;
    let p2 = new_game.player2;
    let id_1 = user_uuid.clone();
    let id_2 = user_uuid;
    let elo1 = new_game.elo1;
    let elo2 = new_game.elo2;
    let stealo1 = new_game.stealo1;
    let stealo2 = new_game.stealo2;
    let id = GameId::new();
    let new_game = ChessGame::new_game(p1, p2, id_1, id_2, elo1, elo2, stealo1, stealo2);
    let _ = state.repository.save_game(&id, &new_game)
        .map_err(|_| return StatusCode::INTERNAL_SERVER_ERROR)
        .await?;
    state.cache.insert(id.clone(), new_game).await;

    log::info!("Game {:?} started", id);
    Ok(id.into_string())
}

pub async fn start_online_game(
    State(state): State<AppState>,
    session: Session,
    new_online_game_dto: NewOnlineGameDTO
) -> Result<String, StatusCode> {
    let user_uuid: Uuid = get_or_create_user_uuid(session)
        .map_err(|_| return StatusCode::INTERNAL_SERVER_ERROR)
        .await?;
    let p1 = new_online_game_dto.player1;
    let elo1 = new_online_game_dto.elo1;

    Ok(p1.to_string())
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