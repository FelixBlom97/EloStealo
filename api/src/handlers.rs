use std::str::FromStr;
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
use persistence::game_info::GameInfo;
use persistence::stealo_rule::StealoRule;

// Local play
pub async fn start_game(
    State(state): State<AppState>,
    session: Session,
    Json(new_game): Json<NewLocalGame>,
) -> Result<Json<GameDTO>, StatusCode> {
    println!("{:?}", session.id());
    let p1 = new_game.player1;
    let p2 = new_game.player2;
    let elo1 = new_game.elo1;
    let elo2 = new_game.elo2;
    let stealo1 = new_game.stealo1;
    let stealo2 = new_game.stealo2;
    let id = Uuid::now_v7();
    session.insert("gameId", id.to_string()).await.unwrap();
    let new_game = domain::chessgame::new_game(p1, p2, elo1, elo2, stealo1, stealo2);
    let game_dto = create_game_dto(&new_game);
    match state.repository.save_game(id, new_game).await {
        Ok(()) => Ok(Json(game_dto)),
        Err(_e) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn play(
    State(state): State<AppState>,
    session: Session,
    Json(play_move): Json<PlayMove>,
) -> Result<Json<GameDTO>, StatusCode> {
    let id = match session.get("gameId").await.unwrap() {
        Some(id) => id,
        None => {
            log::error!("No gameId set for session");
            return Err(StatusCode::BAD_REQUEST);
        }
    };
    let mut chess_game: ChessGame = state.repository.get_game(id)
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
    let id: Uuid = match session.get("gameId").await.unwrap() {
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

// Online play
pub async fn start_online(
    State(state): State<AppState>,
    Json(new_game): Json<NewOnlineGame>,
) -> Result<Json<GameDTO>, StatusCode> {
    let id = new_game.roomcode;
    let p1 = new_game.player1;
    let p2 = new_game.player2;
    let elo1 = new_game.elo1;
    let elo2 = new_game.elo2;
    let stealo1 = new_game.stealo1;
    let stealo2 = new_game.stealo2;
    let new_game = domain::chessgame::new_game(p1, p2, elo1, elo2, stealo1, stealo2);
    let game_dto = create_game_dto(&new_game);
    match state.repository.save_game(Uuid::from_str(&id).unwrap(), new_game).await {
        Ok(()) => Ok(Json(game_dto)),
        Err(_e) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_game_info(
    State(state): State<AppState>,
    Json(get_rule): Json<GetInfo>,
) -> Result<Json<GameInfo>, StatusCode> {
    let game_info = state.repository.load_game_info(Uuid::from_str(&get_rule.roomcode).unwrap(), get_rule.color).await;
    match game_info {
        Ok(info) => Ok(Json(info)),
        Err(e) => {
            log::error!("Failed to fetch game info: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
