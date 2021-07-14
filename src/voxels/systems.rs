use amethyst::ecs::SystemBundle;
use chunk_per_frame_system::chunk_per_frame_system;
use chunk_render::chunk_render_system;
use destroy_on_touch_system::destroy_on_touch_system;
use dirty_around_system::dirty_around_system;
use generate_map_around_system::generate_map_around_system;
use world_change_apply_system::world_apply_changes_system;

pub mod chunk_per_frame_system;
pub mod chunk_render;
pub mod destroy_on_touch_system;
pub mod dirty_around_system;
pub mod generate_map_around_system;
pub mod world_change_apply_system;

#[derive(Debug, Default)]
pub struct VoxelBundle;

impl SystemBundle for VoxelBundle {
    fn load(
        &mut self,
        _world: &mut amethyst::ecs::World,
        _resources: &mut amethyst::ecs::Resources,
        builder: &mut amethyst::ecs::DispatcherBuilder,
    ) -> Result<(), amethyst::Error> {
        builder.add_system(|| generate_map_around_system());
        builder.add_system(|| destroy_on_touch_system());
        builder.add_system(|| world_apply_changes_system());
        builder.add_system(|| dirty_around_system());
        builder.add_system(|| chunk_render_system());
        builder.add_system(|| chunk_per_frame_system());

        Ok(())
    }
}
