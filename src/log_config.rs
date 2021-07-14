use std::path::Path;

use amethyst::LogLevelFilter;
use serde::{Deserialize, Serialize};

use crate::error;

#[derive(Debug, Deserialize, Serialize)]
pub struct LogConfig {
    /// Sets the overarching level filter for the logger.
    pub level_filter: log::LevelFilter,

    /// Sets the levels for specific modules.
    pub module_levels: Vec<(String, log::LevelFilter)>,
}

impl LogConfig {
    pub fn from_file_toml<P: AsRef<Path>>(path: P) -> error::Result<Self> {
        let str = std::fs::read_to_string(path)?;
        let log_levels_config = toml::from_str(str.as_ref())?;
        Ok(log_levels_config)
    }
}
