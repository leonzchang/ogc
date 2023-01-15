use std::{fs, path::PathBuf};

use fantoccini::{Client, ClientBuilder, Locator};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub account: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub user: User,
}

impl Config {
    pub fn load(file_path: &PathBuf) -> anyhow::Result<Self> {
        let config_string = fs::read_to_string(file_path)?;
        Ok(toml::from_str(&config_string)?)
    }
}

#[derive(Clone, Debug)]
pub struct Cheater {
    pub client: Client,
    pub config: Config,
}

impl Cheater {
    pub async fn new(
        config_path: Option<PathBuf>,
        web_driver_url: Option<&str>,
    ) -> anyhow::Result<Self> {
        let config_path = config_path.unwrap_or("./deployment//dev.toml".into());
        let config = Config::load(&config_path)?;

        let web_driver_url = web_driver_url.unwrap_or("http://localhost:9515");
        let client = ClientBuilder::native().connect(web_driver_url).await?;

        Ok(Self { client, config })
    }
}
