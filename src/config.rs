use anyhow::{anyhow, Result};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Client {
    pub id: String,
    pub secret: String,
}

#[derive(Deserialize, Debug)]
pub struct Database {
    pub driver: String,
    pub server: String,
    pub user: User,
    pub database: String,
}

#[derive(Deserialize, Debug)]
pub struct User {
    pub id: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct Settings {
    pub client: Client,
    pub database: Database,
}

impl Settings {
    pub fn new(path: String) -> Result<Self> {
        let contents = std::fs::read_to_string(path)?;
        match toml::from_str(&contents) {
            Ok(s) => Ok(s),
            Err(e) => Err(anyhow!("Error parsing config file: {}", e)),
        }
    }
}
