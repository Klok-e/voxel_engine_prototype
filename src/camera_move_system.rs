use std::{cell::RefCell, ops::DerefMut, rc::Rc};

use crate::core::Vec3f;
use amethyst::{
    core::{dispatcher::ThreadLocalSystem, math, Transform},
    ecs::{system, IntoQuery, Runnable, SubWorld, SystemBundle},
    input::{InputEvent, InputHandler, VirtualKeyCode},
    renderer::Camera,
    shrev::{EventChannel, ReaderId},
};

type GameInputEvent = InputEvent;

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

#[system]
#[read_component(Camera)]
#[write_component(Transform)]
fn camera_move(
    world: &mut SubWorld,
    #[resource] input: &InputHandler,
    #[resource] events: &EventChannel<GameInputEvent>,
    #[resource] sensitivity: &CameraMoveSensitivity,
    #[state] readerid: &mut ReaderId<GameInputEvent>,
) {
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

    let (mut d_x, mut d_y) = (0., 0.);
    for event in events.read(readerid) {
        if let InputEvent::MouseMoved { delta_x, delta_y } = *event {
            d_x -= delta_x;
            d_y -= delta_y;
        }
    }

    let mut q = <(&Camera, &mut Transform)>::query();
    for (_, cam_trans) in q.iter_mut(world) {
        cam_trans.append_translation(translation * sensitivity.translation);
        cam_trans.append_rotation_x_axis(d_y * sensitivity.mouse);
        cam_trans.append_rotation_y_axis(d_x * sensitivity.mouse);

        let right = cam_trans.rotation() * Vec3f::x();
        let angle = right.angle(&Vec3f::y()) - std::f32::consts::FRAC_PI_2;
        cam_trans.append_rotation_z_axis(-angle);
    }
}

#[derive(Default)]
pub struct ControlsBundle;

impl SystemBundle for ControlsBundle {
    fn load(
        &mut self,
        world: &mut legion::World,
        resources: &mut legion::Resources,
        builder: &mut amethyst::ecs::DispatcherBuilder,
    ) -> Result<(), amethyst::Error> {
        let readerid = resources
            .get_mut::<EventChannel<GameInputEvent>>()
            .unwrap()
            .register_reader();

        builder.add_thread_local(Box::new(|| camera_move_system(readerid)));
        Ok(())
    }
}
