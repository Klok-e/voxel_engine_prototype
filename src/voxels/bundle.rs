use bevy::prelude::Plugin;

use super::{
    chunk::CHSIZE,
    resources::EntityChunks,
    systems::{
        chunk_render::chunk_render_system, destroy_on_touch_system::destroy_on_touch_system,
        dirty_around_system::dirty_around_system,
        generate_map_around_system::generate_map_around_system,
        world_change_apply_system::world_apply_changes_system,
    },
    terrain_generation::ProceduralGenerator,
    world::VoxelWorld,
};

#[derive(Debug, Default)]
pub struct VoxelBundle;

impl Plugin for VoxelBundle {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(VoxelWorld::new(ProceduralGenerator::<CHSIZE>::new(42)));
        app.insert_resource(EntityChunks::default());

        app.add_system(generate_map_around_system);
        app.add_system(destroy_on_touch_system);
        app.add_system(dirty_around_system);
        app.add_system(world_apply_changes_system);
        app.add_system(chunk_render_system);
    }
}
