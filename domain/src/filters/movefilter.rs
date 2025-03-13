use chess::{ChessMove, Game, MoveGen};

// Generic trait for EloStealo rules that restrict the moves you can make.
// Implementations provide a filter function that determines if a move can be made.
// The trait provides filter_moves which applies the filter to every allowed move in regular chess,
// and returns a vector of the moves that are allowed.
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
