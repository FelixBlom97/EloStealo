use std::sync::Arc;
use tokio::sync::{broadcast, Mutex};
use tokio::sync::broadcast::Sender;
use uuid::Uuid;
use domain::chessgame::ChessGame;
use crate::dtos::game_dto::GameDTO;
use crate::dtos::game_info_dto::GameInfoDTO;

#[derive(Clone)]
pub struct GameRoom {
    pub tx: Sender<String>,
    game: Arc<Mutex<ChessGame>>
}

impl GameRoom {
    pub fn new(game: ChessGame) -> Self {
        let (tx, _rx) = broadcast::channel(100);
        GameRoom {tx, game: Arc::new(Mutex::new(game))}
    }

    pub async fn make_move(&self, chess_move: String, user_uuid: &Uuid) {
        let mut game_guard = self.game.lock().await;
        if let Some(turn_uuid) = game_guard.get_uuid_with_turn() {
            if user_uuid == turn_uuid {
                game_guard.make_move_new(chess_move);
            }
        }
        let game_state = GameDTO::new(&game_guard);
        drop(game_guard);
        let _ = self.tx.send(serde_json::to_string(&game_state).unwrap());
    }

    pub async fn get_game_state(&self) -> String {
        let game_guard = self.game.lock().await;
        let game_state = GameDTO::new(&game_guard);
        drop(game_guard);
        serde_json::to_string(&game_state).unwrap()
    }

    pub async fn get_game_info(&self, user_uuid: &Uuid) -> String {
        let game_guard = self.game.lock().await;
        let game_info = GameInfoDTO::new(&game_guard, user_uuid);
        drop(game_guard);
        serde_json::to_string(&game_info).unwrap()
    }

    pub fn get_game(&self) -> Arc<Mutex<ChessGame>> {
        Arc::clone(&self.game)
    }
}