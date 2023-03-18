use bevy::prelude::ResMut;

use crate::voxels::world::VoxelWorldProcedural;

pub fn world_apply_changes_system(mut vox_world: ResMut<VoxelWorldProcedural>) {
    vox_world.apply_voxel_changes();
}
