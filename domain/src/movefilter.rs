use crate::filters::cantcapture::CantCapture;
use crate::filters::moveafter::MoveAfter;
use crate::filters::moveto::MoveTo;
use crate::filters::nofilter::NoFilter;
use crate::filters::openingmove::OpeningMove;
use chess::{ChessMove, Game, MoveGen, Piece, Square};

pub trait MoveFilter {
    fn filter(&self, game: &Game, chess_move: &ChessMove) -> bool;

    fn filter_moves(&self, game: &Game) -> Vec<ChessMove> {
        let mut moves = MoveGen::new_legal(&game.current_position());
        let mut result: Vec<ChessMove> = Vec::new();

        for chess_move in &mut moves {
            if !Self::filter(self, game, &chess_move) {
                result.push(chess_move);
            }
        }
        result
    }
}

// Select filter based on id and filter the moves. Default is normal chess.
// Range 1-21 are filters that limit captures.
// Range 22-33 Prevents pieces from moving after move x
// Range 34-45 limits movement of pieces to certain squares
// Range 45-59 forces a number of opening moves
pub fn generate_moves(filter_id: i32, game: &Game) -> Vec<ChessMove> {
    match filter_id {
        59 => OpeningMove::new(vec![
            ChessMove::new(Square::B1, Square::A3, None),
            ChessMove::new(Square::B8, Square::A6, None),
            ChessMove::new(Square::G1, Square::H3, None),
            ChessMove::new(Square::G8, Square::H6, None),
        ])
        .filter_moves(game),
        58 => OpeningMove::new(vec![
            ChessMove::new(Square::E2, Square::E4, None),
            ChessMove::new(Square::E7, Square::E5, None),
            ChessMove::new(Square::E1, Square::E2, None),
            ChessMove::new(Square::E8, Square::E7, None),
            ChessMove::new(Square::E2, Square::E1, None),
            ChessMove::new(Square::E7, Square::E8, None),
        ])
        .filter_moves(game),
        57 => OpeningMove::new(vec![
            ChessMove::new(Square::E2, Square::E4, None),
            ChessMove::new(Square::E7, Square::E5, None),
            ChessMove::new(Square::E1, Square::E2, None),
            ChessMove::new(Square::E8, Square::E7, None),
        ])
        .filter_moves(game),
        56 => OpeningMove::new(vec![
            ChessMove::new(Square::F2, Square::F3, None),
            ChessMove::new(Square::F7, Square::F6, None),
            ChessMove::new(Square::G2, Square::G4, None),
            ChessMove::new(Square::G7, Square::G5, None),
        ])
        .filter_moves(game),
        55 => OpeningMove::new(vec![
            ChessMove::new(Square::A2, Square::A4, None),
            ChessMove::new(Square::A7, Square::A5, None),
            ChessMove::new(Square::A1, Square::A3, None),
            ChessMove::new(Square::A8, Square::A6, None),
            ChessMove::new(Square::H2, Square::H4, None),
            ChessMove::new(Square::H7, Square::H5, None),
            ChessMove::new(Square::H1, Square::H3, None),
            ChessMove::new(Square::H8, Square::H6, None),
        ])
        .filter_moves(game),
        54 => OpeningMove::new(vec![
            ChessMove::new(Square::F2, Square::F3, None),
            ChessMove::new(Square::F7, Square::F6, None),
            ChessMove::new(Square::F3, Square::F4, None),
            ChessMove::new(Square::F6, Square::F5, None),
        ])
        .filter_moves(game),
        53 => OpeningMove::new(vec![
            ChessMove::new(Square::E2, Square::E4, None),
            ChessMove::new(Square::E7, Square::E5, None),
            ChessMove::new(Square::F1, Square::C4, None),
            ChessMove::new(Square::F8, Square::C5, None),
            ChessMove::new(Square::D1, Square::H5, None),
            ChessMove::new(Square::D8, Square::H4, None),
        ])
        .filter_moves(game),
        52 => OpeningMove::new(vec![
            ChessMove::new(Square::B2, Square::B4, None),
            ChessMove::new(Square::B7, Square::B5, None),
            ChessMove::new(Square::B4, Square::B5, None),
            ChessMove::new(Square::B5, Square::B4, None),
        ])
        .filter_moves(game),
        51 => OpeningMove::new(vec![
            ChessMove::new(Square::A2, Square::A4, None),
            ChessMove::new(Square::A7, Square::A5, None),
            ChessMove::new(Square::A4, Square::A5, None),
            ChessMove::new(Square::A5, Square::A4, None),
        ])
        .filter_moves(game),
        50 => OpeningMove::new(vec![
            ChessMove::new(Square::C2, Square::C4, None),
            ChessMove::new(Square::C7, Square::C5, None),
            ChessMove::new(Square::D2, Square::D3, None),
            ChessMove::new(Square::D7, Square::D6, None),
            ChessMove::new(Square::E2, Square::E4, None),
            ChessMove::new(Square::E7, Square::E5, None),
        ])
        .filter_moves(game),
        49 => OpeningMove::new(vec![
            ChessMove::new(Square::A2, Square::A4, None),
            ChessMove::new(Square::A7, Square::A5, None),
            ChessMove::new(Square::H2, Square::H4, None),
            ChessMove::new(Square::H7, Square::H5, None),
        ])
        .filter_moves(game),
        48 => OpeningMove::new(vec![
            ChessMove::new(Square::D2, Square::D3, None),
            ChessMove::new(Square::D7, Square::D6, None),
            ChessMove::new(Square::F2, Square::F3, None),
            ChessMove::new(Square::F7, Square::F6, None),
        ])
        .filter_moves(game),
        47 => OpeningMove::new(vec![
            ChessMove::new(Square::B1, Square::C3, None),
            ChessMove::new(Square::B8, Square::C6, None),
            ChessMove::new(Square::C3, Square::B1, None),
            ChessMove::new(Square::C6, Square::B8, None),
        ])
        .filter_moves(game),
        46 => OpeningMove::new(vec![
            ChessMove::new(Square::G2, Square::G3, None),
            ChessMove::new(Square::G7, Square::G6, None),
            ChessMove::new(Square::G3, Square::G4, None),
            ChessMove::new(Square::G6, Square::G5, None),
        ])
        .filter_moves(game),
        // Cant play on the C, E, H files
        45 => MoveTo::new(
            (0..63)
                .filter(|&x| x % 8 == 4 || x % 8 == 7 || x % 8 == 2)
                .collect(),
            vec![
                Piece::Pawn,
                Piece::Knight,
                Piece::Bishop,
                Piece::Rook,
                Piece::Queen,
                Piece::King,
            ],
        )
        .filter_moves(game),
        // Can't play on the C, H files
        44 => MoveTo::new(
            (0..63).filter(|&x| x % 8 == 2 || x % 8 == 7).collect(),
            vec![
                Piece::Pawn,
                Piece::Knight,
                Piece::Bishop,
                Piece::Rook,
                Piece::Queen,
                Piece::King,
            ],
        )
        .filter_moves(game),
        // Can't play on the C, E files
        43 => MoveTo::new(
            (0..63).filter(|&x| x % 8 == 2 || x % 8 == 4).collect(),
            vec![
                Piece::Pawn,
                Piece::Knight,
                Piece::Bishop,
                Piece::Rook,
                Piece::Queen,
                Piece::King,
            ],
        )
        .filter_moves(game),
        // Can't play on the E, H files
        42 => MoveTo::new(
            (0..63).filter(|&x| x % 8 == 4 || x % 8 == 7).collect(),
            vec![
                Piece::Pawn,
                Piece::Knight,
                Piece::Bishop,
                Piece::Rook,
                Piece::Queen,
                Piece::King,
            ],
        )
        .filter_moves(game),
        // Can't play on the E file
        41 => MoveTo::new(
            (0..8).map(|x| x * 8 + 4).collect(),
            vec![
                Piece::Pawn,
                Piece::Knight,
                Piece::Bishop,
                Piece::Rook,
                Piece::Queen,
                Piece::King,
            ],
        )
        .filter_moves(game),
        // Can't play on the H file
        40 => MoveTo::new(
            (0..8).map(|x| x * 8 + 7).collect(),
            vec![
                Piece::Pawn,
                Piece::Knight,
                Piece::Bishop,
                Piece::Rook,
                Piece::Queen,
                Piece::King,
            ],
        )
        .filter_moves(game),
        // Can't play on the C file
        39 => MoveTo::new(
            (0..8).map(|x| x * 8 + 2).collect(),
            vec![
                Piece::Pawn,
                Piece::Knight,
                Piece::Bishop,
                Piece::Rook,
                Piece::Queen,
                Piece::King,
            ],
        )
        .filter_moves(game),
        // Queen can only move to light squares
        38 => MoveTo::new(
            (0..32).map(|x| x * 2 + (x % 8) / 4).collect(),
            vec![Piece::Queen],
        )
        .filter_moves(game),
        // Queen can only move to dark squares
        37 => MoveTo::new(
            (0..32).map(|x| x * 2 + (x % 8) / 4 + 1).collect(),
            vec![Piece::Queen],
        )
        .filter_moves(game),
        // King can only move to light squares
        36 => MoveTo::new(
            (0..32).map(|x| x * 2 + (x % 8) / 4).collect(),
            vec![Piece::King],
        )
        .filter_moves(game),
        // King can only move to dark squares
        35 => MoveTo::new(
            (0..32).map(|x| 1 + x * 2 + (x % 8) / 4).collect(),
            vec![Piece::King],
        )
        .filter_moves(game),
        // Rooks can only move along the edges
        34 => MoveTo::new(
            (0..64)
                .filter(|&x| x % 8 != 0 && (x + 1) % 8 != 0 && x > 7 && x < 56)
                .collect(),
            vec![Piece::Rook],
        )
        .filter_moves(game),
        33 => MoveAfter::new(Piece::Knight, 10).filter_moves(game),
        32 => MoveAfter::new(Piece::Knight, 15).filter_moves(game),
        31 => MoveAfter::new(Piece::Knight, 20).filter_moves(game),
        30 => MoveAfter::new(Piece::Bishop, 10).filter_moves(game),
        29 => MoveAfter::new(Piece::Bishop, 15).filter_moves(game),
        28 => MoveAfter::new(Piece::Bishop, 20).filter_moves(game),
        27 => MoveAfter::new(Piece::Rook, 15).filter_moves(game),
        26 => MoveAfter::new(Piece::Rook, 20).filter_moves(game),
        25 => MoveAfter::new(Piece::Rook, 25).filter_moves(game),
        24 => MoveAfter::new(Piece::Queen, 6).filter_moves(game),
        23 => MoveAfter::new(Piece::Queen, 9).filter_moves(game),
        22 => MoveAfter::new(Piece::Queen, 12).filter_moves(game),
        21 => CantCapture::new(
            vec![
                Piece::Pawn,
                Piece::Knight,
                Piece::Bishop,
                Piece::Rook,
                Piece::Queen,
                Piece::King,
            ],
            vec![Piece::Knight],
        )
        .filter_moves(game),
        20 => CantCapture::new(
            vec![
                Piece::Pawn,
                Piece::Knight,
                Piece::Bishop,
                Piece::Rook,
                Piece::Queen,
                Piece::King,
            ],
            vec![Piece::Bishop],
        )
        .filter_moves(game),
        19 => CantCapture::new(
            vec![
                Piece::Pawn,
                Piece::Knight,
                Piece::Bishop,
                Piece::Rook,
                Piece::Queen,
                Piece::King,
            ],
            vec![Piece::Rook],
        )
        .filter_moves(game),
        18 => CantCapture::new(vec![Piece::Pawn], vec![Piece::Pawn]).filter_moves(game),
        17 => CantCapture::new(
            vec![Piece::Pawn],
            vec![Piece::Knight, Piece::Bishop, Piece::Rook, Piece::Queen],
        )
        .filter_moves(game),
        16 => CantCapture::new(
            vec![
                Piece::Knight,
                Piece::Bishop,
                Piece::Rook,
                Piece::Queen,
                Piece::King,
            ],
            vec![Piece::Pawn],
        )
        .filter_moves(game),
        15 => CantCapture::new(
            vec![Piece::Queen],
            vec![Piece::Pawn, Piece::Knight, Piece::Rook, Piece::Queen],
        )
        .filter_moves(game),
        14 => CantCapture::new(
            vec![Piece::Queen],
            vec![Piece::Pawn, Piece::Bishop, Piece::Rook, Piece::Queen],
        )
        .filter_moves(game),
        13 => CantCapture::new(
            vec![Piece::Queen],
            vec![Piece::Pawn, Piece::Knight, Piece::Bishop, Piece::Queen],
        )
        .filter_moves(game),
        12 => CantCapture::new(
            vec![Piece::Queen],
            vec![Piece::Knight, Piece::Bishop, Piece::Rook, Piece::Queen],
        )
        .filter_moves(game),
        11 => CantCapture::new(vec![Piece::Rook], vec![Piece::Queen]).filter_moves(game),
        10 => CantCapture::new(vec![Piece::Bishop], vec![Piece::Queen]).filter_moves(game),
        9 => CantCapture::new(vec![Piece::Knight], vec![Piece::Queen]).filter_moves(game),
        8 => CantCapture::new(
            vec![Piece::Knight],
            vec![
                Piece::Pawn,
                Piece::Knight,
                Piece::Bishop,
                Piece::Rook,
                Piece::Queen,
            ],
        )
        .filter_moves(game),
        7 => CantCapture::new(
            vec![Piece::Bishop],
            vec![
                Piece::Pawn,
                Piece::Knight,
                Piece::Bishop,
                Piece::Rook,
                Piece::Queen,
            ],
        )
        .filter_moves(game),
        6 => CantCapture::new(
            vec![Piece::Rook],
            vec![
                Piece::Pawn,
                Piece::Knight,
                Piece::Bishop,
                Piece::Rook,
                Piece::Queen,
            ],
        )
        .filter_moves(game),
        5 => CantCapture::new(
            vec![Piece::Queen],
            vec![
                Piece::Pawn,
                Piece::Knight,
                Piece::Bishop,
                Piece::Rook,
                Piece::Queen,
            ],
        )
        .filter_moves(game),
        4 => CantCapture::new(
            vec![Piece::King],
            vec![
                Piece::Pawn,
                Piece::Knight,
                Piece::Bishop,
                Piece::Rook,
                Piece::Queen,
            ],
        )
        .filter_moves(game),
        3 => CantCapture::new(
            vec![
                Piece::Knight,
                Piece::Bishop,
                Piece::Rook,
                Piece::Queen,
                Piece::Pawn,
                Piece::King,
            ],
            vec![Piece::Pawn],
        )
        .filter_moves(game),
        2 => CantCapture::new(
            vec![
                Piece::Knight,
                Piece::Bishop,
                Piece::Rook,
                Piece::Queen,
                Piece::Pawn,
            ],
            vec![Piece::Pawn],
        )
        .filter_moves(game),
        1 => CantCapture::new(
            vec![Piece::Knight, Piece::Bishop, Piece::Rook, Piece::Queen],
            vec![Piece::Pawn],
        )
        .filter_moves(game),
        _ => NoFilter::new().filter_moves(game),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chess::get_king_moves;
    use std::str::FromStr;

    #[test]
    fn no_filter_test() {
        let game = Game::new();
        assert_eq!(generate_moves(0, &game).len(), 20);
    }

    #[test]
    fn cant_capture_pawns_test() {
        let mut game = Game::new();
        game.make_move(ChessMove::new(Square::E2, Square::E4, None));
        game.make_move(ChessMove::new(Square::D7, Square::D5, None));
        assert!(!generate_moves(3, &game)
            .iter()
            .any(|m| m.get_source() == Square::E4 && m.get_dest() == Square::D5));
    }

    #[test]
    fn queen_cant_move_after_6() {
        let mut game = Game::new();
        game.make_move(ChessMove::new(Square::E2, Square::E4, None));
        game.make_move(ChessMove::new(Square::E7, Square::E5, None));
        game.make_move(ChessMove::new(Square::G1, Square::F3, None));
        game.make_move(ChessMove::new(Square::G8, Square::F6, None));
        game.make_move(ChessMove::new(Square::F3, Square::G1, None));
        game.make_move(ChessMove::new(Square::F6, Square::G8, None));
        game.make_move(ChessMove::new(Square::G1, Square::F3, None));
        game.make_move(ChessMove::new(Square::G8, Square::F6, None));
        game.make_move(ChessMove::new(Square::F3, Square::G1, None));
        game.make_move(ChessMove::new(Square::F6, Square::G8, None));
        assert!(generate_moves(24, &game)
            .iter()
            .any(|m| m.get_source() == Square::D1 && m.get_dest() == Square::H5)); // move 5 can move
        game.make_move(ChessMove::new(Square::G1, Square::F3, None));
        game.make_move(ChessMove::new(Square::G8, Square::F6, None));
        game.make_move(ChessMove::new(Square::F3, Square::G1, None));
        game.make_move(ChessMove::new(Square::F6, Square::G8, None));
        assert!(!generate_moves(24, &game)
            .iter()
            .any(|m| m.get_source() == Square::D1 && m.get_dest() == Square::H5)); // move 7 can't
        assert!(generate_moves(23, &game)
            .iter()
            .any(|m| m.get_source() == Square::D1 && m.get_dest() == Square::H5));
        // Move is there with other rule
    }

    #[test]
    fn queen_cant_move_to_light() {
        let game = Game::from_str("7k/8/8/8/8/5Q2/8/K7 w - - 0 1").unwrap();
        assert!(generate_moves(37, &game)
            .iter()
            .any(|m| m.get_source() == Square::F3 && m.get_dest() == Square::E3));
        assert!(!generate_moves(37, &game)
            .iter()
            .any(|m| m.get_source() == Square::F3 && m.get_dest() == Square::E2));
    }
}
