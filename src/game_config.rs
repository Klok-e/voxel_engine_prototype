use amethyst::ecs::SystemBundle;
use log::warn;
use serde::{Deserialize, Serialize};

use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GameConfig {
    pub chunks_render_per_frame: usize,
    pub chunks_generate_per_frame: usize,
    pub render_around_bubble: usize,
    pub generate_around_bubble: usize,
}

impl GameConfig {
    pub fn from_file_ron<P: AsRef<Path>>(path: P) -> Result<Self, crate::Error> {
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

pub struct RuntimeGameConfig {
    pub chunks_generate_per_frame: usize,
    pub render_around_bubble: usize,
    pub config: GameConfig,
}

impl From<GameConfig> for RuntimeGameConfig {
    fn from(conf: GameConfig) -> Self {
        Self {
            config: conf,
            chunks_generate_per_frame: 1,
            render_around_bubble: 1,
        }
    }
}

pub struct ConfigsBundle {
    game_config: GameConfig,
}

impl ConfigsBundle {
    pub fn new(game_config: GameConfig) -> Self {
        Self { game_config }
    }
}

impl SystemBundle for ConfigsBundle {
    fn load(
        &mut self,
        _world: &mut legion::World,
        resources: &mut legion::Resources,
        _builder: &mut amethyst::ecs::DispatcherBuilder,
    ) -> Result<(), amethyst::Error> {
        resources.insert(RuntimeGameConfig::from(self.game_config.clone()));

        Ok(())
    }
}
