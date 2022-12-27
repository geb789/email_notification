use std::fs;
use std::path::PathBuf;
use serde::Deserialize;
use clap::Parser;


#[derive(Clone, Debug, Deserialize)]
pub struct EmailConfig {
    pub address: String,
    pub password: String,
    pub smtp_host: String,
    pub smtp_server: String,
    pub smtp_port: String,
    pub smtp_ssl: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AppConfig {
    pub email: EmailConfig,
}
#[derive(Debug, Parser)]
pub struct CliOpts {
    /// Sets a custom config file
    #[clap(short = 'c', long, name = "FILE")]
    pub config: PathBuf,

    /// command scene
    #[clap(short = 's', long, name = "SCENE", possible_values = ["mint", "oracle", "npow", "syncoracle", "syncpow", "ezcoracle", "imonline"])]
    pub scene: String,
}


impl CliOpts {
    pub fn init() -> Self {
        Parser::parse()
    }

    pub fn parse(&self) -> AppConfig {
        let toml_str = fs::read_to_string(self.config.as_path()).expect("valid path");
        let mut config = toml::from_str::<AppConfig>(toml_str.as_str()).expect("valid toml");
        config
    }
}
