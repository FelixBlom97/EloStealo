use chess::{Action, ChessMove, Color, Game, Piece, Square};
use domain::chessgame::ChessGame;
use mongodb::bson::{spec::BinarySubtype, Binary};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ChessDTO {
    pub white: String,
    pub black: String,
    pub game: Binary,
    pub elo_white: i32,
    pub elo_black: i32,
    pub filter_id_white: i32,
    pub filter_id_black: i32,
}

pub fn to_dto(chess_game: &ChessGame) -> ChessDTO {
    let game_bytes = Binary {
        subtype: BinarySubtype::Generic,
        bytes: encode_game(&chess_game.game),
    };
    ChessDTO {
        white: chess_game.white.clone(),
        black: chess_game.black.clone(),
        game: game_bytes,
        elo_white: chess_game.elo_white,
        elo_black: chess_game.elo_black,
        filter_id_white: chess_game.filter_id_white,
        filter_id_black: chess_game.filter_id_black,
    }
}

pub fn from_dto(chess_dto: ChessDTO) -> ChessGame {
    let db_game: Vec<u8> = chess_dto.game.bytes;
    ChessGame {
        white: chess_dto.white,
        black: chess_dto.black,
        elo_white: chess_dto.elo_white,
        elo_black: chess_dto.elo_black,
        filter_id_white: chess_dto.filter_id_white,
        filter_id_black: chess_dto.filter_id_black,
        game: decode_game(db_game),
    }
}

// Represent each move as 3 bytes:
// On a regular move: First for source square, second for destination square, third byte 0
// On a promotion move the third byte is used for promotion piece: 1=Q, 2=R, 3=B, 4=N
// Third byte 5: Resign
// Third byte 6: Offer draw
// On these special actions, the first two bytes are 0 if white and 1 if black.
fn encode_game(game: &Game) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::with_capacity(game.actions().len() * 3);
    for action in game.actions() {
        match action {
            Action::MakeMove(chess_move) => {
                result.push(chess_move.get_source().to_int());
                result.push(chess_move.get_dest().to_int());
                match chess_move.get_promotion() {
                    Some(Piece::Queen) => result.push(1),
                    Some(Piece::Rook) => result.push(2),
                    Some(Piece::Bishop) => result.push(3),
                    Some(Piece::Knight) => result.push(4),
                    _ => result.push(0),
                }
            }
            Action::Resign(color) => {
                let c = color.to_index() as u8; // 0 for white, 1 for black.
                result.push(c);
                result.push(c);
                result.push(5);
            }
            Action::OfferDraw(color) => {
                let c = color.to_index() as u8;
                result.push(c);
                result.push(c);
                result.push(6);
            }
            _ => {}
        }
    }
    result
}

fn decode_game(db_game: Vec<u8>) -> Game {
    let mut result = Game::new();
    for chunk in db_game.chunks(3) {
        if let [source, dest, info] = chunk {
            unsafe {
                // unsafe since Square::new panics for value > 63
                let _ = match info {
                    1 => result.make_move(ChessMove::new(
                        Square::new(*source),
                        Square::new(*dest),
                        Some(Piece::Queen),
                    )),
                    2 => result.make_move(ChessMove::new(
                        Square::new(*source),
                        Square::new(*dest),
                        Some(Piece::Rook),
                    )),
                    3 => result.make_move(ChessMove::new(
                        Square::new(*source),
                        Square::new(*dest),
                        Some(Piece::Bishop),
                    )),
                    4 => result.make_move(ChessMove::new(
                        Square::new(*source),
                        Square::new(*dest),
                        Some(Piece::Knight),
                    )),
                    5 => result.resign(to_color(source)),
                    6 => result.offer_draw(to_color(source)),
                    _ => result.make_move(ChessMove::new(
                        Square::new(*source),
                        Square::new(*dest),
                        None,
                    )),
                };
            }
        } else {
            break;
        }
    }
    result
}

fn to_color(num: &u8) -> Color {
    if *num == 0 {
        Color::White
    } else {
        Color::Black
    }
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
        let encoded = encode_game(&game);
        assert_eq!(encoded, vec![12, 28, 0, 52, 36, 0]);
    }

    #[test]
    pub fn test_encode_game_special_actions() {
        let mut game = Game::new();
        game.make_move(ChessMove::new(Square::E2, Square::E4, None));
        game.offer_draw(Color::White);
        game.resign(Color::White);
        let encoded = encode_game(&game);
        assert_eq!(encoded, vec![12, 28, 0, 0, 0, 6, 0, 0, 5]);
    }

    #[test]
    pub fn test_decode_game() {
        let db_game = vec![12, 28, 0, 52, 36, 0];
        let game = decode_game(db_game);
        assert_eq!(game.actions().len(), 2);
        assert!(game.result().is_none());
    }

    #[test]
    pub fn test_decode_game_special_actions() {
        let db_game = vec![12, 28, 0, 52, 36, 0, 0, 0, 5];
        let game = decode_game(db_game);
        assert_eq!(game.actions().len(), 3);
        assert!(game.result().is_some());
    }
}
