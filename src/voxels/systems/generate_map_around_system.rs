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
use legion::{query::Query, IntoQuery, SystemBuilder};

pub struct GenerateMapAround {
    pub distance: i32,
}

impl GenerateMapAround {
    pub fn new(distance: i32) -> Self {
        Self { distance }
    }
}

fn generate_map_around_system() -> impl Runnable {
    SystemBuilder::new("generate_map_around")
        .read_resource::<VoxelWorldProcedural>()
        .read_resource::<GameConfig>()
        .with_query(<(&GenerateMapAround, &Transform)>::query())
        .build(move |_, world, resources, query| {
            generate_map_around(world, &resources.0, &resources.1, query)
        })
}

fn generate_map_around(
    w: &mut SubWorld,
    vox_world: &VoxelWorldProcedural,
    config: &GameConfig,
    q1: &mut Query<(&GenerateMapAround, &Transform)>,
) {
    let guard = pin();
    let mut generated = 0;
    'outer: for (around, transform) in q1.iter(w) {
        let (pos, _) = VoxelWorldProcedural::to_ch_pos_index(transform.translation());
        for z in -around.distance..=around.distance {
            for y in -around.distance..=around.distance {
                for x in -around.distance..=around.distance {
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
