use crate::game_dto::{
    create_game_dto, GameDTO, GameInfoLocal, GetInfo, NewLocalGame, NewOnlineGame, PlayMove,
};
use crate::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use domain::chessgame::ChessGame;
use tower_sessions::Session;
use tracing::log;
use uuid::Uuid;
use persistence::game_id::GameId;
use persistence::stealo_rule::StealoRule;
use crate::DTOs::game_info_dto::GameInfoDTO;
// Local play

pub async fn play(
    State(state): State<AppState>,
    session: Session,
    Json(play_move): Json<PlayMove>,
) -> Result<Json<GameDTO>, StatusCode> {
    let id: GameId = match session.get("gameId").await.unwrap() {
        Some(id) => id,
        None => {
            log::error!("No gameId set for session");
            return Err(StatusCode::BAD_REQUEST);
        }
    };
    let mut chess_game: ChessGame = state.repository.get_game(id.clone())
        .await
        .map_err(|_e| return StatusCode::INTERNAL_SERVER_ERROR)?;
    chess_game.make_move(play_move.play_move, play_move.color);
    match state.repository.update_game(id, &chess_game).await {
        Ok(()) => {
            let game_dto = create_game_dto(&chess_game);
            Ok(Json(game_dto))
        }
        Err(_e) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_local_info(
    State(state): State<AppState>,
    session: Session,
) -> Result<Json<GameInfoLocal>, StatusCode> {
    let id: GameId = match session.get("gameId").await.unwrap().into() {
        Some(id) => id,
        None => {
            log::error!("No gameId set for session");
            return Err(StatusCode::BAD_REQUEST);
        }
    };
    let chess_game: ChessGame = state.repository.get_game(id)
        .await
        .map_err(|_e| return StatusCode::INTERNAL_SERVER_ERROR)?;
    let local_game_info = GameInfoLocal {
        white: chess_game.white,
        black: chess_game.black,
        white_elo: chess_game.elo_white,
        black_elo: chess_game.elo_black,
        white_stealo: chess_game.rule_id_white,
        black_stealo: chess_game.rule_id_black,
    };
    Ok(Json(local_game_info))
}

// Sends an empty vector if it fails
pub async fn stealo_rules(State(state): State<AppState>) -> Json<Vec<StealoRule>> {
    let rules = state.repository.get_stealo_rules().await.unwrap_or(Vec::new());
    Json(rules)
}

