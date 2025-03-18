use serde::{Deserialize, Serialize};
use domain::chessgame::ChessGame;

#[derive(Serialize, Deserialize)]
pub struct GameInfo {
    pub white: String,
    pub black: String,
    pub white_elo: i32,
    pub black_elo: i32,
    pub white_stealo: i32,
    pub black_stealo: i32,
}

impl GameInfo {
    pub fn new(chess_game: ChessGame, color: String) -> Self {
        let game_has_ended = chess_game.get_moves().is_empty();
        let white_elo = if color == "white" || game_has_ended {
            chess_game.elo_white
        } else {
            0
        };
        let white_stealo = if color == "white" || game_has_ended {
            chess_game.rule_id_white
        } else {
            0
        };
        let black_elo = if color == "black" || game_has_ended {
            chess_game.elo_black
        } else {
            0
        };
        let black_stealo = if color == "black" || game_has_ended {
            chess_game.rule_id_black
        } else {
            0
        };
        Self {
            white: chess_game.white,
            black: chess_game.black,
            white_elo,
            black_elo,
            white_stealo,
            black_stealo
        }
    }
}