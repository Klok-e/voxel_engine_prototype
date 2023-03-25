use std::{cell::Cell, cmp::Ordering, sync::Mutex};

use crate::directions::Directions;
use bevy::prelude::{Component, IVec3};
use ndarray::prelude::*;
use serde::{Deserialize, Serialize};

use super::voxel::Voxel;

pub const CHSIZE: usize = 32;
pub const CHSIZEI: i32 = CHSIZE as i32;
pub const CHSIZEF: f32 = CHSIZE as f32;

#[derive(Debug)]
pub struct Chunk<const N: usize> {
    data: Array3<Voxel>,
    is_transparent: Mutex<Cell<Option<bool>>>,
    is_nontransparent: Mutex<Cell<Option<bool>>>,
}

impl<const N: usize> Chunk<N> {
    const NI: i32 = N as i32;

    pub fn new() -> Self {
        Chunk {
            data: Array3::default([N, N, N]),
            is_transparent: Default::default(),
            is_nontransparent: Default::default(),
        }
    }

    #[inline]
    pub fn data_mut(&mut self) -> &mut Array3<Voxel> {
        self.is_transparent.lock().unwrap().set(None);
        self.is_nontransparent.lock().unwrap().set(None);
        &mut self.data
    }

    #[inline]
    pub fn data(&self) -> &Array3<Voxel> {
        &self.data
    }

    /// Checks whether the provided index is on the chunk border
    /// and if it is, return border direction
    pub fn is_on_border(ind: &[usize; 3]) -> Option<Directions> {
        let dir = IVec3::new(ind[0] as i32, ind[1] as i32, ind[2] as i32);
        let dir = IVec3::from_array(dir.to_array().map(|v| {
            if v == Self::NI - 1 {
                1
            } else if v == 0 {
                -1
            } else {
                0
            }
        }));

        if dir.x + dir.y + dir.z == 0 {
            None
        } else {
            let dir = Directions::from(dir);
            Some(dir)
        }
    }

    #[inline(always)]
    fn wrap(v: i32) -> i32 {
        if v < 0 {
            v + Self::NI
        } else {
            v % Self::NI
        }
    }

    #[inline]
    pub fn chunk_voxel_index_wrap(ind: &IVec3) -> Option<IVec3> {
        let x = Self::wrap(ind.x);
        let y = Self::wrap(ind.y);
        let z = Self::wrap(ind.z);
        if x == ind.x && y == ind.y && z == ind.z {
            None
        } else {
            Some([x, y, z].into())
        }
    }

    pub fn is_nontransparent(&self) -> bool {
        if let Some(v) = self.is_nontransparent.lock().unwrap().get() {
            return v;
        }
        let all = self.data.iter().all(|x| !x.is_transparent());
        self.is_nontransparent.lock().unwrap().set(Some(all));
        all
    }

    pub fn is_transparent(&self) -> bool {
        if let Some(v) = self.is_transparent.lock().unwrap().get() {
            return v;
        }
        let all = self.data.iter().all(|x| x.is_transparent());
        self.is_transparent.lock().unwrap().set(Some(all));
        all
    }
}

impl<const N: usize> Default for Chunk<N> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Deserialize, Serialize, Component)]
pub struct ChunkPosition {
    pub pos: IVec3,
}

impl ChunkPosition {
    pub fn new(pos: IVec3) -> Self {
        ChunkPosition { pos }
    }
}

impl From<IVec3> for ChunkPosition {
    fn from(value: IVec3) -> Self {
        ChunkPosition::new(value)
    }
}

impl Default for ChunkPosition {
    fn default() -> Self {
        Self { pos: IVec3::ZERO }
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
        case(IVec3::from([0,0,0]), None),
        case(IVec3::from([0,2,0]), None),
        case(IVec3::from([0,2,1]), None),
        // wrap
        case(IVec3::from([-1,2,1]), Some(IVec3::from([2,2,1]))),
        case(IVec3::from([-1,-1,1]), Some(IVec3::from([2,2,1]))),
        case(IVec3::from([-1,3,1]), Some(IVec3::from([2,0,1]))),
        // overwrap
        case(IVec3::from([-1,4,1]), Some(IVec3::from([2,1,1]))),
    )]
    fn chunk_voxel_index_wrap(to_wrap: IVec3, exp_wrapped: Option<IVec3>) {
        let actual_wrp = SmallChunk::chunk_voxel_index_wrap(&to_wrap);

        assert_eq!(exp_wrapped, actual_wrp);
    }
}
