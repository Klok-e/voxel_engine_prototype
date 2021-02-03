use crate::{
    core::Vec3i,
    voxels::{
        chunk::{ChunkPosition, CHSIZE},
        world::VoxelWorldProcedural,
    },
};
use amethyst::{
    core::Transform,
    ecs::{IntoQuery, Runnable, SubWorld},
};
use flurry::epoch::pin;
use legion::{query::Query, SystemBuilder};
use std::collections::HashSet;

pub struct RenderAround {
    pub distance: i32,
}

impl RenderAround {
    pub fn new(distance: i32) -> Self {
        RenderAround { distance }
    }
}

pub fn dirty_around_system() -> impl Runnable {
    SystemBuilder::new("dirty_around_system")
        .read_resource::<VoxelWorldProcedural>()
        .with_query(<(&RenderAround, &Transform)>::query())
        .with_query(<(&ChunkPosition,)>::query())
        .build(move |_, world, resources, query| {
            dirty_around(world, resources, &mut query.0, &mut query.1)
        })
}

fn dirty_around(
    w: &mut SubWorld,
    vox_world: &VoxelWorldProcedural,
    q1: &mut Query<(&RenderAround, &Transform)>,
    chunk_positions: &mut Query<(&ChunkPosition,)>,
) {
    let mut loaded_chunks = HashSet::new();
    let mut chunks_to_load = HashSet::new();
    for (loader, transform) in q1.iter(w) {
        let pos = transform.translation() / CHSIZE as f32;
        let pos = Vec3i::new(
            pos.x.floor() as i32,
            pos.y.floor() as i32,
            pos.z.floor() as i32,
        );

        for (chunk_pos,) in chunk_positions.iter(w) {
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
        vox_world.dirty().insert(*to_load_pos, &guard);
    }
}
