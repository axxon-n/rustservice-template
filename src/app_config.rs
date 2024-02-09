use color_eyre::Result;
use envy;
use serde::Deserialize;
use tracing::info;
use tracing_subscriber::EnvFilter;

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Config {
    pub host: String,
    pub port: i32,
    pub parallel_files: usize,
    pub payload_max_size: usize,
    pub is_test: usize,
}

fn init_tracer() {
    #[cfg(debug_assertions)]
    let tracer = tracing_subscriber::fmt();
    #[cfg(not(debug_assertions))]
    let tracer = tracing_subscriber::fmt().json();
    tracer.with_env_filter(EnvFilter::from_default_env()).init();
}

impl Config {
    pub fn from_env() -> Result<Config> {
        init_tracer();
        info!("Loading configuration");
        let ret_env = match envy::from_env::<Config>() {
            Ok(environment) => environment,
            Err(_e) => return Ok(Config::default()),
        };
        Ok(ret_env)
    }
}