use bevy::{
    prelude::{Component, Query, Res, With},
    text::Text,
};

use crate::voxels::{
    chunk::ChunkPosition, systems::components::RenderedTag, world::VoxelWorldProcedural,
};

#[derive(Component)]
pub struct ChunkCountersText;

pub fn chunk_counter_ui_system(
    voxel_world: Res<VoxelWorldProcedural>,
    mut ui_text: Query<&mut Text, With<ChunkCountersText>>,
    rend_chunks: Query<&ChunkPosition, With<RenderedTag>>,
) {
    let mut text = ui_text.single_mut();

    text.sections[1].value = format!("{:.2}", voxel_world.chunks().len());
    text.sections[3].value = format!("{:.2}", voxel_world.dirty().len());
    text.sections[5].value = format!("{:.2}", rend_chunks.iter().count());
}
