use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GameInfo {
    pub white: String,
    pub black: String,
    pub white_elo: i32,
    pub black_elo: i32,
    pub white_stealo: i32,
    pub black_stealo: i32,
}
