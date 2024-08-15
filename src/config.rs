use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Config {
    pub app_name: String,
    pub base2_version: String,
    pub bind_address: String, // ip:port
    pub access_token: String,
    pub database_url: String,
    pub data_dir: String,
    pub https_schema: bool,
    pub app_version: String,
    pub main_menu: IndexMap<String, String>,
}

impl Config {
    pub fn new(app_version: &str, main_menu: IndexMap<String, String>) -> Self {
        dotenv::dotenv().ok();
        config::Config::builder()
            .set_default("base2_version", env!("CARGO_PKG_VERSION"))
            .expect("can't set base2_version")
            .set_default("app_version", app_version)
            .expect("can't set app_version")
            .set_default("main_menu", main_menu)
            .expect("can't set main_menu")
            .set_default("https_schema", false)
            .expect("can't set https_schema")
            .add_source(config::Environment::default())
            .build()
            .expect("can't parse AppConfig")
            .try_deserialize()
            .expect("can't parse AppConfig")
    }
}
