use crate::filters::movefilter::MoveFilter;
use chess::{ChessMove, Game, Piece};

pub struct CantCapture {
    source: Vec<Piece>,
    target: Vec<Piece>,
}

impl CantCapture {
    pub fn only_pawns_or_king_can_capture_pawns() -> Self {
        Self {
            source: vec![Piece::Knight, Piece::Bishop, Piece::Rook, Piece::Queen],
            target: vec![Piece::Pawn],
        }
    }

    pub fn only_king_can_capture_pawns() -> Self {
        Self {
            source: vec![
                Piece::Knight,
                Piece::Bishop,
                Piece::Rook,
                Piece::Queen,
                Piece::Pawn,
            ],
            target: vec![Piece::Pawn],
        }
    }

    pub fn pawns_cant_be_captured() -> Self {
        Self {
            source: vec![
                Piece::Knight,
                Piece::Bishop,
                Piece::Rook,
                Piece::Queen,
                Piece::Pawn,
                Piece::King,
            ],
            target: vec![Piece::Pawn],
        }
    }

    pub fn king_cant_capture_anything() -> Self {
        Self {
            source: vec![Piece::King],
            target: vec![
                Piece::Pawn,
                Piece::Knight,
                Piece::Bishop,
                Piece::Rook,
                Piece::Queen,
            ],
        }
    }

    pub fn queen_cant_capture_anything() -> Self {
        Self {
            source: vec![Piece::Queen],
            target: vec![
                Piece::Pawn,
                Piece::Knight,
                Piece::Bishop,
                Piece::Rook,
                Piece::Queen,
            ],
        }
    }

    pub fn rooks_cant_capture_anything() -> Self {
        Self {
            source: vec![Piece::Rook],
            target: vec![
                Piece::Pawn,
                Piece::Knight,
                Piece::Bishop,
                Piece::Rook,
                Piece::Queen,
            ],
        }
    }

    pub fn bishops_cant_capture_anything() -> Self {
        Self {
            source: vec![Piece::Bishop],
            target: vec![
                Piece::Pawn,
                Piece::Knight,
                Piece::Bishop,
                Piece::Rook,
                Piece::Queen,
            ],
        }
    }

    pub fn knights_cant_capture_anything() -> Self {
        Self {
            source: vec![Piece::Knight],
            target: vec![
                Piece::Pawn,
                Piece::Knight,
                Piece::Bishop,
                Piece::Rook,
                Piece::Queen,
            ],
        }
    }

    pub fn knights_cant_capture_queens() -> Self {
        Self {
            source: vec![Piece::Knight],
            target: vec![Piece::Queen],
        }
    }

    pub fn bishops_cant_capture_queens() -> Self {
        Self {
            source: vec![Piece::Bishop],
            target: vec![Piece::Queen],
        }
    }

    pub fn rooks_cant_capture_queens() -> Self {
        Self {
            source: vec![Piece::Rook],
            target: vec![Piece::Queen],
        }
    }

    pub fn queen_can_only_capture_pawns() -> Self {
        Self {
            source: vec![Piece::Queen],
            target: vec![Piece::Knight, Piece::Bishop, Piece::Rook, Piece::Queen],
        }
    }

    pub fn queen_can_only_capture_rooks() -> Self {
        Self {
            source: vec![Piece::Queen],
            target: vec![Piece::Pawn, Piece::Knight, Piece::Bishop, Piece::Queen],
        }
    }

    pub fn queen_can_only_capture_knights() -> Self {
        Self {
            source: vec![Piece::Queen],
            target: vec![Piece::Pawn, Piece::Knight, Piece::Bishop, Piece::Queen],
        }
    }

    pub fn queen_can_only_capture_bishops() -> Self {
        Self {
            source: vec![Piece::Queen],
            target: vec![Piece::Pawn, Piece::Knight, Piece::Rook, Piece::Queen],
        }
    }

    pub fn only_pawns_can_capture_pawns() -> Self {
        Self {
            source: vec![
                Piece::Knight,
                Piece::Bishop,
                Piece::Rook,
                Piece::Queen,
                Piece::King,
            ],
            target: vec![Piece::Pawn],
        }
    }

    pub fn pawns_can_only_capture_pawns() -> Self {
        Self {
            source: vec![Piece::Pawn],
            target: vec![Piece::Knight, Piece::Bishop, Piece::Rook, Piece::Queen],
        }
    }

    pub fn pawns_cant_capture_pawns() -> Self {
        Self {
            source: vec![Piece::Pawn],
            target: vec![Piece::Pawn],
        }
    }

    pub fn cant_capture_rooks() -> Self {
        Self {
            source: vec![
                Piece::Pawn,
                Piece::Knight,
                Piece::Bishop,
                Piece::Rook,
                Piece::Queen,
                Piece::King,
            ],
            target: vec![Piece::Rook],
        }
    }

    pub fn cant_capture_bishops() -> Self {
        Self {
            source: vec![
                Piece::Pawn,
                Piece::Knight,
                Piece::Bishop,
                Piece::Rook,
                Piece::Queen,
                Piece::King,
            ],
            target: vec![Piece::Bishop],
        }
    }

    pub fn cant_capture_knights() -> Self {
        Self {
            source: vec![
                Piece::Pawn,
                Piece::Knight,
                Piece::Bishop,
                Piece::Rook,
                Piece::Queen,
                Piece::King,
            ],
            target: vec![Piece::Knight],
        }
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
        let pp_filter = CantCapture::pawns_cant_capture_pawns();
        assert_eq!(30, pp_filter.filter_moves(&game).len());
    }

    #[test]
    // White king on a1, black queen on a2, black king on h2
    fn king_cant_capture_queen() {
        let game = Game::from_str("8/8/8/8/8/8/q6k/K7 w - - 0 1").expect("wrong");
        let king_filter = CantCapture::king_cant_capture_anything();
        assert_eq!(0, king_filter.filter_moves(&game).len());
    }
}
