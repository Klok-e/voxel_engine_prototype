use std::collections::HashMap;

use bevy::prelude::{Entity, Resource};

use super::chunk::ChunkPosition;

#[derive(Debug, Resource, Default)]
pub struct EntityChunks {
    pub map: HashMap<ChunkPosition, Entity>,
}
