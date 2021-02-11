use crate::{core::Vec3i, directions::Directions};
use bitflags::_core::cmp::Ordering;
use ndarray::prelude::*;
use serde::{Deserialize, Serialize};

use super::voxel::Voxel;

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

    #[inline]
    pub fn data_mut(&mut self) -> &mut Array3<Voxel> {
        &mut self.data
    }

    #[inline]
    pub fn data(&self) -> &Array3<Voxel> {
        &self.data
    }

    /// Checks whether the provided index is on the chunk border
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

    #[inline]
    fn wrap_index(v: i32) -> (i32, i32) {
        match if v < 0 { v + Self::NI } else { v % Self::NI } {
            x if x == v => (0, x),
            x if x < 0 => (-1, x),
            x => (1, x),
        }
    }

    pub fn chunk_voxel_index_wrap(ind: &Vec3i) -> Option<Vec3i> {
        let v1 = Self::wrap_index(ind.x);
        let v2 = Self::wrap_index(ind.y);
        let v3 = Self::wrap_index(ind.z);

        let wrapped = ind.map(|v| match if v < 0 { v + Self::NI } else { v % Self::NI } {
            x if x == v => (0, x),
            x if x < 0 => (-1, x),
            x => (1, x),
        });
        let dir_wrapped = Vec3i::new(wrapped[0].0, wrapped[1].0, wrapped[2].0);
        let ind = Vec3i::new(wrapped[0].1, wrapped[1].1, wrapped[2].1);
        if dir_wrapped == Vec3i::new(0, 0, 0) {
            None
        } else {
            Some(ind)
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

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    const SMALLCH: usize = 3;
    //const SMALLCHI: i32 = SMALLCH as i32;
    type SmallChunk = Chunk<SMALLCH>;

    #[test]
    fn chunk_data_dimensions() {
        let mut chunk = SmallChunk::new();

        let data_mut = chunk.data_mut().shape().to_owned();
        let data = chunk.data().shape().to_owned();
        let data_inn = chunk.data.shape().to_owned();

        assert_eq!(data, vec![SMALLCH, SMALLCH, SMALLCH]);
        assert_eq!(data, data_mut);
        assert_eq!(data, data_inn);
    }

    #[rstest(to_wrap, exp_wrapped,
        // no wrap
        case(Vec3i::from([0,0,0]), None),
        case(Vec3i::from([0,2,0]), None),
        case(Vec3i::from([0,2,1]), None),
        // wrap
        case(Vec3i::from([-1,2,1]), Some(Vec3i::from([2,2,1]))),
        case(Vec3i::from([-1,-1,1]), Some(Vec3i::from([2,2,1]))),
        case(Vec3i::from([-1,3,1]), Some(Vec3i::from([2,0,1]))),
        // overwrap
        case(Vec3i::from([-1,4,1]), Some(Vec3i::from([2,1,1]))),
    )]
    fn chunk_voxel_index_wrap(to_wrap: Vec3i, exp_wrapped: Option<Vec3i>) {
        let actual_wrp = SmallChunk::chunk_voxel_index_wrap(&to_wrap);

        assert_eq!(actual_wrp, exp_wrapped);
    }
}
