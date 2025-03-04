use crate::movefilter::generate_moves;
use crate::stringtomove::string_to_move;
use chess::GameResult::{BlackResigns, Stalemate, WhiteResigns};
use chess::{Action, Board, ChessMove, Color, Game, MoveGen};

pub struct ChessGame {
    pub white: String,
    pub black: String,
    pub game: Game,
    pub elo_white: i32,
    pub elo_black: i32,
    pub filter_id_white: i32,
    pub filter_id_black: i32,
}

impl ChessGame {
    pub fn get_move_gen(&self) -> MoveGen {
        let board = self.game.current_position();
        let moves = MoveGen::new_legal(&board);
        moves
    }

    pub fn get_position(&self) -> Board {
        self.game.current_position()
    }

    pub fn make_move(&mut self, move_to_make: String, color: Option<String>) {
        let side_to_move = match color.as_deref() {
            Some("white") => Some(Color::White),
            Some("black") => Some(Color::Black),
            _ => None,
        };
        if move_to_make == "resign" && side_to_move.is_some() {
            self.game.resign(side_to_move.unwrap());
        } else {
            let chess_move = string_to_move(move_to_make);
            if self.get_moves().contains(&chess_move) {
                self.game.make_move(chess_move);
            }
        }
    }

    pub fn get_moves(&self) -> Vec<ChessMove> {
        if self.game.result().is_some() {
            Vec::new()
        } else if self.game.side_to_move() == Color::White {
            generate_moves(self.filter_id_white, &self.game)
        } else {
            generate_moves(self.filter_id_black, &self.game)
        }
    }

    pub fn get_moves_string(&self) -> Vec<String> {
        let moves = self.get_moves();
        moves.iter().map(|m| m.to_string()).collect()
    }

    pub fn turn(&self) -> u16 {
        let mut counter: u16 = 2;
        for action in self.game.actions() {
            match action {
                Action::MakeMove(_) => counter += 1,
                _ => {}
            }
        }
        counter / 2
    }

    // Only called when there are no available moves after filtering.
    // If you can't move due to stealo rule, you lose. Stalemate is still a draw.
    pub fn winner_when_no_moves(&self) -> String {
        let result = self.game.result();
        match (result, self.game.side_to_move()) {
            (Some(Stalemate), _) => "draw".to_string(),
            (Some(WhiteResigns), _) => "black".to_string(),
            (Some(BlackResigns), _) => "white".to_string(),
            (_, Color::White) => "black".to_string(),
            (_, Color::Black) => "white".to_string(),
        }
    }
}

pub fn new_game(
    player1: String,
    player2: String,
    elo1: i32,
    elo2: i32,
    stealo1: i32,
    stealo2: i32,
) -> ChessGame {
    let g = Game::new();
    ChessGame {
        white: player1,
        black: player2,
        game: g,
        elo_white: elo1,
        elo_black: elo2,
        filter_id_white: stealo1,
        filter_id_black: stealo2,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn get_position_test() {
        let game = new_game("AtoomBlom".to_string(), "Opponent".to_string(), 0, 0, 0, 0);
        assert_eq!(game.get_position(), Board::default());
    }

    #[test]
    fn turn_number_test() {
        let mut game = new_game("AtoomBlom".to_string(), "Opponent".to_string(), 0, 0, 0, 0);
        game.make_move("e2e4".to_string(), None);
        game.make_move("e7e5".to_string(), None);
        game.make_move("e1e2".to_string(), None);
        assert_eq!(2, game.turn());
        game.make_move("e8e7".to_string(), None);
        assert_eq!(3, game.turn());
    }

    #[test]
    fn stalemate_test() {
        let mut game = new_game("AtoomBlom".to_string(), "Opponent".to_string(), 0, 0, 0, 0);
        let stalemate_position = Board::from_str("k7/8/8/8/8/8/2q5/K7 w - - 0 1").unwrap();
        game.game = Game::new_with_board(stalemate_position);
        assert_eq!("draw".to_string(), game.winner_when_no_moves());
    }

    #[test]
    fn no_moves_white_turn() {
        let game = new_game("AtoomBlom".to_string(), "Opponent".to_string(), 0, 0, 0, 0);
        assert_eq!("black".to_string(), game.winner_when_no_moves());
    }

    #[test]
    fn no_moves_black_turn() {
        let mut game = new_game("AtoomBlom".to_string(), "Opponent".to_string(), 0, 0, 0, 0);
        game.make_move("e2e4".to_string(), None);
        assert_eq!("white".to_string(), game.winner_when_no_moves());
    }

    #[test]
    fn white_resigns() {
        let mut game = new_game("AtoomBlom".to_string(), "Opponent".to_string(), 0, 0, 0, 0);
        game.make_move("resign".to_string(), Some("white".to_string()));
        assert_eq!(game.get_moves().len(), 0);
    }

    #[test]
    fn illegal_move_due_to_stealo() {
        // Stealo 59: white has to begin with Nb1-a3
        let mut game = new_game("AtoomBlom".to_string(), "Opponent".to_string(), 0, 0, 59, 0);
        game.make_move("e2e4".to_string(), None);
        assert_eq!(game.get_position(), chess::Game::new().current_position());
    }
}
