use chess::{ChessMove, Error, Piece, Square};
use std::str::FromStr;

pub fn string_to_move(str_move: String) -> ChessMove {
    let error = Error::InvalidSanMove;
    let str_source = str_move.get(0..2).ok_or(error.clone());
    let str_dest = str_move.get(2..4).ok_or(error.clone());
    let source = Square::from_str(str_source.unwrap());
    let dest = Square::from_str(str_dest.unwrap());
    ChessMove::new(
        source.unwrap(),
        dest.unwrap(),
        get_promotion_piece(str_move),
    )
}

fn get_promotion_piece(str_move: String) -> Option<Piece> {
    if str_move.len() <= 4 {
        None
    } else {
        let piece = str_move.chars().nth(4).unwrap();
        match piece {
            'n' => Some(Piece::Knight),
            'b' => Some(Piece::Bishop),
            'r' => Some(Piece::Rook),
            'q' => Some(Piece::Queen),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_move() {
        let e2e4 = string_to_move("e2e4".to_string());
        assert_eq!(e2e4, ChessMove::new(Square::E2, Square::E4, None));
    }

    #[test]
    fn promote() {
        let e7e8q = string_to_move("e7e8q".to_string());
        assert_eq!(
            e7e8q,
            ChessMove::new(Square::E7, Square::E8, Some(Piece::Queen))
        );
    }
}
