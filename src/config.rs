use config::ConfigError;
use serde::Deserialize;
use slog::{o, Drain, Logger};
use slog_async;
use slog_envlogger;
use slog_term;

#[derive(Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: i32,
}

#[derive(Deserialize)]
pub struct Config {
    pub server: ServerConfig,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let mut conf = config::Config::new();

        conf.merge(config::Environment::new())?;
        conf.try_into()
    }

    pub fn configure_log() -> Logger {
        let decorator = slog_term::TermDecorator::new().build();
        let console_drain = slog_term::FullFormat::new(decorator).build().fuse();
        let console_drain = slog_envlogger::new(console_drain);
        let console_drain = slog_async::Async::new(console_drain).build().fuse();
        slog::Logger::root(console_drain, o!("v" => env!("CARGO_PKG_VERSION")))
    }
}
