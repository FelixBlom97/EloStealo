use config::{Config, ConfigError, File};
use serde::Deserialize;
use std::env;

#[derive(Deserialize)]
pub struct ApplicationSettings {
    pub host: [u8; 4],
    pub port: u16,
}

impl ApplicationSettings {
    pub fn load() -> Result<Self, ConfigError> {
        let run_mode = env::var("CONFIG").unwrap_or_else(|_| "default".into());

        let builder = Config::builder()
            .add_source(File::with_name("./config/default.yaml"))
            .add_source(File::with_name(&format!("./config/{}.yaml", run_mode)).required(false))
            .build()?;

        builder.try_deserialize()
    }
}
