use serde::Deserialize;

#[derive(Deserialize)]
pub struct NewLocalGameDTO {
    pub player1: String,
    pub player2: String,
    pub elo1: i32,
    pub elo2: i32,
    pub stealo1: i32,
    pub stealo2: i32,
}