use crate::{
    core::Vec3i,
    game_config::RuntimeGameConfig,
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
use legion::{
    component,
    query::{And, ComponentFilter, EntityFilterTuple, Passthrough, Query},
    SystemBuilder,
};
use std::collections::HashSet;

pub struct RenderedTag;

pub struct RenderAround;

pub fn dirty_around_system() -> impl Runnable {
    SystemBuilder::new("dirty_around_system")
        .read_resource::<VoxelWorldProcedural>()
        .read_resource::<RuntimeGameConfig>()
        .with_query(<&Transform>::query().filter(component::<RenderAround>()))
        .with_query(<&ChunkPosition>::query().filter(component::<RenderedTag>()))
        .build(move |_, world, resources, query| {
            dirty_around(
                world,
                &resources.0,
                &resources.1,
                &mut query.0,
                &mut query.1,
            )
        })
}

fn dirty_around(
    w: &mut SubWorld,
    vox_world: &VoxelWorldProcedural,
    config: &RuntimeGameConfig,
    render_bubbles: &mut Query<
        &Transform,
        EntityFilterTuple<
            And<(ComponentFilter<Transform>, ComponentFilter<RenderAround>)>,
            Passthrough,
        >,
    >,
    rendered_chunks: &mut Query<
        &ChunkPosition,
        EntityFilterTuple<
            And<(ComponentFilter<ChunkPosition>, ComponentFilter<RenderedTag>)>,
            Passthrough,
        >,
    >,
) {
    let mut loaded_chunks = HashSet::new();
    let mut chunks_to_load = HashSet::new();
    for transform in render_bubbles.iter(w) {
        let pos = transform.translation() / CHSIZE as f32;
        let pos = Vec3i::new(
            pos.x.floor() as i32,
            pos.y.floor() as i32,
            pos.z.floor() as i32,
        );

        for chunk_pos in rendered_chunks.iter(w) {
            loaded_chunks.insert(*chunk_pos);
        }

        let render_around = config.config.render_around_bubble as i32;
        for z in -render_around..=render_around {
            for y in -render_around..=render_around {
                for x in -render_around..=render_around {
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
