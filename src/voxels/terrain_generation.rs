use super::{chunk::ChunkPosition, voxel::Voxel};
use crate::core::{to_vec2d, Vec2i};
use ndarray::prelude::*;
use noise::{Fbm, NoiseFn, Seedable};

pub trait VoxelGenerator<const N: usize> {
    fn fill_random(&self, pos: &ChunkPosition, arr: &mut ArrayViewMut3<Voxel>);
}

pub struct ProceduralGenerator<const N: usize> {
    rng: Fbm,
}

impl<const N: usize> Default for ProceduralGenerator<N> {
    fn default() -> Self {
        ProceduralGenerator::new(42)
    }
}

impl<const N: usize> ProceduralGenerator<N> {
    const NI: i32 = N as i32;

    pub fn new(seed: u32) -> Self {
        Self {
            rng: Fbm::new().set_seed(seed),
        }
    }
}

impl<const N: usize> VoxelGenerator<N> for ProceduralGenerator<N> {
    fn fill_random(&self, pos: &ChunkPosition, arr: &mut ArrayViewMut3<Voxel>) {
        //let mut filled = 0;
        for x in 0..Self::NI {
            for z in 0..Self::NI {
                let p = Vec2i::from([x, z]);
                let p = to_vec2d(p + pos.pos.xz() * Self::NI);
                let value = self.rng.get([p.x / 100., p.y / 100.]);
                for y in 0..Self::NI {
                    let height = y + pos.pos[0] * Self::NI;
                    arr[(x as usize, y as usize, z as usize)] = match value {
                        value if height as f64 + 5. > value * 10. => Voxel { id: 0 },
                        _ => {
                            //filled += 1;
                            Voxel { id: 1 }
                        }
                    };
                }
            }
        }
        //dbg!(pos, filled);
    }
}
