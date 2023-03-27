use bevy::{
    prelude::{Camera3d, Component, Query, Res, Transform, With},
    text::Text,
};

use crate::voxels::world::VoxelWorldProcedural;

#[derive(Component)]
pub struct CurrentChunkInfoText;

pub fn current_chunk_info_system(
    voxel_world: Res<VoxelWorldProcedural>,
    mut ui_text: Query<&mut Text, With<CurrentChunkInfoText>>,
    camera: Query<&Transform, (With<Camera3d>,)>,
) {
    let transform = camera.single();
    let (curr_chpos, _) = VoxelWorldProcedural::to_ch_pos_index(&transform.translation);
    if let Some(chunk) = voxel_world.get_chunk_at(&curr_chpos) {
        let mut text = ui_text.single_mut();
        text.sections[1].value = format!(
            "transp: {}; nontransp: {}",
            chunk.is_transparent(),
            chunk.is_nontransparent()
        );
    }
}
