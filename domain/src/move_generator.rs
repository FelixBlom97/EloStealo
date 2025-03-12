use crate::filters::cantcapture::CantCapture;
use crate::filters::moveafter::MoveAfter;
use crate::filters::movefilter::MoveFilter;
use crate::filters::moveto::MoveTo;
use crate::filters::nofilter::NoFilter;
use crate::filters::openingmove::OpeningMove;
use chess::{ChessMove, Game, Piece, Square};

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
        45 => MoveTo::cant_play_on_the_c_or_e_or_h_file().filter_moves(game),
        44 => MoveTo::cant_play_on_the_c_or_h_file().filter_moves(game),
        43 => MoveTo::cant_play_on_the_c_or_e_file().filter_moves(game),
        42 => MoveTo::cant_play_on_the_e_or_h_file().filter_moves(game),
        41 => MoveTo::cant_play_on_the_e_file().filter_moves(game),
        40 => MoveTo::cant_play_on_the_h_file().filter_moves(game),
        39 => MoveTo::cant_play_on_the_c_file().filter_moves(game),
        38 => MoveTo::queen_can_only_move_to_light_squares().filter_moves(game),
        37 => MoveTo::queen_can_only_move_to_dark_squares().filter_moves(game),
        36 => MoveTo::king_can_only_move_to_light_squares().filter_moves(game),
        35 => MoveTo::king_can_only_move_to_dark_squares().filter_moves(game),
        34 => MoveTo::rooks_can_only_move_to_the_edges().filter_moves(game),
        33 => MoveAfter::knight_cant_move_after_10().filter_moves(game),
        32 => MoveAfter::knight_cant_move_after_15().filter_moves(game),
        31 => MoveAfter::knight_cant_move_after_20().filter_moves(game),
        30 => MoveAfter::bishop_cant_move_after_10().filter_moves(game),
        29 => MoveAfter::bishop_cant_move_after_15().filter_moves(game),
        28 => MoveAfter::bishop_cant_move_after_20().filter_moves(game),
        27 => MoveAfter::rook_cant_move_after_15().filter_moves(game),
        26 => MoveAfter::rook_cant_move_after_20().filter_moves(game),
        25 => MoveAfter::rook_cant_move_after_25().filter_moves(game),
        24 => MoveAfter::queen_cant_move_after_6().filter_moves(game),
        23 => MoveAfter::queen_cant_move_after_9().filter_moves(game),
        22 => MoveAfter::queen_cant_move_after_12().filter_moves(game),
        21 => CantCapture::cant_capture_knights().filter_moves(game),
        20 => CantCapture::cant_capture_bishops().filter_moves(game),
        19 => CantCapture::cant_capture_rooks().filter_moves(game),
        18 => CantCapture::pawns_cant_capture_pawns().filter_moves(game),
        17 => CantCapture::pawns_can_only_capture_pawns().filter_moves(game),
        16 => CantCapture::only_pawns_can_capture_pawns().filter_moves(game),
        15 => CantCapture::queen_can_only_capture_bishops().filter_moves(game),
        14 => CantCapture::queen_can_only_capture_knights().filter_moves(game),
        13 => CantCapture::queen_can_only_capture_rooks().filter_moves(game),
        12 => CantCapture::queen_can_only_capture_pawns().filter_moves(game),
        11 => CantCapture::rooks_cant_capture_queens().filter_moves(game),
        10 => CantCapture::bishops_cant_capture_queens().filter_moves(game),
        9 => CantCapture::knights_cant_capture_queens().filter_moves(game),
        8 => CantCapture::knights_cant_capture_anything().filter_moves(game),
        7 => CantCapture::bishops_cant_capture_anything().filter_moves(game),
        6 => CantCapture::rooks_cant_capture_anything().filter_moves(game),
        5 => CantCapture::queen_cant_capture_anything().filter_moves(game),
        4 => CantCapture::king_cant_capture_anything().filter_moves(game),
        3 => CantCapture::pawns_cant_be_captured().filter_moves(game),
        2 => CantCapture::only_king_can_capture_pawns().filter_moves(game),
        1 => CantCapture::only_pawns_or_king_can_capture_pawns().filter_moves(game),
        _ => NoFilter::new().filter_moves(game),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
