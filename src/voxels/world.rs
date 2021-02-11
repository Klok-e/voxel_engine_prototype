use super::{
    chunk::{Chunk, ChunkPosition, CHSIZE},
    chunk_mesh::ChunkMeshData,
    terrain_generation::{ProceduralGenerator, VoxelGenerator},
    voxel::Voxel,
};
use crate::{
    core::{to_uarr, to_vecf, ConcurrentHashMap, ConcurrentHashSet, Vec3f, Vec3i},
    directions::Directions,
};
use flurry::epoch::{pin, Guard};
use rayon::prelude::*;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    sync::{Mutex, RwLock},
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

pub struct VoxelWorld<G, const N: usize>
where
    G: VoxelGenerator<N> + Send + Sync,
{
    chunks: HashMap<ChunkPosition, Chunk<N>>,
    chunk_changes: ConcurrentHashMap<ChunkPosition, Mutex<VecDeque<VoxChange>>>,
    dirty: ConcurrentHashSet<ChunkPosition>,
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

    pub fn dirty(&self) -> &ConcurrentHashSet<ChunkPosition> {
        &self.dirty
    }

    pub fn chunk_at<'a>(&'a self, pos: &ChunkPosition) -> Option<&'a Chunk<N>> {
        self.chunks.get(pos)
    }

    pub fn chunk_at_mut<'a>(&'a mut self, pos: &ChunkPosition) -> Option<&'a mut Chunk<N>> {
        self.chunks.get_mut(pos)
    }

    pub fn generate_at<'a>(&'a mut self, pos: &ChunkPosition) {
        // or create and insert a new chunk
        let mut c = Chunk::<N>::new();
        self.procedural.fill_random(&pos, &mut c.data_mut());
        self.chunks.insert(*pos, c);
    }

    pub fn voxel_at_pos(&self, pos: &Vec3f) -> Option<Voxel> {
        let (ch, ind) = Self::to_ch_pos_index(pos);
        self.voxel_at(&ch, &ind)
    }
    pub fn voxel_at(&self, chunk: &ChunkPosition, ind: &[usize; 3]) -> Option<Voxel> {
        self.chunk_at(chunk).map(|c| c.data()[*ind])
    }

    pub fn set_voxel_at_pos(&self, pos: &Vec3f, new_vox: Voxel, guard: &Guard) {
        let (ch, ind) = Self::to_ch_pos_index(pos);
        self.set_voxel_at(&ch, &ind, new_vox, guard)
    }
    pub fn set_voxel_at(
        &self,
        chunk: &ChunkPosition,
        ind: &[usize; 3],
        new_vox: Voxel,
        guard: &Guard,
    ) {
        let ch_list = match self.chunk_changes.get(chunk, guard) {
            Some(change_list) => change_list,
            None => {
                self.chunk_changes
                    .try_insert(*chunk, Mutex::new(VecDeque::new()), guard)
                    .unwrap();
                self.chunk_changes.get(chunk, guard).unwrap()
            }
        };
        let mut ch_list = ch_list.lock().unwrap();
        ch_list.push_back(VoxChange::new(*ind, new_vox));
    }

    pub fn mesh(&self, chpos: &ChunkPosition) -> Option<ChunkMeshData> {
        let onef: Vec3f = [1., 1., 1.].into();

        let chunk = self.chunk_at(chpos)?;
        let mut chunk_mesh = ChunkMeshData::new();
        for x in 0..Self::NI {
            for y in 0..Self::NI {
                for z in 0..Self::NI {
                    let pos: Vec3i = [x, y, z].into();
                    if chunk.data()[to_uarr(pos)].is_transparent() {
                        // if current voxel is transparent
                        continue;
                    }
                    // if current voxel is solid
                    for dir in Directions::all().into_iter() {
                        let dir_vec = dir.to_vec::<i32>();
                        let spos: Vec3i = pos + dir_vec;
                        let adj_vox = match Chunk::<N>::chunk_voxel_index_wrap(&spos) {
                            Some(index) => self
                                .chunk_at(&ChunkPosition::new(chpos.pos + dir_vec))?
                                .data()[to_uarr(index)],
                            None => chunk.data()[to_uarr(spos)],
                        };

                        if adj_vox.is_transparent() {
                            // if adjacent voxel is transparent
                            chunk_mesh.insert_quad(to_vecf(pos) + onef / 2., dir);
                        }
                    }
                }
            }
        }

        Some(chunk_mesh)
    }

    pub fn apply_voxel_changes(&mut self) {
        let borders_changed = ConcurrentHashSet::new();

        let chunks = &mut self.chunks;
        let chunk_changes = &self.chunk_changes;
        let dirty = &self.dirty;

        chunks.par_iter_mut().for_each_init(
            || pin(),
            |guard, (pos, chunk)| {
                let changes = match chunk_changes.get(pos, guard) {
                    Some(x) => x,
                    None => return,
                };
                let mut list = changes.try_lock().unwrap();
                list.iter().for_each(|change| {
                    chunk.data_mut()[change.index] = change.new_vox;

                    dirty.insert(*pos, guard);

                    // if on a border
                    let border = Chunk::<N>::is_on_border(&change.index);
                    if let Some(border_dir) = border {
                        borders_changed.insert((*pos, border_dir), guard);
                    }
                });
                list.clear();
            },
        );

        let guard = pin();
        for (chunk_pos, adj_dir) in borders_changed.iter(&guard) {
            let adj_vec = adj_dir.to_vec::<i32>();
            let next_chunk_pos = chunk_pos.pos + adj_vec;

            self.dirty.insert(
                ChunkPosition {
                    pos: next_chunk_pos,
                },
                &guard,
            );
        }
    }

    pub fn to_ch_pos_index(pos: &Vec3f) -> (ChunkPosition, [usize; 3]) {
        let posch: Vec3f = pos / Self::NF;
        let ch_pos = Vec3i::new(
            posch.x.floor() as i32,
            posch.y.floor() as i32,
            posch.z.floor() as i32,
        );
        let index: Vec3f = (posch - to_vecf(ch_pos)) * Self::NF;
        let index = [
            index.x.floor() as usize,
            index.y.floor() as usize,
            index.z.floor() as usize,
        ];

        (ChunkPosition { pos: ch_pos }, index)
    }
}
