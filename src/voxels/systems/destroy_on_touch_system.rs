use crate::voxels::{voxel::Voxel, world::VoxelWorldProcedural};
use bevy::prelude::{Query, Res};

use super::components::DestroyVoxOnTouch;

pub fn destroy_on_touch_system(
    vox_world: Res<VoxelWorldProcedural>,
    q1: Query<(&DestroyVoxOnTouch, &bevy::prelude::Transform)>,
) {
    let guard = vox_world.dirty().guard();
    for (_destr_on_touch, transform) in q1.iter() {
        match vox_world.voxel_at_pos(&transform.translation) {
            Some(Voxel { id: 0 }) => {}
            Some(_) => vox_world.set_voxel_at_pos(&transform.translation, Voxel { id: 0 }, &guard),
            None => {}
        }
    }
}
