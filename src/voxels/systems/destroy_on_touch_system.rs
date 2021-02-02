use crate::voxels::{voxel::Voxel, world::VoxelWorldProcedural};
use amethyst::core::Transform;
use amethyst::prelude::*;
use flurry::epoch::pin;
use legion::query::Query;

#[derive(Debug, Default)]
pub struct DestroyVoxOnTouch;

fn destroy_on_touch_system() -> impl Runnable {
    SystemBuilder::new("destroy_on_touch_system")
        .read_resource::<VoxelWorldProcedural>()
        .with_query(<(&DestroyVoxOnTouch, &Transform)>::query())
        .build(move |_, world, resources, query| destroy_on_touch(world, resources, query))
}

fn destroy_on_touch(
    w: &mut SubWorld,
    vox_world: &VoxelWorldProcedural,
    q1: &mut Query<(&DestroyVoxOnTouch, &Transform)>,
) {
    let guard = pin();
    for (_, transform) in q1.iter(w) {
        match vox_world.voxel_at_pos(&transform.translation(), &guard) {
            Voxel { id: 0 } => {}
            _ => vox_world.set_voxel_at_pos(&transform.translation(), Voxel { id: 0 }, &guard),
        }
    }
}
