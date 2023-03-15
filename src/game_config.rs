use bevy::prelude::{Plugin, Resource, warn};
use serde::{Deserialize, Serialize};

use std::path::Path;

use crate::error;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GameConfig {
    pub generation_maintain_fps: f32,
    pub render_around_bubble: usize,
    pub generate_around_bubble: usize,
}

impl GameConfig {
    pub fn from_file_ron<P: AsRef<Path>>(path: P) -> error::Result<Self> {
        let str = std::fs::read_to_string(path)?;
        let config: GameConfig = ron::from_str(str.as_ref())?;
        if config.render_around_bubble >= config.generate_around_bubble {
            warn!(
                "It isn't recommended to have render bubble be bigger than generate bubble. 
                Render bubble size: {}. Generate bubble size: {}.",
                config.render_around_bubble, config.generate_around_bubble
            );
        }
        Ok(config)
    }
}

#[derive(Resource)]
pub struct RuntimeGameConfig {
    pub chunks_render_per_frame: u32,
    pub chunks_generate_per_frame: u32,
    pub config: GameConfig,
}

impl From<GameConfig> for RuntimeGameConfig {
    fn from(conf: GameConfig) -> Self {
        Self {
            config: conf,
            chunks_generate_per_frame: 1,
            chunks_render_per_frame: 1,
        }
    }
}

pub struct GameConfigPlugin {
    game_config: GameConfig,
}

impl GameConfigPlugin {
    pub fn new(game_config: GameConfig) -> Self {
        Self { game_config }
    }
}

impl Plugin for GameConfigPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(RuntimeGameConfig::from(self.game_config.clone()));
    }
}
