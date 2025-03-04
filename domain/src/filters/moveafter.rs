use crate::movefilter::MoveFilter;
use chess::{Action, ChessMove, Game, Piece};

pub struct MoveAfter {
    piece: Piece,
    turn: u16,
}

impl MoveAfter {
    pub fn new(piece: Piece, turn: u16) -> MoveAfter {
        MoveAfter { piece, turn }
    }
}

impl MoveFilter for MoveAfter {
    fn filter(&self, game: &Game, chess_move: &ChessMove) -> bool {
        let board = &game.current_position();
        if board.piece_on(chess_move.get_source()).unwrap() != self.piece {
            return false;
        }
        let mut counter: u16 = 2;
        for action in game.actions() {
            match action {
                Action::MakeMove(_) => counter += 1,
                _ => {}
            }
        }
        let turn = counter / 2;
        return turn > self.turn;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::filters::moveafter::MoveAfter;
    use chess::{ChessMove, Square};

    #[test]
    fn knight_after_turn_two() {
        let mut game = Game::new();
        game.make_move(ChessMove::new(Square::B1, Square::C3, None));
        game.make_move(ChessMove::new(Square::B8, Square::C6, None));
        game.make_move(ChessMove::new(Square::C3, Square::B1, None));
        game.make_move(ChessMove::new(Square::C6, Square::B8, None));
        let no_knight_after_two = MoveAfter::new(Piece::Knight, 2);
        assert_eq!(16, no_knight_after_two.filter_moves(&game).len());
    }
}
