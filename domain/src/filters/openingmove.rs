use crate::movefilter::MoveFilter;
use chess::{ChessMove, Game};

pub struct OpeningMove {
    moves: Vec<ChessMove>,
}

impl OpeningMove {
    pub fn new(moves: Vec<ChessMove>) -> OpeningMove {
        OpeningMove { moves }
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
    fn force_kings_pawn() {
        let mut game = Game::new();
        let kings_pawns_filter = OpeningMove::new(vec![
            ChessMove::new(Square::E2, Square::E4, None),
            ChessMove::new(Square::E7, Square::E5, None),
            ChessMove::new(Square::G1, Square::F3, None),
        ]);
        assert_eq!(1, kings_pawns_filter.filter_moves(&game).len());
        game.make_move(ChessMove::new(Square::E2, Square::E4, None));
        assert_eq!(1, kings_pawns_filter.filter_moves(&game).len());
    }
}
