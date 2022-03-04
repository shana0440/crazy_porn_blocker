use confy;
use egg_mode::KeyPair;
use serde::{Deserialize, Serialize};
use std::default::Default;

#[derive(Serialize, Deserialize)]
pub struct Config {
    consumer_api_key: String,
    consumer_api_key_secret: String,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            consumer_api_key: String::new(),
            consumer_api_key_secret: String::new(),
        }
    }
}

impl Config {
    pub fn load() -> Config {
        confy::load_path("./config.toml").unwrap()
    }

    pub fn con_token(&self) -> KeyPair {
        KeyPair::new(
            self.consumer_api_key.clone(),
            self.consumer_api_key_secret.clone(),
        )
    }
}
