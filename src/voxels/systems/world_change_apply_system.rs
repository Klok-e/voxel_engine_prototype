use crate::voxels::world::VoxelWorldProcedural;
use amethyst::ecs::{Runnable, SubWorld};
use flurry::epoch::pin;
use legion::SystemBuilder;

pub fn world_apply_changes_system() -> impl Runnable {
    SystemBuilder::new("world_apply_changes")
        .read_resource::<VoxelWorldProcedural>()
        .build(move |_, world, resources, query| world_apply_changes(world, resources))
}

fn world_apply_changes(w: &mut SubWorld, vox_world: &VoxelWorldProcedural) {
    let guard = pin();
    vox_world.apply_voxel_changes(&guard);
}
