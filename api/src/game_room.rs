use std::sync::Arc;
use tokio::sync::{broadcast, Mutex};
use tokio::sync::broadcast::Sender;
// use domain::chessgame::ChessGame;

#[derive(Clone)]
pub struct GameRoom {
    pub tx: Sender<String>,
    game: Arc<Mutex<String>>
}

impl GameRoom {
    pub fn new(_id: String) -> Self {
        let (tx, _rx) = broadcast::channel(100);
        GameRoom {tx, game: Arc::new(Mutex::new("".to_owned()))}
    }

    pub async fn make_move(&self, add: &str) {
        let mut game_guard = self.game.lock().await;
        *game_guard += add;
        let game_clone = game_guard.clone();
        drop(game_guard);
        let _ = self.tx.send(game_clone);
    }

    pub async fn get_state_copy(&self) -> String {
        let game_guard = self.game.lock().await;
        game_guard.clone()
    }
}