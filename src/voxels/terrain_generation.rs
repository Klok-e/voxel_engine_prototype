use super::chunk::{Chunk, ChunkPosition, CHUNK_SIZEI};
use super::Voxel;
use crate::core::Vec3i;
use amethyst::{core::components::Transform, derive::SystemDesc, ecs::prelude::*, prelude::*};
use ndarray::prelude::*;
use ndarray::Zip;
use noise::{NoiseFn, Perlin};

pub struct ProceduralGenerator {
    rng: Perlin,
}

impl ProceduralGenerator {
    pub fn new() -> Self {
        Self { rng: Perlin::new() }
    }
    pub fn fill_random(&mut self, pos: &ChunkPosition, arr: &mut ArrayViewMut3<Voxel>) {
        for x in 0..CHUNK_SIZEI {
            for y in 0..CHUNK_SIZEI {
                for z in 0..CHUNK_SIZEI {
                    let p = Vec3i::from([x, y, z]);
                    let p = p + pos.pos * CHUNK_SIZEI;
                    arr[(x as usize, y as usize, z as usize)] = match p {
                        p if p.y % 2 == 0 => Voxel { id: 0 },
                        _ => Voxel { id: 1 },
                    };
                    //self.rng.get(p.into());
                }
            }
        }
    }
}
