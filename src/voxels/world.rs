use super::chunk::Chunk;
use super::chunk::{ChunkPosition, CHUNK_SIZE};
use crate::core::{Vec3f, Vec3i};
use amethyst::{core::components::Transform, derive::SystemDesc, ecs::prelude::*, prelude::*};
use std::collections::{HashMap, HashSet};

#[derive(Default)]
pub struct VoxelWorld {
    pub chunks: HashMap<Vec3i, Entity>,
}
