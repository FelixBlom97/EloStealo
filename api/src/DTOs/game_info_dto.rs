use serde::{Deserialize, Serialize};
use uuid::Uuid;
use domain::chessgame::ChessGame;

#[derive(Serialize, Deserialize)]
pub struct GameInfoDTO {
    pub game_type: String,
    pub white: String,
    pub black: String,
    pub white_elo: i32,
    pub black_elo: i32,
    pub white_stealo: i32,
    pub black_stealo: i32,
}

impl GameInfoDTO {

    pub fn new(chess_game: &ChessGame, player_id: &Uuid) -> Self {

        // Local games get full info.
        if chess_game.white_id == chess_game.black_id {
            GameInfoDTO {
                game_type: "local".to_string(),
                white: chess_game.white.clone(),
                black: chess_game.black.clone(),
                white_elo: chess_game.elo_white,
                black_elo: chess_game.elo_black,
                white_stealo: chess_game.rule_id_white,
                black_stealo: chess_game.rule_id_black,
            }
        }
        // In online games, players only know their own Elo and rule, unless there is a result.
        else {
            let game_has_ended = chess_game.get_moves().is_empty();
            let (white_elo, white_stealo) = if game_has_ended || player_id == &chess_game.white_id {
                (chess_game.elo_white, chess_game.rule_id_white)
            } else {
                (0, 0)
            };
            let (black_elo, black_stealo) = if game_has_ended || player_id == &chess_game.black_id {
                (chess_game.elo_black, chess_game.rule_id_black)
            } else {
                (0, 0)
            };
            GameInfoDTO {
                game_type: "online".to_string(),
                white: chess_game.white.clone(),
                black: chess_game.black.clone(),
                white_elo,
                black_elo,
                white_stealo,
                black_stealo
            }
        }

    }
}