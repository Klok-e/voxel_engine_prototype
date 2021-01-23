use crate::{
    core::Vec3i,
    directions::Directions,
    voxels::{
        chunk::{ChunkPosition, CHSIZE},
        world::VoxelWorld,
    },
};
use amethyst::{core::components::Transform, derive::SystemDesc, ecs::prelude::*};
use flurry::epoch::pin;
use std::collections::HashSet;

pub struct RenderAround {
    pub distance: i32,
}

impl RenderAround {
    pub fn new(distance: i32) -> Self {
        RenderAround { distance }
    }
}

impl Component for RenderAround {
    type Storage = DenseVecStorage<Self>;
}

#[derive(SystemDesc)]
pub struct DirtyAroundSystem;

impl<'a> System<'a> for DirtyAroundSystem {
    type SystemData = (
        Read<'a, VoxelWorld>,
        ReadStorage<'a, RenderAround>,
        ReadStorage<'a, ChunkPosition>,
        ReadStorage<'a, Transform>,
    );

    fn run(&mut self, (voxel_world, load_around, chunk_positions, transforms): Self::SystemData) {
        let mut loaded_chunks = HashSet::new();
        let mut chunks_to_load = HashSet::new();
        for (loader, transform) in (&load_around, &transforms).join() {
            let pos = transform.translation() / CHSIZE as f32;
            let pos = Vec3i::new(
                pos.x.floor() as i32,
                pos.y.floor() as i32,
                pos.z.floor() as i32,
            );

            for (chunk_pos,) in (&chunk_positions,).join() {
                loaded_chunks.insert(*chunk_pos);
            }

            for z in -loader.distance..=loader.distance {
                for y in -loader.distance..=loader.distance {
                    for x in -loader.distance..=loader.distance {
                        let pos = ChunkPosition::new(Vec3i::new(x, y, z) + pos.clone());
                        chunks_to_load.insert(pos);
                    }
                }
            }
        }

        let guard = pin();
        for to_load_pos in chunks_to_load.difference(&loaded_chunks) {
            let mut chunk = voxel_world
                .chunk_at_or_create(&to_load_pos, &guard)
                .write()
                .unwrap();
            for (dir, dirvec) in Directions::all()
                .into_iter()
                .map(|d| (d, d.to_vec::<i32>()))
            {
                let neighb = voxel_world
                    .chunk_at_or_create(&(to_load_pos.pos + dirvec).into(), &guard)
                    .read()
                    .unwrap();

                chunk.copy_borders(&*neighb, dir);
            }

            voxel_world.dirty().insert(*to_load_pos, &guard);
        }
    }

    fn setup(&mut self, world: &mut World) {
        <Self as System<'_>>::SystemData::setup(world);
        world.register::<RenderAround>();
        world.register::<ChunkPosition>();
    }
}
