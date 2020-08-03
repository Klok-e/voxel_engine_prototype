use super::Voxel;
use crate::core::Vec3i;
use amethyst::ecs::prelude::*;
use bitflags::_core::cmp::Ordering;
use ndarray::prelude::*;
use serde::{Deserialize, Serialize};

pub const CHUNK_SIZE: usize = 32;
pub const CHUNK_SIZEI: i32 = CHUNK_SIZE as i32;

pub struct Chunk {
    data: Array3<Voxel>,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            data: Array3::default([CHUNK_SIZE + 2, CHUNK_SIZE + 2, CHUNK_SIZE + 2]),
        }
    }

    pub fn data(&mut self) -> ArrayViewMut3<Voxel> {
        self.data.slice_mut(s![1..-1, 1..-1, 1..-1])
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Deserialize, Serialize)]
pub struct ChunkPosition {
    pub pos: Vec3i,
}

impl ChunkPosition {
    pub fn new(pos: Vec3i) -> Self {
        ChunkPosition { pos }
    }
}

impl From<Vec3i> for ChunkPosition {
    fn from(value: Vec3i) -> Self {
        ChunkPosition::new(value)
    }
}

impl Default for ChunkPosition {
    fn default() -> Self {
        Self {
            pos: Vec3i::zeros(),
        }
    }
}

impl Ord for ChunkPosition {
    fn cmp(&self, other: &Self) -> Ordering {
        self.pos
            .x
            .cmp(&other.pos.x)
            .then(self.pos.y.cmp(&other.pos.y))
            .then(self.pos.z.cmp(&other.pos.z))
    }
}

impl PartialOrd for ChunkPosition {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Component for ChunkPosition {
    type Storage = DenseVecStorage<Self>;
}
