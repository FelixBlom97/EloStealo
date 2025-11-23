#[allow(non_snake_case)]
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct GameDTO {
    board: String,
    moves: Vec<String>,
    result: String,
}
#[derive(Serialize, Deserialize)]
pub struct WaitingPlayer {
    pub room: String,
    pub name: String,
    pub elo: String,
    pub stealo: i32,
}

#[derive(Serialize)]
pub struct GameInfoLocal {
    pub white: String,
    pub black: String,
    pub white_elo: i32,
    pub black_elo: i32,
    pub white_stealo: i32,
    pub black_stealo: i32,
}