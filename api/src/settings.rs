use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Database {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct Teloxide {
    pub token: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Pack {
    pub logo: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub database: Database,
    pub teloxide: Teloxide,
    pub pack: Pack,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();

        s.merge(File::with_name("config/default"))?;
        s.merge(Environment::with_prefix("DATABASE_URL"))?;
        s.merge(Environment::with_prefix("TELOXIDE_TOKEN"))?;
        s.merge(File::with_name("config/develop").required(false))?;

        s.try_into()
    }
}
