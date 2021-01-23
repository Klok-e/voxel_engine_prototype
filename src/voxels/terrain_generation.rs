use super::{chunk::ChunkPosition, voxel::Voxel};
use crate::core::{to_vecf, Vec3i};
use ndarray::prelude::*;
use noise::{Fbm, NoiseFn, Seedable};

pub struct ProceduralGenerator<const N: usize> {
    rng: Fbm,
}

impl<const N: usize> Default for ProceduralGenerator<N> {
    fn default() -> Self {
        ProceduralGenerator::new()
    }
}

impl<const N: usize> ProceduralGenerator<N> {
    const NI: i32 = N as i32;

    pub fn new() -> Self {
        Self {
            rng: Fbm::new().set_seed(42),
        }
    }
    pub fn fill_random(&self, pos: &ChunkPosition, arr: &mut ArrayViewMut3<Voxel>) {
        //let mut filled = 0;
        for x in 0..Self::NI {
            for y in 0..Self::NI {
                for z in 0..Self::NI {
                    let p = Vec3i::from([x, y, z]);
                    let p = to_vecf(p + pos.pos * Self::NI);
                    let value = self.rng.get([p.x as f64 / 100., p.z as f64 / 100.]);
                    arr[(x as usize, y as usize, z as usize)] = match value {
                        value if p.y as f64 + 5. > value * 10. => Voxel { id: 0 },
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
