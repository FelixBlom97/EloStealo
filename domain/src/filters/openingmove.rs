use crate::filters::movefilter::MoveFilter;
use chess::{ChessMove, Game, Square};

pub struct OpeningMove {
    moves: Vec<ChessMove>,
}

impl OpeningMove {
    pub fn two_g_pawn_moves() -> Self {
        Self {
            moves: vec![
            ChessMove::new(Square::G2, Square::G3, None),
            ChessMove::new(Square::G7, Square::G6, None),
            ChessMove::new(Square::G3, Square::G4, None),
            ChessMove::new(Square::G6, Square::G5, None),]
        }
    }

    pub fn knight_and_back() -> Self {
        Self {
            moves: vec![
                ChessMove::new(Square::B1, Square::C3, None),
                ChessMove::new(Square::B8, Square::C6, None),
                ChessMove::new(Square::C3, Square::B1, None),
                ChessMove::new(Square::C6, Square::B8, None),
            ]
        }
    }

    pub fn g_and_f_pawn() -> Self {
        Self {
            moves: vec![
                ChessMove::new(Square::D2, Square::D3, None),
                ChessMove::new(Square::D7, Square::D6, None),
                ChessMove::new(Square::F2, Square::F3, None),
                ChessMove::new(Square::F7, Square::F6, None),
            ]
        }
    }

    pub fn edge_pawns_two_squares() -> Self {
        Self {
            moves: vec![
                ChessMove::new(Square::A2, Square::A4, None),
                ChessMove::new(Square::A7, Square::A5, None),
                ChessMove::new(Square::H2, Square::H4, None),
                ChessMove::new(Square::H7, Square::H5, None),
            ]
        }
    }

    pub fn the_cheese_opening() -> Self {
        Self {
            moves: vec![
                ChessMove::new(Square::C2, Square::C4, None),
                ChessMove::new(Square::C7, Square::C5, None),
                ChessMove::new(Square::D2, Square::D3, None),
                ChessMove::new(Square::D7, Square::D6, None),
                ChessMove::new(Square::E2, Square::E4, None),
                ChessMove::new(Square::E7, Square::E5, None),
            ]
        }
    }

    pub fn rush_a() -> Self {
        Self {
            moves: vec![
                ChessMove::new(Square::A2, Square::A4, None),
                ChessMove::new(Square::A7, Square::A5, None),
                ChessMove::new(Square::A4, Square::A5, None),
                ChessMove::new(Square::A5, Square::A4, None),
                ]
        }
    }

    pub fn rush_b() -> Self {
        Self {
            moves: vec![
                ChessMove::new(Square::B2, Square::B4, None),
                ChessMove::new(Square::B7, Square::B5, None),
                ChessMove::new(Square::B4, Square::B5, None),
                ChessMove::new(Square::B5, Square::B4, None),
            ]
        }
    }

    pub fn scholars_mate() -> Self {
        Self {
            moves: vec![
                ChessMove::new(Square::E2, Square::E4, None),
                ChessMove::new(Square::E7, Square::E5, None),
                ChessMove::new(Square::F1, Square::C4, None),
                ChessMove::new(Square::F8, Square::C5, None),
                ChessMove::new(Square::D1, Square::H5, None),
                ChessMove::new(Square::D8, Square::H4, None),
            ]
        }
    }

    pub fn move_f_pawn_twice() -> Self {
        Self {
            moves: vec![
                ChessMove::new(Square::F2, Square::F3, None),
                ChessMove::new(Square::F7, Square::F6, None),
                ChessMove::new(Square::F3, Square::F4, None),
                ChessMove::new(Square::F6, Square::F5, None),
            ]
        }
    }

    pub fn bring_both_rooks_out() -> Self {
        Self {
            moves:vec![
                ChessMove::new(Square::A2, Square::A4, None),
                ChessMove::new(Square::A7, Square::A5, None),
                ChessMove::new(Square::A1, Square::A3, None),
                ChessMove::new(Square::A8, Square::A6, None),
                ChessMove::new(Square::H2, Square::H4, None),
                ChessMove::new(Square::H7, Square::H5, None),
                ChessMove::new(Square::H1, Square::H3, None),
                ChessMove::new(Square::H8, Square::H6, None),
            ]
        }
    }

    pub fn allow_fools_mate() -> Self {
        Self {
            moves:vec![
                ChessMove::new(Square::F2, Square::F3, None),
                ChessMove::new(Square::F7, Square::F6, None),
                ChessMove::new(Square::G2, Square::G4, None),
                ChessMove::new(Square::G7, Square::G5, None),
            ]
        }
    }

    pub fn bongcloud() -> Self {
        Self {
            moves: vec![
                ChessMove::new(Square::E2, Square::E4, None),
                ChessMove::new(Square::E7, Square::E5, None),
                ChessMove::new(Square::E1, Square::E2, None),
                ChessMove::new(Square::E8, Square::E7, None),
            ]
        }
    }

    pub fn bongcloud_and_back() -> Self {
        Self {
            moves: vec![
                ChessMove::new(Square::E2, Square::E4, None),
                ChessMove::new(Square::E7, Square::E5, None),
                ChessMove::new(Square::E1, Square::E2, None),
                ChessMove::new(Square::E8, Square::E7, None),
                ChessMove::new(Square::E2, Square::E1, None),
                ChessMove::new(Square::E7, Square::E8, None),
            ]
        }
    }

    pub fn knights_to_the_edges() -> Self {
        Self {
            moves: vec![
                ChessMove::new(Square::B1, Square::A3, None),
                ChessMove::new(Square::B8, Square::A6, None),
                ChessMove::new(Square::G1, Square::H3, None),
                ChessMove::new(Square::G8, Square::H6, None),
            ]
        }
    }
}

impl MoveFilter for OpeningMove {
    fn filter(&self, game: &Game, chess_move: &ChessMove) -> bool {
        let turn = game.actions().len();
        match self.moves.get(turn) {
            Some(x) => !(x == chess_move),
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chess::Square;

    #[test]
    fn force_bong_cloud() {
        let mut game = Game::new();
        let bongcloud = OpeningMove::bongcloud();
        assert_eq!(1, bongcloud.filter_moves(&game).len());
        game.make_move(ChessMove::new(Square::E2, Square::E4, None));
        assert_eq!(1, bongcloud.filter_moves(&game).len());
    }
}
