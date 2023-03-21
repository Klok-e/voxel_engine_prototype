use crate::{
    core::ConvertVecExtension,
    game_config::RuntimeGameConfig,
    voxels::{
        chunk::{ChunkPosition, CHSIZE},
        world::VoxelWorldProcedural,
    },
};

use bevy::prelude::{Component, IVec3, Query, Res, Transform, With};
use flurry::epoch::pin;
use nalgebra::Vector3;
use std::collections::HashSet;

#[derive(Component)]
pub struct RenderedTag;

#[derive(Component)]
pub struct RenderAround;

pub fn dirty_around_system(
    vox_world: Res<VoxelWorldProcedural>,
    config: Res<RuntimeGameConfig>,
    render_bubbles: Query<&Transform, (With<Transform>, With<RenderAround>)>,
    rendered_chunks: Query<&ChunkPosition, (With<ChunkPosition>, With<RenderedTag>)>,
) {
    let mut loaded_chunks = HashSet::new();
    let mut chunks_to_load = HashSet::new();
    for transform in render_bubbles.iter() {
        let pos = transform.translation / CHSIZE as f32;
        let pos = Vector3::<i32>::new(
            pos.x.floor() as i32,
            pos.y.floor() as i32,
            pos.z.floor() as i32,
        );

        for chunk_pos in rendered_chunks.iter() {
            loaded_chunks.insert(*chunk_pos);
        }

        let render_around = config.config.render_around_bubble as i32;
        for z in -render_around..=render_around {
            for y in -render_around..=render_around {
                for x in -render_around..=render_around {
                    let convert_vec: IVec3 = pos.convert_vec();
                    let pos = ChunkPosition::new(IVec3::new(x, y, z) + convert_vec);
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
