use clap::Parser;
use config::{Config as ConfigLoader, File, FileFormat};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct PostgresConfig {
    pub url: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Server {
    // Which HOST the server is listening to
    pub host: String,
    // Which PORT the server is listening to
    pub port: u16,
    // Which PORT the server jaeger collection is listening to
    pub jaeger_port: u16,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    // Which errors we want to log (info, warn or error)
    pub log_level: String,
    pub service_name: String,
    pub server: Server,
    pub database: Option<String>,
    pub postgres: PostgresConfig,
}

#[derive(Parser, Debug)]
pub struct Args {
    // Config file
    #[clap(short, long, default_value = "config-default.toml")]
    pub config_path: String,
}

impl Config {
    pub fn new() -> Result<Config, config::ConfigError> {
        let args = Args::parse();
        let content = ConfigLoader::builder()
            .add_source(File::new(&args.config_path, FileFormat::Toml))
            .build()?;
        let config: Config = content.try_deserialize::<Config>()?;

        Ok(config)
    }
}
