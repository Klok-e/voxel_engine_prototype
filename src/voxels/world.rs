use super::chunk::SChunk;
use super::chunk::{ChunkPosition, CHSIZE};
use super::terrain_generation::ProceduralGenerator;
use super::voxel::Voxel;
use crate::core::{ConcurrentHashMap, ConcurrentHashSet, Vec3f, Vec3i};
use amethyst::{core::components::Transform, derive::SystemDesc, ecs::prelude::*, prelude::*};
use flurry::epoch::Guard;
use std::collections::{HashMap, HashSet};
use std::sync::{RwLock, RwLockWriteGuard};

#[derive(Default)]
pub struct VoxelWorld {
    chunks: ConcurrentHashMap<ChunkPosition, RwLock<SChunk>>,
    dirty: ConcurrentHashSet<ChunkPosition>,
}

impl VoxelWorld {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn chunk_at_or_create<'a>(
        &'a self,
        pos: &ChunkPosition,
        guard: &'a Guard,
    ) -> &'a RwLock<SChunk> {
        let chunk = match self.chunks.get(pos, guard) {
            Some(c) => c,
            None => {
                let mut c = SChunk::new();
                ProceduralGenerator::new().fill_random(&pos, &mut c.data_mut());
                self.chunks.insert(*pos, RwLock::new(c), guard);
                self.chunks.get(pos, guard).unwrap()
            }
        };
        chunk
    }

    pub fn voxel_at(&self, chunk: &ChunkPosition, pos: &[usize; 3], guard: &Guard) -> Voxel {
        let chunk = self.chunk_at_or_create(chunk, guard).write().unwrap();
        chunk.data()[*pos]
    }
}
