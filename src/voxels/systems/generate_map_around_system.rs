use crate::{
    core::Vec3i,
    game_config::RuntimeGameConfig,
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
use rayon::prelude::*;

pub struct GenerateMapAround;

pub fn generate_map_around_system() -> impl Runnable {
    SystemBuilder::new("generate_map_around")
        .write_resource::<VoxelWorldProcedural>()
        .read_resource::<RuntimeGameConfig>()
        .with_query(<&Transform>::query().filter(component::<GenerateMapAround>()))
        .build(move |_, world, resources, query| {
            generate_map_around(world, &mut resources.0, &resources.1, query)
        })
}

fn generate_map_around(
    w: &mut SubWorld,
    vox_world: &mut VoxelWorldProcedural,
    config: &RuntimeGameConfig,
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
    // TODO:: consider using binary heap ()
    let mut positions = Vec::new();
    for transform in q1.iter(w) {
        let (pos, _) = VoxelWorldProcedural::to_ch_pos_index(transform.translation());
        let generate_around = config.config.generate_around_bubble as i32;
        for z in -generate_around..=generate_around {
            for y in -generate_around..=generate_around {
                for x in -generate_around..=generate_around {
                    positions.push((x, y, z, pos));
                }
            }
        }
    }
    positions.sort_unstable_by_key(|&(x, y, z, pos)| (pos.pos - Vec3i::from([x, y, z])).abs().sum());
    positions
        .into_par_iter()
        .take(config.chunks_generate_per_frame as usize)
        .flat_map(|(x, y, z, pos)| {
            let pos = ChunkPosition::new(pos.pos + Vec3i::from([x, y, z]));
            match vox_world.chunk_at(&pos) {
                Some(_) => None,
                None => Some((vox_world.gen_chunk(&pos), pos)),
            }
        })
        .collect::<Vec<_>>()
        .into_iter()
        .for_each(|(c, pos)| vox_world.insert_at(&pos, c));
}
