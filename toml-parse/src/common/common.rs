use crate::errors::error::ConfigError;

pub type Result<T> = std::result::Result<T, ConfigError>;

pub(crate) const CONFIG_FILENAME: &str = "src/conf/config.toml";