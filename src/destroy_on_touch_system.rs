use crate::voxels::{voxel::Voxel, world::VoxelWorld};
use amethyst::core::Transform;
use amethyst::prelude::*;
use amethyst::{derive::SystemDesc, ecs::prelude::*};
use flurry::epoch::pin;

#[derive(Debug, Default)]
pub struct DestroyVoxOnTouch;

impl Component for DestroyVoxOnTouch {
    type Storage = NullStorage<Self>;
}

#[derive(SystemDesc)]
pub struct DestroyOnTouchSystem;

impl<'a> System<'a> for DestroyOnTouchSystem {
    type SystemData = (
        ReadStorage<'a, DestroyVoxOnTouch>,
        ReadStorage<'a, Transform>,
        ReadExpect<'a, VoxelWorld>,
    );

    fn run(&mut self, (on_touch, transform, world): Self::SystemData) {
        let guard = pin();
        for (_, transform) in (&on_touch, &transform).join() {
            match world.voxel_at_pos(&transform.translation(), &guard) {
                Voxel { id: 0 } => {}
                _ => world.set_voxel_at_pos(&transform.translation(), Voxel { id: 0 }, &guard),
            }
        }
    }

    fn setup(&mut self, world: &mut World) {
        <Self as System>::SystemData::setup(world);

        world.register::<DestroyVoxOnTouch>();
    }
}
