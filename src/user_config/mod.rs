use serde;

#[derive(Debug, serde::Serialize, serde::Deserialize, )]
pub struct Config {
    pub path: String,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
}

pub fn read_config() -> Config{
    let content = std::fs::read_to_string("Config.toml").unwrap();
    toml::from_str(&content).unwrap()
}
