use amethyst::ecs::SystemBundle;
use chunk_render::chunk_render_system;
use destroy_on_touch_system::destroy_on_touch_system;
use dirty_around_system::dirty_around_system;
use generate_map_around_system::generate_map_around_system;
use world_change_apply_system::world_apply_changes_system;

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
        _world: &mut legion::World,
        _resources: &mut legion::Resources,
        builder: &mut amethyst::ecs::DispatcherBuilder,
    ) -> Result<(), amethyst::Error> {
        builder.add_system(Box::new(|| generate_map_around_system()));
        builder.add_system(Box::new(|| destroy_on_touch_system()));
        builder.add_system(Box::new(|| world_apply_changes_system()));
        builder.add_system(Box::new(|| dirty_around_system()));
        builder.add_system(Box::new(|| chunk_render_system()));

        Ok(())
    }
}
