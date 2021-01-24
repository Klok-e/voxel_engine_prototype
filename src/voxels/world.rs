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
use flurry::epoch::Guard;
use std::{
    collections::{HashSet, VecDeque},
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

pub struct VoxelWorld<G: VoxelGenerator<N>, const N: usize> {
    chunks: ConcurrentHashMap<ChunkPosition, RwLock<Chunk<N>>>,
    chunk_changes: ConcurrentHashMap<ChunkPosition, Mutex<VecDeque<VoxChange>>>,
    dirty: ConcurrentHashSet<ChunkPosition>,
    procedural: G,
}

impl<G: VoxelGenerator<N>, const N: usize> VoxelWorld<G, N> {
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

    pub fn chunks(&self) -> &ConcurrentHashMap<ChunkPosition, RwLock<Chunk<N>>> {
        &self.chunks
    }

    pub fn dirty(&self) -> &ConcurrentHashSet<ChunkPosition> {
        &self.dirty
    }

    pub fn chunk_at<'a>(
        &'a self,
        pos: &ChunkPosition,
        guard: &'a Guard,
    ) -> Option<&'a RwLock<Chunk<N>>> {
        self.chunks.get(pos, guard)
    }

    pub fn chunk_at_or_create<'a>(
        &'a self,
        pos: &ChunkPosition,
        guard: &'a Guard,
    ) -> &'a RwLock<Chunk<N>> {
        let chunk = self.chunk_at(pos, guard).unwrap_or_else(|| {
            // or create and insert a new chunk
            let mut c = Chunk::<N>::new();
            self.procedural.fill_random(&pos, &mut c.data_mut());
            self.chunks.try_insert(*pos, RwLock::new(c), guard).unwrap();
            self.chunks.get(pos, guard).unwrap()
        });
        chunk
    }

    pub fn voxel_at_pos(&self, pos: &Vec3f, guard: &Guard) -> Voxel {
        let (ch, ind) = Self::to_ch_pos_index(pos);
        self.voxel_at(&ch, &ind, guard)
    }
    pub fn voxel_at(&self, chunk: &ChunkPosition, ind: &[usize; 3], guard: &Guard) -> Voxel {
        let chunk = self.chunk_at_or_create(chunk, guard).read().unwrap();
        chunk.data()[*ind]
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
        let mut ch_list = ch_list.try_lock().unwrap();
        ch_list.push_back(VoxChange::new(*ind, new_vox));
    }

    pub fn mesh(&self, chpos: &ChunkPosition, guard: &Guard) -> ChunkMeshData {
        let onef: Vec3f = [1., 1., 1.].into();

        let chunk = self.chunk_at_or_create(chpos, guard).try_read().unwrap();
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
                                .chunk_at_or_create(&ChunkPosition::new(chpos.pos + dir_vec), guard)
                                .try_read()
                                .unwrap()
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

        chunk_mesh
    }

    pub fn apply_voxel_changes(&self, guard: &Guard) {
        let mut borders_changed = HashSet::new();

        // TODO: when flurry supports rayon use parallel iterators
        self.chunk_changes.iter(guard).for_each(|(pos, list)| {
            let mut chunk = self.chunk_at_or_create(pos, guard).try_write().unwrap();
            let mut list = list.try_lock().unwrap();
            list.iter().for_each(|change| {
                chunk.data_mut()[change.index] = change.new_vox;

                self.dirty.insert(*pos, guard);

                // if on a border
                let border = Chunk::<N>::is_on_border(&change.index);
                if let Some(border_dir) = border {
                    borders_changed.insert((*pos, border_dir));
                }
            });
            list.clear()
        });

        for (chunk_pos, copy_to_dir) in borders_changed {
            let curr_chunk = self
                .chunk_at_or_create(&chunk_pos, guard)
                .try_read()
                .unwrap();

            let copy_to_vec = copy_to_dir.to_vec::<i32>();
            let next_chunk_pos = chunk_pos.pos + copy_to_vec;
            let mut next_chunk = self
                .chunk_at_or_create(
                    &ChunkPosition {
                        pos: next_chunk_pos,
                    },
                    guard,
                )
                .try_write()
                .unwrap();

            next_chunk.copy_borders(&*curr_chunk, copy_to_dir.invert());

            self.dirty.insert(
                ChunkPosition {
                    pos: next_chunk_pos,
                },
                guard,
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
