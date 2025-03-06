use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct StealoRule {
    pub id: i32,
    pub name: String,
    pub elo: i32,
    pub description: String,
}