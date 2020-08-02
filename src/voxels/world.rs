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
    chunks: ConcurrentHashMap<ChunkPosition, RwLock<Chunk>>,
    dirty: ConcurrentHashSet<ChunkPosition>,
}

impl VoxelWorld {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn guard(&self) -> Guard {
        self.chunks.guard()
    }

    pub fn chunk_at(&self, pos: ChunkPosition, guard: &Guard) -> ChunkRefMut {
        let chunk = match self.chunks.get(&pos, guard) {
            Some(chunk) => chunk,
            None => {
                let mut c = Chunk::new();
                ProceduralGenerator::new().fill_random(&pos, &mut c.data);
                self.chunks.insert(pos, RwLock::new(c), guard);
                self.chunks.get(&pos, guard).unwrap()
            }
        };
        ChunkRefMut {
            chunk,
            dirty: &self.dirty,
        }
        //self.chunks.entry(pos).or_insert_with(|| {
        //    let mut c = Chunk::new();
        //    ProceduralGenerator::new().fill_random(&pos, &mut c.data);
        //    c
        //})
    }
}

pub struct ChunkRefMut<'a> {
    chunk: &'a RwLock<Chunk>,
    dirty: &'a ConcurrentHashSet<ChunkPosition>,
}

impl ChunkRefMut {
    fn new(chunk: &RwLock<Chunk>, dirty: &ConcurrentHashSet<ChunkPosition>) -> Self {
        Self { chunk, dirty }
    }

    pub fn chunk(&mut self) -> RwLockWriteGuard<'_, Chunk> {
        self.chunk.write().unwrap()
    }
}
