use crate::filters::cantcapture::CantCapture;
use crate::filters::moveafter::MoveAfter;
use crate::filters::movefilter::MoveFilter;
use crate::filters::moveto::MoveTo;
use crate::filters::nofilter::NoFilter;
use crate::filters::openingmove::OpeningMove;
use chess::{ChessMove, Game};

// Select filter based on id and filter the moves. Default is normal chess.
// Range 1-21 are filters that limit captures.
// Range 22-33 Prevents pieces from moving after move x
// Range 34-45 limits movement of pieces to certain squares
// Range 45-59 forces a number of opening moves
pub fn generate_moves(filter_id: i32, game: &Game) -> Vec<ChessMove> {
    match filter_id {
        59 => OpeningMove::knights_to_the_edges().filter_moves(game),
        58 => OpeningMove::bongcloud_and_back().filter_moves(game),
        57 => OpeningMove::bongcloud().filter_moves(game),
        56 => OpeningMove::allow_fools_mate().filter_moves(game),
        55 => OpeningMove::bring_both_rooks_out().filter_moves(game),
        54 => OpeningMove::move_f_pawn_twice().filter_moves(game),
        53 => OpeningMove::scholars_mate().filter_moves(game),
        52 => OpeningMove::rush_b().filter_moves(game),
        51 => OpeningMove::rush_a().filter_moves(game),
        50 => OpeningMove::the_cheese_opening().filter_moves(game),
        49 => OpeningMove::edge_pawns_two_squares().filter_moves(game),
        48 => OpeningMove::g_and_f_pawn().filter_moves(game),
        47 => OpeningMove::knight_and_back().filter_moves(game),
        46 => OpeningMove::two_g_pawn_moves().filter_moves(game),
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
