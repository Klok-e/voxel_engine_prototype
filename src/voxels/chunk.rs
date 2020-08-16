use super::Voxel;
use crate::core::{to_uarr, to_vecf, Vec3f, Vec3i};
use crate::directions::Directions;
use crate::voxels::chunk_mesh::ChunkMeshData;
use amethyst::ecs::prelude::*;
use amethyst::renderer::rendy::mesh::MeshBuilder;
use bitflags::_core::cmp::Ordering;
use ndarray::prelude::*;
use serde::{Deserialize, Serialize};

pub const CHUNK_SIZE: usize = 8;
pub const CHUNK_SIZEI: i32 = CHUNK_SIZE as i32;
pub const CHUNK_SIZEF: f32 = CHUNK_SIZE as f32;

pub struct Chunk {
    data: Array3<Voxel>,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            data: Array3::default([CHUNK_SIZE + 2, CHUNK_SIZE + 2, CHUNK_SIZE + 2]),
        }
    }

    pub fn data_mut(&mut self) -> ArrayViewMut3<Voxel> {
        self.data.slice_mut(s![1..-1, 1..-1, 1..-1])
    }
    pub fn data(&self) -> ArrayView3<Voxel> {
        self.data.slice(s![1..-1, 1..-1, 1..-1])
    }

    pub fn mesh(&self) -> MeshBuilder<'_> {
        let one: Vec3i = [1, 1, 1].into();
        let onef: Vec3f = [1., 1., 1.].into();

        let mut chunk_mesh = ChunkMeshData::new();
        for x in 0..CHUNK_SIZEI {
            for y in 0..CHUNK_SIZEI {
                for z in 0..CHUNK_SIZEI {
                    let pos: Vec3i = [x, y, z].into();
                    if self.data[to_uarr(pos)].is_transparent() {
                        // if current voxel is transparent
                        continue;
                    }
                    // if current voxel is solid
                    for dir in Directions::all().into_iter() {
                        let dir: Directions = dir;
                        let spos: Vec3i = pos.clone() + dir.to_vec::<i32>() + one.clone();
                        if self.data[to_uarr(spos)].is_transparent() {
                            // if adjacent voxel is transparent
                            chunk_mesh.insert_quad(to_vecf(pos) + onef / 2., dir);
                        }
                    }
                }
            }
        }

        chunk_mesh.build_mesh()
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

// impl Ord for ChunkPosition {
//     fn cmp(&self, other: &Self) -> Ordering {
//         self.pos
//             .x
//             .cmp(&other.pos.x)
//             .then(self.pos.y.cmp(&other.pos.y))
//             .then(self.pos.z.cmp(&other.pos.z))
//     }
// }

// impl PartialOrd for ChunkPosition {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(self.cmp(other))
//     }
// }

impl Component for ChunkPosition {
    type Storage = DenseVecStorage<Self>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chunk_data_dimensions() {
        let mut chunk = Chunk::new();

        let data = chunk.data_mut();

        assert_eq!(data.shape(), &[CHUNK_SIZE, CHUNK_SIZE, CHUNK_SIZE]);
    }
}
