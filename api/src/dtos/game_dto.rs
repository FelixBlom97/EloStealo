use serde::{Deserialize, Serialize};
use domain::chessgame::ChessGame;

// Represents a game state for ongoing games.
// Represents a game history for concluded games.
#[derive(Deserialize, Serialize, Debug)]
pub struct GameDTO {
    board: String,
    moves: Vec<String>,
    result: String,
}

impl GameDTO {
    pub fn new(chess_game: &ChessGame) -> Self {
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
}