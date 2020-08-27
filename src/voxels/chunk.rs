use super::Voxel;
use crate::core::{to_uarr, to_vecf, Vec3f, Vec3i};
use crate::directions::Directions;
use crate::voxels::chunk_mesh::ChunkMeshData;
use amethyst::ecs::prelude::*;
use amethyst::renderer::rendy::mesh::MeshBuilder;
use bitflags::_core::cmp::Ordering;
use ndarray::prelude::*;
use ndarray::Zip;
use serde::{Deserialize, Serialize};
use std::convert::identity;

pub const CHSIZE: usize = 32;
pub const CHSIZEI: i32 = CHSIZE as i32;
pub const CHSIZEF: f32 = CHSIZE as f32;

pub type SChunk = Chunk<CHSIZE>;

#[derive(Debug)]
pub struct Chunk<const N: usize> {
    data: Array3<Voxel>,
}

impl<const N: usize> Chunk<N> {
    const NI: i32 = N as i32;

    pub fn new() -> Self {
        Chunk {
            data: Array3::default([N + 2, N + 2, N + 2]),
        }
    }

    pub fn data_mut(&mut self) -> ArrayViewMut3<Voxel> {
        self.data.slice_mut(s![1..-1, 1..-1, 1..-1])
    }
    pub fn data(&self) -> ArrayView3<Voxel> {
        self.data.slice(s![1..-1, 1..-1, 1..-1])
    }

    fn copy_face_up(
        data: &mut Array3<Voxel>,
        other: &Array3<Voxel>,
        index_transform: impl Fn((i32, i32, i32)) -> (i32, i32, i32),
    ) {
        let one: Vec3i = [1, 1, 1].into();
        for x in 0..(Self::NI) {
            for z in 0..(Self::NI) {
                let (xi, yi, zi) = index_transform((x, Self::NI, z));
                let dest_index: Vec3i = [xi, yi, zi].into();
                let (xi, yi, zi) = index_transform((x, 0, z));
                let source_index: Vec3i = [xi, yi, zi].into();

                data[to_uarr(dest_index + one)] = other[to_uarr(source_index + one)];
            }
        }
    }

