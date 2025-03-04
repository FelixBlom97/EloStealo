use crate::movefilter::MoveFilter;
use chess::{ChessMove, Game, Piece};

pub struct CantCapture {
    source: Vec<Piece>,
    target: Vec<Piece>,
}

impl CantCapture {
    pub fn new(source: Vec<Piece>, target: Vec<Piece>) -> CantCapture {
        CantCapture { source, target }
    }
}

impl MoveFilter for CantCapture {
    fn filter(&self, game: &Game, chess_move: &ChessMove) -> bool {
        let board = &game.current_position();
        if self
            .source
            .contains(&board.piece_on(chess_move.get_source()).unwrap())
        {
            match board.piece_on(chess_move.get_dest()) {
                Some(piece) => self.target.contains(&piece),
                None => false,
            }
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chess::{Game, Square};
    use std::str::FromStr;

    #[test]
    // Without filters, there are 31 moves for white after 1. e2e4 d7d5
    fn pawn_cant_capture_pawn() {
        let mut game = Game::new();
        game.make_move(ChessMove::new(Square::E2, Square::E4, None));
        game.make_move(ChessMove::new(Square::D7, Square::D5, None));
        let pp_filter = CantCapture::new(vec![Piece::Pawn], vec![Piece::Pawn]);
        assert_eq!(30, pp_filter.filter_moves(&game).len());
    }

    #[test]
    // White king on a1, black queen on a2, black king on h2
    fn king_cant_capture_queen() {
        let game = Game::from_str("8/8/8/8/8/8/q6k/K7 w - - 0 1").expect("wrong");
        let kq_filter = CantCapture::new(vec![Piece::King], vec![Piece::Queen]);
        assert_eq!(0, kq_filter.filter_moves(&game).len());
    }

    #[test]
    // Without filter, the king should be able to capture the bishop on D2 or the knight on E2
    fn can_only_capture_rook() {
        let game = Game::from_str("k7/8/8/8/4r3/3Kn3/2pbn3/8 w - - 0 1").expect("no");
        let all_but_rook_filter = CantCapture::new(
            vec![Piece::King],
            vec![Piece::Knight, Piece::Pawn, Piece::Queen, Piece::Bishop],
        );
        assert_eq!(1, all_but_rook_filter.filter_moves(&game).len());
    }
}
