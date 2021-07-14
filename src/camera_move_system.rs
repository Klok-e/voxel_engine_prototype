use crate::core::Vec3f;
use amethyst::ecs::SystemBuilder;
use amethyst::{
    core::{math, Transform},
    ecs::{IntoQuery, Runnable, SystemBundle},
    input::{InputHandler, VirtualKeyCode},
    renderer::Camera,
};

pub struct CameraMoveSensitivity {
    mouse: f32,
    translation: f32,
}
impl Default for CameraMoveSensitivity {
    fn default() -> Self {
        Self {
            mouse: 0.001,
            translation: 0.1,
        }
    }
}

fn camera_move_system() -> impl Runnable {
    SystemBuilder::new("camera_move_system")
        .read_resource::<InputHandler>()
        .read_resource::<CameraMoveSensitivity>()
        .with_query(<(&Camera, &mut Transform)>::query())
        .build(move |_, world, (input, sensitivity), query| {
            let (input, sensitivity): (&InputHandler, &CameraMoveSensitivity) =
                (input, sensitivity);

            let mut translation = math::Vector3::<f32>::zeros();
            if input.key_is_down(VirtualKeyCode::W) {
                translation -= math::Vector3::z();
            }
            if input.key_is_down(VirtualKeyCode::S) {
                translation += math::Vector3::z();
            }
            if input.key_is_down(VirtualKeyCode::A) {
                translation -= math::Vector3::x();
            }
            if input.key_is_down(VirtualKeyCode::D) {
                translation += math::Vector3::x();
            }
            if input.key_is_down(VirtualKeyCode::V) {
                translation += math::Vector3::y();
            }
            if input.key_is_down(VirtualKeyCode::C) {
                translation -= math::Vector3::y();
            }
            if translation.abs().sum() > 1. {
                translation.normalize_mut();
            }

            let d_x = input.axis_value("mouse_x").unwrap();
            let d_y = input.axis_value("mouse_y").unwrap();

            for (_, cam_trans) in query.iter_mut(world) {
                cam_trans.append_translation(translation * sensitivity.translation);
                cam_trans.append_rotation_x_axis(d_y * sensitivity.mouse);
                cam_trans.append_rotation_y_axis(d_x * sensitivity.mouse);

                let right = cam_trans.rotation() * Vec3f::x();
                let angle = right.angle(&Vec3f::y()) - std::f32::consts::FRAC_PI_2;
                cam_trans.append_rotation_z_axis(-angle);
            }
        })
}

#[derive(Default)]
pub struct ControlsBundle;

impl SystemBundle for ControlsBundle {
    fn load(
        &mut self,
        _world: &mut amethyst::prelude::World,
        _resources: &mut amethyst::prelude::Resources,
        builder: &mut amethyst::ecs::DispatcherBuilder,
    ) -> Result<(), amethyst::Error> {
        builder.add_thread_local(Box::new(|| camera_move_system()));
        Ok(())
    }
}
