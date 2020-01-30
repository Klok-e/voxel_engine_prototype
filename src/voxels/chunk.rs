use super::Voxel;
use crate::core::Vec3i;
use amethyst::ecs::prelude::*;
use ndarray::prelude::*;

pub const CHUNK_SIZE: usize = 32;

pub struct Chunk {
    data: Array3<Voxel>,
    pub maintained: bool,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            data: Array3::default([CHUNK_SIZE, CHUNK_SIZE, CHUNK_SIZE]),
            maintained: false,
        }
    }

    pub fn maintain(&mut self, value: bool) -> &mut Self {
        self.maintained = value;
        self
    }
    pub fn maintained(mut self, value: bool) -> Self {
        self.maintained = value;
        self
    }
}

impl Component for Chunk {
    type Storage = DenseVecStorage<Self>;
}

pub struct ChunkPosition {
    pub pos: Vec3i,
}

impl Component for ChunkPosition {
    type Storage = DenseVecStorage<Self>;
}