    pub fn copy_borders(&mut self, other: &Self, dir: Directions) {
        let dir: Directions = dir.to_vec::<i32>().into();
        match dir {
            x if x == Directions::NORTH => {
                Self::copy_face_up(&mut self.data, &other.data, |p| {
                    Self::rotate90_yz(Self::rotate90_yz(Self::rotate90_yz(p)))
                });
            }
            x if x == Directions::SOUTH => {
                Self::copy_face_up(&mut self.data, &other.data, |p| Self::rotate90_yz(p));
            }
            x if x == Directions::WEST => {
                Self::copy_face_up(&mut self.data, &other.data, |p| Self::rotate90_xy(p));
            }
            x if x == Directions::EAST => {
                Self::copy_face_up(&mut self.data, &other.data, |p| {
                    Self::rotate90_xy(Self::rotate90_xy(Self::rotate90_xy(p)))
                });
            }
            x if x == Directions::UP => {
                Self::copy_face_up(&mut self.data, &other.data, |p| identity(p));
            }
            x if x == Directions::DOWN => {
                Self::copy_face_up(&mut self.data, &other.data, |p| {
                    Self::reverse_x(Self::rotate90_xy(Self::rotate90_xy(p)))
                });
            }
            // x if x == (Directions::UP | Directions::EAST) => {
            //     copy_face_up(&mut self.data, &other, |p| identity(p));
            // }
            _ => {} //todo!("add all 26 combinations of directions")
        }
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

    fn transpose_xy((x, y, z): (i32, i32, i32)) -> (i32, i32, i32) {
        (y, x, z)
    }
    fn transpose_xz((x, y, z): (i32, i32, i32)) -> (i32, i32, i32) {
        (z, y, x)
    }
    fn transpose_yz((x, y, z): (i32, i32, i32)) -> (i32, i32, i32) {
        (x, z, y)
    }
    fn reverse_x((x, y, z): (i32, i32, i32)) -> (i32, i32, i32) {
        (Self::NI - x - 1, y, z)
    }
    fn reverse_y((x, y, z): (i32, i32, i32)) -> (i32, i32, i32) {
        (x, Self::NI - y - 1, z)
    }
    fn reverse_z((x, y, z): (i32, i32, i32)) -> (i32, i32, i32) {
        (x, y, Self::NI - z - 1)
    }
    pub fn rotate90_xy((x, y, z): (i32, i32, i32)) -> (i32, i32, i32) {
        Self::reverse_x(Self::transpose_xy((x, y, z)))
    }
    pub fn rotate90_xz((x, y, z): (i32, i32, i32)) -> (i32, i32, i32) {
        Self::reverse_x(Self::transpose_xz((x, y, z)))
    }
    pub fn rotate90_yz((x, y, z): (i32, i32, i32)) -> (i32, i32, i32) {
        Self::reverse_y(Self::transpose_yz((x, y, z)))
    }

    /// Checks whether the provided idnex is on the chunk border
    /// and if it is, return border direction
    pub fn is_on_border(ind: &[usize; 3]) -> Option<Directions> {
        let mut dir = Vec3i::new(ind[0] as i32, ind[1] as i32, ind[2] as i32);
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
    use rstest::rstest;

    const SMALLCH: usize = 3;
    const SMALLCHI: i32 = SMALLCH as i32;
    type SmallChunk = Chunk<SMALLCH>;

    #[test]
    fn chunk_data_dimensions() {
        let mut chunk = SmallChunk::new();

        let data = chunk.data_mut();

        assert_eq!(data.shape(), &[SMALLCH, SMALLCH, SMALLCH]);
    }

    fn check(
        expected: &Array3<i32>,
        control: &Array3<i32>,
        fn_view: impl Fn((i32, i32, i32)) -> (i32, i32, i32),
    ) {
        let mut actual = Array3::default([SMALLCH, SMALLCH, SMALLCH]);
        Zip::indexed(&mut actual).apply(|(x, y, z), actual| {
            let (x, y, z) = fn_view((x as i32, y as i32, z as i32));
            *actual = control[[x as usize, y as usize, z as usize]];
        });
        assert_eq!(expected, &actual);
    }

    /// expected results were checked by hand with a python visualization
    #[test]
    fn rotate_xy() {
        let control = array![
            [[1, 10, 19], [2, 11, 20], [3, 12, 21]],
            [[4, 13, 22], [5, 14, 23], [6, 15, 24]],
            [[7, 16, 25], [8, 17, 26], [9, 18, 27]]
        ];
        let expected = array![
            [[7, 16, 25], [4, 13, 22], [1, 10, 19]],
            [[8, 17, 26], [5, 14, 23], [2, 11, 20]],
            [[9, 18, 27], [6, 15, 24], [3, 12, 21]]
        ];

        check(&expected, &control, |p| SmallChunk::rotate90_xy(p));
    }

    /// expected results were checked by hand with a python visualization
    #[test]
    fn rotate_xz() {
        let control = array![
            [[1, 10, 19], [2, 11, 20], [3, 12, 21]],
            [[4, 13, 22], [5, 14, 23], [6, 15, 24]],
            [[7, 16, 25], [8, 17, 26], [9, 18, 27]]
        ];
        let expected = array![
            [[7, 4, 1], [8, 5, 2], [9, 6, 3]],
            [[16, 13, 10], [17, 14, 11], [18, 15, 12]],
            [[25, 22, 19], [26, 23, 20], [27, 24, 21]]
        ];

        check(&expected, &control, |p| SmallChunk::rotate90_xz(p));
    }

    /// expected results were checked by hand with a python visualization
    #[test]
    fn rotate_yz() {
        let control = array![
            [[1, 10, 19], [2, 11, 20], [3, 12, 21]],
            [[4, 13, 22], [5, 14, 23], [6, 15, 24]],
            [[7, 16, 25], [8, 17, 26], [9, 18, 27]]
        ];
        let expected = array![
            [[3, 2, 1], [12, 11, 10], [21, 20, 19]],
            [[6, 5, 4], [15, 14, 13], [24, 23, 22]],
            [[9, 8, 7], [18, 17, 16], [27, 26, 25]]
        ];

        check(&expected, &control, |p| SmallChunk::rotate90_yz(p));
    }

    fn get_small_chunk() -> SmallChunk {
        let up = array![
            [[1, 10, 19], [2, 11, 20], [3, 12, 21]],
            [[4, 13, 22], [5, 14, 23], [6, 15, 24]],
            [[7, 16, 25], [8, 17, 26], [9, 18, 27]]
        ]
        .map(|v| Voxel::from(*v as u16));
        let mut upch = SmallChunk::new();
        for x in 0..SMALLCH {
            for y in 0..SMALLCH {
                for z in 0..SMALLCH {
                    upch.data_mut()[(x, y, z)] = up[(x, y, z)];
                }
            }
        }
        upch
    }

    fn ch_index_to_arru16(ch: &SmallChunk, ax: Axis, index: usize) -> Array2<u16> {
        let data = dbg!(ch.data.map(|v| v.id));
        let slice = data.index_axis(ax, index).to_owned();
        slice
    }

    #[rstest(
        dir,
        axis,
        other_index,
        this_index,
        case::up(Directions::UP, 1, 1, 4),
        case::down(Directions::DOWN, 1, 3, 0),
        case::west(Directions::WEST, 0, 3, 0),
        case::east(Directions::EAST, 0, 1, 4),
        case::north(Directions::NORTH, 2, 3, 0),
        case::south(Directions::SOUTH, 2, 1, 4)
    )]
    fn copy_face(dir: Directions, axis: usize, other_index: usize, this_index: usize) {
        let otherch = get_small_chunk();

        let mut this = SmallChunk::new();
        this.copy_borders(&otherch, dir);

        let expected = ch_index_to_arru16(&otherch, Axis(axis), other_index);
        let actual = ch_index_to_arru16(&this, Axis(axis), this_index);

        assert_eq!(expected, actual);
    }

    fn ch_index_index_to_arru16(
        ch: &SmallChunk,
        ax1: Axis,
        index1: usize,
        ax2: Axis,
        index2: usize,
    ) -> Array1<u16> {
        let data = ch.data.map(|v| v.id);
        let slice = data
            .index_axis(ax1, index1)
            .index_axis(ax2, index2)
            .to_owned();
        slice
    }

    #[test]
    fn copy_edge() {
        let otherch = get_small_chunk();

        let mut this = SmallChunk::new();
        this.copy_borders(&otherch, Directions::UP | Directions::WEST);

        let expected = ch_index_index_to_arru16(&otherch, Axis(1), 1, Axis(0), 3);
        let actual = ch_index_index_to_arru16(&this, Axis(1), 4, Axis(0), 0);

        assert_eq!(expected, actual);
    }
}
