use crate::{
    core::ConvertVecExtension,
    game_config::RuntimeGameConfig,
    voxels::{chunk::ChunkPosition, world::VoxelWorldProcedural},
};
use bevy::prelude::{Component, Query, Res, ResMut, Transform, With};
use nalgebra::Vector3;
use rayon::prelude::*;

#[derive(Component)]
pub struct GenerateMapAround;

pub fn generate_map_around_system(
    mut vox_world: ResMut<VoxelWorldProcedural>,
    config: Res<RuntimeGameConfig>,
    q1: Query<&Transform, (With<Transform>, With<GenerateMapAround>)>,
) {
    // TODO:: consider using binary heap ()
    let mut positions = Vec::new();
    for transform in q1.iter() {
        let (pos, _) = VoxelWorldProcedural::to_ch_pos_index(&transform.translation.convert_vec());
        let generate_around = config.config.generate_around_bubble as i32;
        for z in -generate_around..=generate_around {
            for y in -generate_around..=generate_around {
                for x in -generate_around..=generate_around {
                    positions.push((x, y, z, pos));
                }
            }
        }
    }
    positions.sort_unstable_by_key(|&(x, y, z, pos)| {
        (pos.pos - Vector3::<i32>::from([x, y, z])).abs().sum()
    });
    positions
        .into_par_iter()
        .take(config.chunks_generate_per_frame as usize)
        .flat_map(|(x, y, z, pos)| {
            let pos = ChunkPosition::new(pos.pos + Vector3::<i32>::from([x, y, z]));
            match vox_world.chunk_at(&pos) {
                Some(_) => None,
                None => Some((vox_world.gen_chunk(&pos), pos)),
            }
        })
        .collect::<Vec<_>>()
        .into_iter()
        .for_each(|(c, pos)| vox_world.insert_at(&pos, c));
}
