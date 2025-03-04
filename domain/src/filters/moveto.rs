use crate::movefilter::MoveFilter;
use chess::{ChessMove, Game, Piece, Square};
use std::iter::Iterator;

// Prohibited squares are passed as numbers 0-63 to work with ranges
pub struct MoveTo {
    square_numbers: Vec<u8>,
    piece: Vec<Piece>,
}

impl MoveTo {
    pub fn new(square_numbers: Vec<u8>, piece: Vec<Piece>) -> MoveTo {
        MoveTo {
            square_numbers,
            piece,
        }
    }
}

impl MoveFilter for MoveTo {
    fn filter(&self, game: &Game, chess_move: &ChessMove) -> bool {
        let board = &game.current_position();
        let squares: Vec<Square> = unsafe {
            self.square_numbers
                .iter()
                .map(|&x| Square::new(x))
                .collect()
        };
        if !self
            .piece
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
        let a_filter = MoveTo::new((24..32).collect(), vec![Piece::Pawn]);
        assert_eq!(12, a_filter.filter_moves(&game).len());
    }
}
