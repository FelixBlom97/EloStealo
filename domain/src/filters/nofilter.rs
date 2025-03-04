use crate::movefilter::MoveFilter;
use chess::{ChessMove, Game};

pub struct NoFilter {}

impl NoFilter {
    pub fn new() -> NoFilter {
        NoFilter {}
    }
}

impl MoveFilter for NoFilter {
    fn filter(&self, _: &Game, _: &ChessMove) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chess::Game;

    #[test]
    fn new_game_test() {
        let game = Game::new();
        let no_filter = NoFilter::new();
        assert_eq!(no_filter.filter_moves(&game).len(), 20);
    }
}
