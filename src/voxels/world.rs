use super::{
    chunk::{Chunk, ChunkPosition, CHSIZE},
    chunk_mesh::ChunkMeshData,
    terrain_generation::{ProceduralGenerator, VoxelGenerator},
    voxel::Voxel,
};
use crate::{
    core::{ConvertVecExtension, VecExtensions},
    directions::Directions,
};
use bevy::prelude::{IVec3, Resource, Vec3};

use std::{
    collections::{HashMap, VecDeque},
    sync::Mutex,
};

#[derive(Debug, Copy, Clone)]
pub struct VoxChange {
    pub new_vox: Voxel,
    pub index: [usize; 3],
}

impl VoxChange {
    pub fn new(index: [usize; 3], new_vox: Voxel) -> Self {
        Self { new_vox, index }
    }
}

pub type VoxelWorldProcedural = VoxelWorld<ProceduralGenerator<CHSIZE>, CHSIZE>;

#[derive(Resource)]
pub struct VoxelWorld<G, const N: usize> {
    chunks: HashMap<ChunkPosition, Chunk<N>>,
    chunk_changes: flurry::HashMap<ChunkPosition, Mutex<VecDeque<VoxChange>>>,
    dirty: flurry::HashSet<ChunkPosition>,
    procedural: G,
}

impl<G, const N: usize> VoxelWorld<G, N>
where
    G: VoxelGenerator<N> + Send + Sync,
{
    const NI: i32 = N as i32;
    const NF: f32 = N as f32;

    pub fn new(generator: G) -> Self {
        Self {
            chunks: Default::default(),
            chunk_changes: Default::default(),
            dirty: Default::default(),
            procedural: generator,
        }
    }

    pub fn chunks(&self) -> &HashMap<ChunkPosition, Chunk<N>> {
        &self.chunks
    }

    pub fn dirty(&self) -> &flurry::HashSet<ChunkPosition> {
        &self.dirty
    }

    pub fn chunk_changes(&self) -> &flurry::HashMap<ChunkPosition, Mutex<VecDeque<VoxChange>>> {
        &self.chunk_changes
    }

    pub fn get_chunk_at<'a>(&'a self, pos: &ChunkPosition) -> Option<&'a Chunk<N>> {
        self.chunks.get(pos)
    }

    pub fn chunk_at<'a>(&'a self, pos: &ChunkPosition) -> &'a Chunk<N> {
        self.chunks.get(pos).expect("Chunk expected.")
    }

    pub fn chunk_at_mut<'a>(&'a mut self, pos: &ChunkPosition) -> Option<&'a mut Chunk<N>> {
        self.chunks.get_mut(pos)
    }

    pub fn gen_chunk(&self, pos: &ChunkPosition) -> Chunk<N> {
        let mut c = Chunk::<N>::new();
        self.procedural.fill_random(pos, c.data_mut());
        c
    }

    pub fn insert_at(&mut self, pos: &ChunkPosition, chunk: Chunk<N>) {
        self.chunks.insert(*pos, chunk);
    }

    pub fn voxel_at_pos(&self, pos: &Vec3) -> Option<Voxel> {
        let (ch, ind) = Self::to_ch_pos_index(pos);
        self.voxel_at(&ch, &ind)
    }
    pub fn voxel_at(&self, chunk: &ChunkPosition, ind: &[usize; 3]) -> Option<Voxel> {
        self.get_chunk_at(chunk).map(|c| c.data()[*ind])
    }

    pub fn set_voxel_at_pos(&self, pos: &Vec3, new_vox: Voxel) {
        let (ch, ind) = Self::to_ch_pos_index(pos);
        self.set_voxel_at(&ch, &ind, new_vox)
    }
    pub fn set_voxel_at(&self, chunk: &ChunkPosition, ind: &[usize; 3], new_vox: Voxel) {
        let chunk_changes = self.chunk_changes.pin();
        let ch_list = match chunk_changes.get(chunk) {
            Some(change_list) => change_list,
            None => {
                chunk_changes
                    .try_insert(*chunk, Mutex::new(VecDeque::new()))
                    .unwrap();
                chunk_changes.get(chunk).unwrap()
            }
        };
        let mut ch_list = ch_list.lock().unwrap();
        ch_list.push_back(VoxChange::new(*ind, new_vox));
    }

    pub fn mesh(&self, chpos: &ChunkPosition) -> ChunkMeshData {
        let onef: Vec3 = [1., 1., 1.].into();

        let chunk = self.chunk_at(chpos);
        let mut chunk_mesh = ChunkMeshData::new();
        for x in 0..Self::NI {
            for y in 0..Self::NI {
                for z in 0..Self::NI {
                    let pos: IVec3 = [x, y, z].into();
                    let convert_vec: [usize; 3] = (pos).to_usize();
                    if chunk.data()[convert_vec].is_transparent() {
                        // if current voxel is transparent
                        continue;
                    }
                    // if current voxel is solid
                    for dir in Directions::all().into_iter() {
                        let dir_vec = dir.to_ivec();
                        let spos = pos + dir_vec;
                        let adj_vox = match Chunk::<N>::chunk_voxel_index_wrap(&spos) {
                            Some(index) => {
                                let convert_vec: [usize; 3] = index.to_usize();
                                self.chunk_at(&ChunkPosition::new(chpos.pos + dir_vec))
                                    .data()[convert_vec]
                            }
                            None => {
                                let convert_vec: [usize; 3] = spos.to_usize();
                                chunk.data()[convert_vec]
                            }
                        };

                        if adj_vox.is_transparent() {
                            // if adjacent voxel is transparent
                            let convert_vec: Vec3 = pos.convert_vec();
                            chunk_mesh.insert_quad(convert_vec + onef / 2., dir);
                        }
                    }
                }
            }
        }

        chunk_mesh
    }

    pub fn apply_voxel_changes(&mut self) {
        let borders_changed = flurry::HashSet::new();
        let borders_changed = borders_changed.pin();

        let chunks = &mut self.chunks;
        let chunk_changes = self.chunk_changes.pin();
        let dirty = self.dirty.pin();

        chunks.iter_mut().for_each(|(pos, chunk)| {
            let changes = match chunk_changes.get(pos) {
                Some(x) => x,
                None => return,
            };
            let mut list = changes.try_lock().unwrap();
            list.iter().for_each(|change| {
                chunk.data_mut()[change.index] = change.new_vox;

                dirty.insert(*pos);

                // if on a border
                let border = Chunk::<N>::is_on_border(&change.index);
                if let Some(border_dir) = border {
                    borders_changed.insert((*pos, border_dir));
                }
            });
            list.clear();
        });

        for (chunk_pos, adj_dir) in borders_changed.iter() {
            let adj_vec = adj_dir.to_ivec();
            let next_chunk_pos = chunk_pos.pos + adj_vec;

            dirty.insert(ChunkPosition {
                pos: next_chunk_pos,
            });
        }
    }

    pub fn to_ch_pos_index(pos: &Vec3) -> (ChunkPosition, [usize; 3]) {
        let posch: Vec3 = *pos / Self::NF;
        let ch_pos = IVec3::new(
            posch.x.floor() as i32,
            posch.y.floor() as i32,
            posch.z.floor() as i32,
        );
        let convert_vec: Vec3 = ch_pos.convert_vec();
        let index: Vec3 = (posch - convert_vec) * Self::NF;
        let index = [
            index.x.floor() as usize,
            index.y.floor() as usize,
            index.z.floor() as usize,
        ];

        (ChunkPosition { pos: ch_pos }, index)
    }
}
