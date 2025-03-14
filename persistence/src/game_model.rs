use anyhow::anyhow;
use chess::{Action, ChessMove, Color, Game, Piece, Square, Board, MoveGen};
use domain::chessgame::ChessGame;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GameModel {
    pub white: String,
    pub black: String,
    pub game: Vec<u8>,
    pub elo_white: i32,
    pub elo_black: i32,
    pub filter_id_white: i32,
    pub filter_id_black: i32,
}

pub fn chess_game_to_model(chess_game: &ChessGame) -> GameModel {
    GameModel {
        white: chess_game.white.clone(),
        black: chess_game.black.clone(),
        game: encode_game(&chess_game.game).unwrap(),
        elo_white: chess_game.elo_white,
        elo_black: chess_game.elo_black,
        filter_id_white: chess_game.rule_id_white,
        filter_id_black: chess_game.rule_id_black,
    }
}

pub fn model_to_chess_game(game_model: GameModel) -> ChessGame {
    ChessGame {
        white: game_model.white,
        black: game_model.black,
        elo_white: game_model.elo_white,
        elo_black: game_model.elo_black,
        rule_id_white: game_model.filter_id_white,
        rule_id_black: game_model.filter_id_black,
        game: decode_game(game_model.game).unwrap(),
    }
}

// MoveGen is deterministic and the currently known position with the most allowed moves is 218.
// Therefore, we can encode every move into a byte (number in MoveGen) and have space left for
// special actions like offering draws and resigning which we'll put at the end of the byte range.
fn encode_game(game: &Game) -> anyhow::Result<Vec<u8>> {
    let mut result: Vec<u8> = Vec::with_capacity(game.actions().len());
    let mut current_pos = Board::default();
    for action in game.actions() {
        match action {
            Action::MakeMove(chess_move) => {
                let move_index = MoveGen::new_legal(&current_pos).position(|m| m == *chess_move)
                    .ok_or_else(|| anyhow!("Cannot encode game"))?;
                result.push(move_index as u8);
                current_pos = current_pos.make_move_new(*chess_move);
            }
            Action::Resign(Color::White) => { result.push(255); }
            Action::Resign(Color::Black) => { result.push(254); }
            Action::OfferDraw(Color::White) => {result.push(253)}
            Action::OfferDraw(Color::Black) => {result.push(252)}
            Action::AcceptDraw => {result.push(251)}
            Action::DeclareDraw => {result.push(250)}
        }
    }
    Ok(result)
}

fn decode_game(game: Vec<u8>) -> anyhow::Result<Game> {
    let mut result = Game::new();
    for action in game {
        match action {
            255 => {result.resign(Color::White);},
            254 => {result.resign(Color::Black);},
            253 => {result.offer_draw(Color::White);},
            252 => {result.offer_draw(Color::Black);},
            251 => {result.accept_draw();},
            250 => {result.declare_draw();},
            n   => {
                let mut moves = MoveGen::new_legal(&result.current_position());
                let next_move = moves.nth(n as usize).ok_or_else(|| anyhow!("Cannot decode game"))?;
                result.make_move(next_move);
            }
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chess::{ChessMove, Square};

    #[test]
    pub fn test_encode_game() {
        let mut game = Game::new();
        game.make_move(ChessMove::new(Square::E2, Square::E4, None));
        game.make_move(ChessMove::new(Square::E7, Square::E5, None));
        let encoded = encode_game(&game).unwrap();
        assert_eq!(encoded, vec![9,8]);
    }

    #[test]
    pub fn test_encode_game_special_actions() {
        let mut game = Game::new();
        game.make_move(ChessMove::new(Square::E2, Square::E4, None));
        game.offer_draw(Color::White);
        game.resign(Color::White);
        let encoded = encode_game(&game).unwrap();
        assert_eq!(encoded, vec![9, 253, 255]);
    }

    #[test]
    pub fn test_decode_game() {
        let db_game = vec![8, 9];
        let game = decode_game(db_game).unwrap();
        assert_eq!(game.actions().len(), 2);
        assert!(game.result().is_none());
    }

    #[test]
    pub fn test_decode_game_special_actions() {
        let db_game = vec![8, 9, 252, 251];
        let game = decode_game(db_game).unwrap();
        assert_eq!(game.actions().len(), 4);
        assert!(game.result().is_some());
    }

}
