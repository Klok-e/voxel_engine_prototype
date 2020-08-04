use super::chunk::Chunk;
use super::chunk::{ChunkPosition, CHUNK_SIZE};
use super::terrain_generation::ProceduralGenerator;
use crate::core::{ConcurrentHashMap, ConcurrentHashSet, Vec3f, Vec3i};
use amethyst::{core::components::Transform, derive::SystemDesc, ecs::prelude::*, prelude::*};
use flurry::epoch::Guard;
use std::collections::{HashMap, HashSet};
use std::sync::{RwLock, RwLockWriteGuard};

#[derive(Default)]
pub struct VoxelWorld {
    chunks: HashMap<ChunkPosition, RwLock<Chunk>>,
    dirty: HashSet<ChunkPosition>,
}

impl VoxelWorld {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn chunk_at_or_create(&mut self, pos: &ChunkPosition) -> &RwLock<Chunk> {
        let chunk = self.chunks.entry(*pos)
            .or_insert_with(|| {
                let mut c = Chunk::new();
                ProceduralGenerator::new().fill_random(&pos,  &mut c.data());
                RwLock::new(c)
            });
        chunk
    }
}
