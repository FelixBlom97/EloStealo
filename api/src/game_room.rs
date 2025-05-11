use tokio::sync::broadcast;
use tokio::sync::broadcast::Sender;
use domain::chessgame::ChessGame;

pub struct GameRoom {
    pub tx: Sender<String>,
    game: Option<ChessGame>
}

impl GameRoom {
    pub fn new(_id: String) -> Self {
        let (tx, _rx) = broadcast::channel(100);
        GameRoom {tx, game: None}
    }
}