use amethyst::{
    core::{
        math,
        math::geometry::{Rotation, Rotation3},
        Transform,
    },
    prelude::*,
};

use crate::game_config::GameConfig;
use crate::{camera_move_system::init_camera, ui::init_fps_counter, voxels::create_cube};
use amethyst::input::StringBindings;
use amethyst::window::ScreenDimensions;

pub struct GameplayState {}

impl SimpleState for GameplayState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        init_camera(data.world);
        init_fps_counter(data.world);
        let mut transform = Transform::default();
        transform.set_translation_xyz(0., 0., 0.);
        create_cube(data.world, transform);

        data.world.insert(GameConfig {
            chunks_render_per_frame: 20,
        })
    }
}
