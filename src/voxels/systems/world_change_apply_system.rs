use crate::voxels::world::VoxelWorldProcedural;
use amethyst::ecs::{Runnable, SubWorld};

use amethyst::ecs::SystemBuilder;

pub fn world_apply_changes_system() -> impl Runnable {
    SystemBuilder::new("world_apply_changes")
        .write_resource::<VoxelWorldProcedural>()
        .build(move |_, world, resources, _query| world_apply_changes(world, resources))
}

fn world_apply_changes(_w: &mut SubWorld, vox_world: &mut VoxelWorldProcedural) {
    vox_world.apply_voxel_changes();
}
