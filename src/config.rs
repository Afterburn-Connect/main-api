use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub surreal_username: String,
    pub surreal_password: String,
    pub surreal_host: String,
    pub nats_host: String,
    pub surreal_db: String,
}

lazy_static! {
    pub static ref CONFIG: Config = {
        let config = std::fs::read_to_string("config.yaml").unwrap();
        serde_yaml::from_str(&config).unwrap()
    };
}

pub fn get_config() -> Config {
    CONFIG.clone()
}