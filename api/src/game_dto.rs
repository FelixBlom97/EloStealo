#[allow(non_snake_case)]
use domain::chessgame::ChessGame;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct GameDTO {
    board: String,
    moves: Vec<String>,
    result: String,
}

#[derive(Deserialize)]
pub struct NewLocalGame {
    pub player1: String,
    pub player2: String,
    pub elo1: i32,
    pub elo2: i32,
    pub stealo1: i32,
    pub stealo2: i32,
}

#[derive(Deserialize)]
pub struct PlayMove {
    pub play_move: String,
    pub color: Option<String>,
}

#[derive(Deserialize)]
pub struct NewOnlineGame {
    pub roomcode: String,
    pub player1: String,
    pub player2: String,
    pub elo1: i32,
    pub elo2: i32,
    pub stealo1: i32,
    pub stealo2: i32,
}

#[derive(Deserialize)]
pub struct PlayOnlineMove {
    pub roomcode: String,
    pub play_move: String,
}

#[derive(Serialize, Deserialize)]
pub struct WaitingPlayer {
    pub room: String,
    pub name: String,
    pub elo: String,
    pub stealo: i32,
}

#[derive(Deserialize)]
pub struct GetInfo {
    pub roomcode: String,
    pub color: String,
}

#[derive(Serialize)]
pub struct GameInfoLocal {
    pub white: String,
    pub black: String,
    pub white_elo: i32,
    pub black_elo: i32,
    pub white_stealo: i32,
    pub black_stealo: i32,
}

pub fn create_game_dto(chess_game: &ChessGame) -> GameDTO {
    let available_moves = chess_game.get_moves_string();
    let game_result = if available_moves.len() != 0 {
        "none".to_string()
    } else {
        chess_game.winner_when_no_moves()
    };

    let game_dto = GameDTO {
        board: format!("{}", chess_game.game.current_position()),
        moves: available_moves,
        result: game_result,
    };
    game_dto
}
