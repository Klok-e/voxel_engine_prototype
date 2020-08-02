use amethyst::{
    core::{
        math,
        math::geometry::{Rotation, Rotation3},
        Transform,
    },
    prelude::*,
};

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
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        use amethyst::winit::{Event, WindowEvent};
        match event {
            StateEvent::Window(Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            }) => {
                let dim = data.world.try_fetch_mut::<ScreenDimensions>();
                dim.map(|mut dim| {
                    let new_dim: (f64, f64) = size.into();
                    dim.update(new_dim.0,new_dim.1);
                }).unwrap_or_else(||{
                    log::
                });
            }
            _ => {}
        }
        SimpleTrans::None
    }
}
