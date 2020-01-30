use super::chunk::Chunk;
use crate::core::{Vec3f, Vec3i};
use crate::voxels::chunk::{ChunkPosition, CHUNK_SIZE};
use amethyst::{core::components::Transform, derive::SystemDesc, ecs::prelude::*, prelude::*};
use std::collections::HashMap;

#[derive(Default)]
struct VoxelWorld {
    pub chunks: HashMap<Vec3i, Entity>,
}

struct LoadAround {
    pub distance: i32,
}

impl Component for LoadAround {
    type Storage = DenseVecStorage<Self>;
}

#[derive(SystemDesc)]
struct ChunksSystem;

impl<'a> System<'a> for ChunksSystem {
    type SystemData = (
        Write<'a, VoxelWorld>,
        WriteStorage<'a, Chunk>,
        ReadStorage<'a, LoadAround>,
        WriteStorage<'a, ChunkPosition>,
        Entities<'a>,
    );

    fn run(
        &mut self,
        (mut voxel_world, mut chunks, load_around, mut positions, ents): Self::SystemData,
    ) {
        for (loader, pos) in (&load_around, &mut positions).join() {
            for z in -loader.distance..=loader.distance {
                for y in -loader.distance..=loader.distance {
                    for x in -loader.distance..=loader.distance {
                        let coord = Vec3i::from([x, y, z]) + pos.pos.clone();
                        let chunk_ent = *voxel_world.chunks.entry(coord).or_insert_with(|| {
                            let ent = ents.create();
                            chunks.insert(ent, Chunk::new());
                            ent
                        });
                        if chunks
                            .get_mut(chunk_ent)
                            .and_then(|c| Some(c.maintain(true)))
                            .is_none()
                        {
                            log::warn!("Couldn't insert or create chunk!")
                        }
                    }
                }
            }
        }
        for (ent, pos, chunk) in (&ents, &positions, &mut chunks).join() {
            if chunk.maintained {
                chunk.maintain(false);
            } else {
                voxel_world.chunks.remove(&pos.pos);
                ents.delete(ent);
            }
        }
    }

    fn setup(&mut self, world: &mut World) {
        world.register::<Chunk>();
        world.register::<ChunkPosition>();
    }
}
