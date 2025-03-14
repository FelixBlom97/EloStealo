use persistence::elo_stealo_postgres::EloStealoPostgresStore;
use persistence::elo_stealo_repository::EloStealoRepository;
use persistence::stealo_rule::StealoRule;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};
use std::{env, fs};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load the necessary files
    let mut migrations_yml =
        File::open("./migrations/migrations.yml").expect("could not find migrations.yml");
    let mut content = String::new();
    migrations_yml.read_to_string(&mut content)?;
    let mut migrations =
        serde_yaml::from_str::<MigrationVersion>(&content).expect("Failed to parse migrations.yml");
    let rules_json = fs::read_to_string("./migrations/rules.json").expect("File not found.");
    let rules_data: RulesData = serde_json::from_str(&rules_json).expect("Invalid JSON format.");

    // Perform migrations if current version is behind.
    if migrations.version != rules_data.version {
        println!("Migrating version {}", rules_data.version);
        let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| "localhost:5433".into());
        let postgres_user = env::var("POSTGRES_USER").unwrap_or_else(|_| "postgres".into());
        let postgres_password = env::var("POSTGRES_PASSWORD").unwrap_or_else(|_| "postgres".into());
        let database_name = env::var("DATABASE_NAME").unwrap_or_else(|_| "EloStealo".into());
        let connection_string = format!(
            "postgres://{}:{}@{}/{}",
            postgres_user, postgres_password, database_url, database_name
        );
        println!("Connecting to {}", connection_string);
        let db: Box<dyn EloStealoRepository> = EloStealoPostgresStore::new(connection_string).await;
        db.delete_old_stealo_rules().await?;
        for rule in rules_data.rules {
            db.insert_stealo_rule(rule).await?;
        }

        let new_version = MigrationVersion {
            version: rules_data.version,
        };
        let new_version_str =
            serde_yaml::to_string(&new_version).expect("Failed to serialize yaml");
        File::create("./migrations/migrations.yml")?
            .write_all(new_version_str.as_bytes())
            .expect("Failed to overwrite migrations.yml");
    } else {
        println!("Migrations already up to date. Skipping.");
    }
    Ok(())
}

#[derive(Deserialize, Serialize)]
struct RulesData {
    version: u64,
    rules: Vec<StealoRule>,
}

#[derive(Deserialize, Serialize)]
struct MigrationVersion {
    version: u64,
}
