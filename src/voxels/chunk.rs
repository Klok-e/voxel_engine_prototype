use super::Voxel;
use crate::core::Vec3i;
use amethyst::ecs::prelude::*;
use ndarray::prelude::*;

pub const CHUNK_SIZE: usize = 32;

pub struct Chunk {
    data: Array3<Voxel>,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            data: Array3::default([CHUNK_SIZE, CHUNK_SIZE, CHUNK_SIZE]),
        }
    }
}

impl Component for Chunk {
    type Storage = DenseVecStorage<Self>;
}

pub struct ChunkPosition {
    pub pos: Vec3i,
}

impl ChunkPosition {
    pub fn new(pos: Vec3i) -> Self {
        ChunkPosition { pos }
    }
}

impl Component for ChunkPosition {
    type Storage = DenseVecStorage<Self>;
}
