use super::{
    chunk::{Chunk, ChunkPosition, CHSIZE, CHSIZEF},
    terrain_generation::ProceduralGenerator,
    voxel::Voxel,
};
use crate::core::{to_vecf, ConcurrentHashMap, ConcurrentHashSet, Vec3f, Vec3i};
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

#[derive(Default)]
pub struct VoxelWorld {
    chunks: ConcurrentHashMap<ChunkPosition, RwLock<Chunk<CHSIZE>>>,
    chunk_changes: ConcurrentHashMap<ChunkPosition, Mutex<VecDeque<VoxChange>>>,
    dirty: ConcurrentHashSet<ChunkPosition>,
    procedural: ProceduralGenerator<CHSIZE>,
}

impl VoxelWorld {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn chunks(&self) -> &ConcurrentHashMap<ChunkPosition, RwLock<Chunk<CHSIZE>>> {
        &self.chunks
    }

    pub fn dirty(&self) -> &ConcurrentHashSet<ChunkPosition> {
        &self.dirty
    }

    pub fn chunk_at<'a>(
        &'a self,
        pos: &ChunkPosition,
        guard: &'a Guard,
    ) -> Option<&'a RwLock<Chunk<CHSIZE>>> {
        self.chunks.get(pos, guard)
    }

    pub fn chunk_at_or_create<'a>(
        &'a self,
        pos: &ChunkPosition,
        guard: &'a Guard,
    ) -> &'a RwLock<Chunk<CHSIZE>> {
        let chunk = self.chunk_at(pos, guard).unwrap_or_else(|| {
            // or create and insert a new chunk
            let mut c = Chunk::<CHSIZE>::new();
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
                let border = Chunk::<CHSIZE>::is_on_border(&change.index);
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
        let posch: Vec3f = pos / CHSIZEF;
        let ch_pos = Vec3i::new(
            posch.x.floor() as i32,
            posch.y.floor() as i32,
            posch.z.floor() as i32,
        );
        let index: Vec3f = (posch - to_vecf(ch_pos)) * CHSIZEF;
        let index = [
            index.x.floor() as usize,
            index.y.floor() as usize,
            index.z.floor() as usize,
        ];

        (ChunkPosition { pos: ch_pos }, index)
    }
}
