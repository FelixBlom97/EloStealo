use serde::{Deserialize, Serialize};

pub trait RulesRepository {
    async fn get_stealo_rules(&self) -> anyhow::Result<Vec<StealoRule>>;
    async fn save_stealo_rules(&self, rules: Vec<StealoRule>) -> anyhow::Result<()>;
}

#[derive(Serialize, Deserialize)]
pub struct StealoRule {
    pub id: i32,
    pub name: String,
    pub elo: i32,
    pub description: String,
}