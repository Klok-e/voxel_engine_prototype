use crate::{
    core::Vec3i,
    game_config::GameConfig,
    voxels::{chunk::ChunkPosition, world::VoxelWorldProcedural},
};
use amethyst::{
    core::Transform,
    ecs::{Runnable, SubWorld},
};
use flurry::epoch::pin;
use legion::{
    component,
    query::{And, ComponentFilter, EntityFilterTuple, Passthrough, Query},
    IntoQuery, SystemBuilder,
};

pub struct GenerateMapAround;

pub fn generate_map_around_system() -> impl Runnable {
    SystemBuilder::new("generate_map_around")
        .read_resource::<VoxelWorldProcedural>()
        .read_resource::<GameConfig>()
        .with_query(<&Transform>::query().filter(component::<GenerateMapAround>()))
        .build(move |_, world, resources, query| {
            generate_map_around(world, &resources.0, &resources.1, query)
        })
}

fn generate_map_around(
    w: &mut SubWorld,
    vox_world: &VoxelWorldProcedural,
    config: &GameConfig,
    q1: &mut Query<
        &Transform,
        EntityFilterTuple<
            And<(
                ComponentFilter<Transform>,
                ComponentFilter<GenerateMapAround>,
            )>,
            Passthrough,
        >,
    >,
) {
    let guard = pin();
    let mut generated = 0;
    'outer: for transform in q1.iter(w) {
        let (pos, _) = VoxelWorldProcedural::to_ch_pos_index(transform.translation());
        let render_around = config.render_around_bubble as i32;
        for z in -render_around..=render_around {
            for y in -render_around..=render_around {
                for x in -render_around..=render_around {
                    let pos = ChunkPosition::new(pos.pos + Vec3i::from([x, y, z]));
                    match vox_world.chunk_at(&pos, &guard) {
                        Some(_) => {}
                        None => {
                            vox_world.chunk_at_or_create(&pos, &guard);
                            generated += 1;
                            if generated > config.chunks_generate_per_frame {
                                break 'outer;
                            }
                        }
                    };
                }
            }
        }
    }
}
