use crate::filters::movefilter::MoveFilter;
use chess::{ChessMove, Game, Piece, Square};
use std::iter::Iterator;

// Prohibited squares are passed as numbers 0-63 to work with ranges
pub struct MoveTo {
    square_indexes: Vec<u8>,
    pieces: Vec<Piece>,
}

impl MoveTo {

    pub fn rooks_can_only_move_to_the_edges() -> Self {
        Self {
            square_indexes: (0..64)
                .filter(|&x| x % 8 != 0 && (x + 1) % 8 != 0 && x > 7 && x < 56)
                .collect(),
            pieces: vec![Piece::Rook],
        }
    }

    pub fn king_can_only_move_to_dark_squares() -> Self {
        Self {
            square_indexes: (0..32).map(|x| 1 + x * 2 + (x % 8) / 4).collect(),
            pieces: vec![Piece::King],
        }
    }

    pub fn king_can_only_move_to_light_squares() -> Self {
        Self {
            square_indexes: (0..32).map(|x| x * 2 + (x % 8) / 4).collect(),
            pieces: vec![Piece::King],
        }
    }

    pub fn queen_can_only_move_to_dark_squares() -> Self {
        Self {
            square_indexes: (0..32).map(|x| 1 + x * 2 + (x % 8) / 4).collect(),
            pieces: vec![Piece::Queen],
        }
    }

    pub fn queen_can_only_move_to_light_squares() -> Self {
        Self {
            square_indexes: (0..32).map(|x| x * 2 + (x % 8) / 4).collect(),
            pieces: vec![Piece::Queen],
        }
    }

    pub fn cant_play_on_the_c_file() -> Self {
        Self {
            square_indexes: (0..8).map(|x| x * 8 + 2).collect(),
            pieces: vec![
                Piece::Pawn,
                Piece::Knight,
                Piece::Bishop,
                Piece::Rook,
                Piece::Queen,
                Piece::King,
            ],
        }
    }

    pub fn cant_play_on_the_h_file() -> Self {
        Self {
            square_indexes: (0..8).map(|x| x * 8 + 7).collect(),
            pieces: vec![
                Piece::Pawn,
                Piece::Knight,
                Piece::Bishop,
                Piece::Rook,
                Piece::Queen,
                Piece::King,
            ],
        }
    }

    pub fn cant_play_on_the_e_file() -> Self {
        Self {
            square_indexes: (0..8).map(|x| x * 8 + 4).collect(),
            pieces: vec![
                Piece::Pawn,
                Piece::Knight,
                Piece::Bishop,
                Piece::Rook,
                Piece::Queen,
                Piece::King,
            ],
        }
    }

    pub fn cant_play_on_the_e_or_h_file() -> Self {
        Self {
            square_indexes: (0..63).filter(|&x| x % 8 == 4 || x % 8 == 7).collect(),
            pieces: vec![
                Piece::Pawn,
                Piece::Knight,
                Piece::Bishop,
                Piece::Rook,
                Piece::Queen,
                Piece::King,
            ],
        }
    }

    pub fn cant_play_on_the_c_or_e_file() -> Self {
        Self {
            square_indexes: (0..63).filter(|&x| x % 8 == 2 || x % 8 == 4).collect(),
            pieces: vec![
                Piece::Pawn,
                Piece::Knight,
                Piece::Bishop,
                Piece::Rook,
                Piece::Queen,
                Piece::King,
            ],
        }
    }

    pub fn cant_play_on_the_c_or_h_file() -> Self {
        Self {
            square_indexes: (0..63).filter(|&x| x % 8 == 2 || x % 8 == 7).collect(),
            pieces: vec![
                Piece::Pawn,
                Piece::Knight,
                Piece::Bishop,
                Piece::Rook,
                Piece::Queen,
                Piece::King,
            ],
        }
    }

    pub fn cant_play_on_the_c_or_e_or_h_file() -> Self {
        Self {
            square_indexes: (0..63)
                .filter(|&x| x % 8 == 4 || x % 8 == 7 || x % 8 == 2)
                .collect(),
            pieces: vec![
                Piece::Pawn,
                Piece::Knight,
                Piece::Bishop,
                Piece::Rook,
                Piece::Queen,
                Piece::King,
            ],
        }
    }
}

impl MoveFilter for MoveTo {
    fn filter(&self, game: &Game, chess_move: &ChessMove) -> bool {
        let board = &game.current_position();
        let squares: Vec<Square> = unsafe {
            self.square_indexes
                .iter()
                .map(|&x| Square::new(x))
                .collect()
        };
        if !self
            .pieces
            .contains(&board.piece_on(chess_move.get_source()).unwrap())
        {
            return false;
        }
        if squares.contains(&chess_move.get_dest()) {
            return true;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forbidden_a_pawn() {
        let game = Game::new();
        let c_filter = MoveTo::cant_play_on_the_c_file();
        assert_eq!(17, c_filter.filter_moves(&game).len());
    }
}