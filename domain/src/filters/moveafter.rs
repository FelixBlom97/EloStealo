use crate::filters::movefilter::MoveFilter;
use chess::{Action, ChessMove, Game, Piece};

pub struct MoveAfter {
    piece: Piece,
    turn: u16,
}

impl MoveAfter {
    pub fn queen_cant_move_after_12() -> Self {
        Self {
            piece: Piece::Queen,
            turn: 12,
        }
    }

    pub fn queen_cant_move_after_9() -> Self {
        Self {
            piece: Piece::Queen,
            turn: 9,
        }
    }

    pub fn queen_cant_move_after_6() -> Self {
        Self {
            piece: Piece::Queen,
            turn: 6,
        }
    }

    pub fn rook_cant_move_after_25() -> Self {
        Self {
            piece: Piece::Rook,
            turn: 25,
        }
    }

    pub fn rook_cant_move_after_20() -> Self {
        Self {
            piece: Piece::Rook,
            turn: 20,
        }
    }

    pub fn rook_cant_move_after_15() -> Self {
        Self {
            piece: Piece::Rook,
            turn: 15,
        }
    }

    pub fn bishop_cant_move_after_10() -> Self {
        Self {
            piece: Piece::Bishop,
            turn: 10,
        }
    }

    pub fn bishop_cant_move_after_15() -> Self {
        Self {
            piece: Piece::Bishop,
            turn: 15,
        }
    }

    pub fn bishop_cant_move_after_20() -> Self {
        Self {
            piece: Piece::Bishop,
            turn: 20,
        }
    }

    pub fn knight_cant_move_after_10() -> Self {
        Self {
            piece: Piece::Knight,
            turn: 10,
        }
    }

    pub fn knight_cant_move_after_15() -> Self {
        Self {
            piece: Piece::Knight,
            turn: 15,
        }
    }

    pub fn knight_cant_move_after_20() -> Self {
        Self {
            piece: Piece::Knight,
            turn: 20,
        }
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
        turn > self.turn
    }
}

#[cfg(test)]
mod tests {
}
