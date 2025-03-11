use std::fs;
use persistence::stealo_rule::StealoRule;
use serde::{Deserialize, Serialize};

fn main() -> anyhow::Result<()> {
    let mut migrations_yml = serde_yaml::from_str::<serde_yaml::Value>("./migrations/migrations.yml")
        .expect("Could not find migrations.yml");
    let mut version: u64 = migrations_yml.get_mut("version").expect("Missing version")
        .as_u64().expect("Expected version as number");
    let rules_json = fs::read_to_string("./migrations/rules.json").expect("File not found.");
    let rules_data: RulesData = serde_json::from_str(&rules_json).expect("Invalid JSON format.");
    if version != rules_data.version {
        println!("MIGRATE THAT BISH");
        version = rules_data.version;
        serde_yaml::to_writer(std::io::stdout(), &version).expect("Could not write.");
    }
    else {
        println!("Migrations already up to date. Skipping.");
    }
    Ok(())
}

#[derive(Deserialize, Serialize)]
struct RulesData {
    version: u64,
    rules: Vec<StealoRule>,
}