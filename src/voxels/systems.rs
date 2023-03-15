use bevy::prelude::Plugin;
use chunk_per_frame_system::chunk_per_frame_system;
use chunk_render::chunk_render_system;
use destroy_on_touch_system::destroy_on_touch_system;
use dirty_around_system::dirty_around_system;
use generate_map_around_system::generate_map_around_system;
use world_change_apply_system::world_apply_changes_system;

use super::{chunk::CHSIZE, terrain_generation::ProceduralGenerator, world::VoxelWorld};

pub mod chunk_per_frame_system;
pub mod chunk_render;
pub mod destroy_on_touch_system;
pub mod dirty_around_system;
pub mod generate_map_around_system;
pub mod world_change_apply_system;

#[derive(Debug, Default)]
pub struct VoxelBundle;

impl Plugin for VoxelBundle {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(VoxelWorld::new(ProceduralGenerator::<CHSIZE>::new(42)));

        app.add_system(|| generate_map_around_system());
        app.add_system(|| destroy_on_touch_system());
        app.add_system(|| world_apply_changes_system());
        app.add_system(|| dirty_around_system());
        app.add_system(|| chunk_render_system());
        app.add_system(|| chunk_per_frame_system());
    }
}
