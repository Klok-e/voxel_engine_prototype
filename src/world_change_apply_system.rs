use crate::voxels::{voxel::Voxel, world::VoxelWorld};
use amethyst::core::Transform;
use amethyst::prelude::*;
use amethyst::{derive::SystemDesc, ecs::prelude::*};
use flurry::epoch::pin;

#[derive(SystemDesc)]
pub struct WorldApplyChangesSystem;

impl<'a> System<'a> for WorldApplyChangesSystem {
    type SystemData = (
        ReadExpect<'a, VoxelWorld>,
    );

    fn run(&mut self, (world,): Self::SystemData) {
        let guard = pin();
        world.apply_voxel_changes(&guard);
    }
}
