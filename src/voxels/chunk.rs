use super::Voxel;
use crate::{
    core::{to_uarr, to_vecf, Vec3f, Vec3i},
    directions::Directions,
    voxels::chunk_mesh::ChunkMeshData,
};
use amethyst::ecs::prelude::*;
use bitflags::_core::cmp::Ordering;
use ndarray::prelude::*;
use serde::{Deserialize, Serialize};

pub const CHSIZE: usize = 32;
pub const CHSIZEI: i32 = CHSIZE as i32;
pub const CHSIZEF: f32 = CHSIZE as f32;

#[derive(Debug)]
pub struct Chunk<const N: usize> {
    data: Array3<Voxel>,
}

impl<const N: usize> Chunk<N> {
    const NI: i32 = N as i32;

    pub fn new() -> Self {
        Chunk {
            data: Array3::default([N, N, N]),
        }
    }

    pub fn data_mut(&mut self) -> ArrayViewMut3<Voxel> {
        self.data.view_mut()
    }
    pub fn data(&self) -> ArrayView3<Voxel> {
        self.data.view()
    }

    pub fn mesh(&self) -> ChunkMeshData {
        let one: Vec3i = [1, 1, 1].into();
        let onef: Vec3f = [1., 1., 1.].into();

        let mut chunk_mesh = ChunkMeshData::new();
        for x in 0..Self::NI {
            for y in 0..Self::NI {
                for z in 0..Self::NI {
                    let pos: Vec3i = [x, y, z].into();
                    if self.data[to_uarr(pos + one)].is_transparent() {
                        // if current voxel is transparent
                        continue;
                    }
                    // if current voxel is solid
                    for dir in Directions::all().into_iter() {
                        let spos: Vec3i = pos + dir.to_vec::<i32>();
                        if self.data[to_uarr(spos + one)].is_transparent() {
                            // if adjacent voxel is transparent
                            chunk_mesh.insert_quad(to_vecf(pos) + onef / 2., dir);
                        }
                    }
                }
            }
        }

        chunk_mesh
    }

    /// Checks whether the provided idnex is on the chunk border
    /// and if it is, return border direction
    pub fn is_on_border(ind: &[usize; 3]) -> Option<Directions> {
        let dir = Vec3i::new(ind[0] as i32, ind[1] as i32, ind[2] as i32);
        let dir = dir.map(|v| {
            if v == Self::NI - 1 {
                1
            } else if v == 0 {
                -1
            } else {
                0
            }
        });

        if dir.x + dir.y + dir.z == 0 {
            None
        } else {
            let dir = Directions::from(dir);
            Some(dir)
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    const SMALLCH: usize = 3;
    //const SMALLCHI: i32 = SMALLCH as i32;
    type SmallChunk = Chunk<SMALLCH>;

    #[test]
    fn chunk_data_dimensions() {
        let mut chunk = SmallChunk::new();

        let data_priv_shp = chunk.data.shape().to_owned();
        let data_mut_shp = chunk.data_mut().shape().to_owned();
        let data_shp = chunk.data().shape().to_owned();

        assert_eq!(data_shp, vec![SMALLCH, SMALLCH, SMALLCH]);
        assert_eq!(data_priv_shp, data_mut_shp);
        assert_eq!(data_mut_shp, data_shp);
    }
}
