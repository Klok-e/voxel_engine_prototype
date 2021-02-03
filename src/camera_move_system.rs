use crate::core::Vec3f;
use amethyst::{
    core::{dispatcher::ThreadLocalSystem, math, Transform},
    ecs::{system, IntoQuery, Runnable, SubWorld, SystemBundle},
    input::{InputEvent, InputHandler, VirtualKeyCode},
    renderer::Camera,
    shrev::{EventChannel, ReaderId},
};
use legion::SystemBuilder;

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

fn camera_move_system(/*mut readerid: ReaderId<InputEvent>*/) -> impl Runnable {
    SystemBuilder::new("camera_move_system")
        .read_resource::<InputHandler>()
        //.read_resource::<EventChannel<InputEvent>>()
        .read_resource::<CameraMoveSensitivity>()
        .with_query(<(&Camera, &mut Transform)>::query())
        .build(move |_, world, (input, /*events,*/ sensitivity), query| {
            let (input, /*events,*/ sensitivity): (
                &InputHandler,
                //&EventChannel<InputEvent>,
                &CameraMoveSensitivity,
            ) = (input, /*events,*/ sensitivity);

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
            // for event in events.read(&mut readerid) {
            //     if let InputEvent::MouseMoved { delta_x, delta_y } = *event {
            //         d_x -= delta_x;
            //         d_y -= delta_y;
            //     }
            // }
            d_x = input.axis_value("mouse_x").unwrap();
            d_y = input.axis_value("mouse_y").unwrap();

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
        _world: &mut legion::World,
        _resources: &mut legion::Resources,
        builder: &mut amethyst::ecs::DispatcherBuilder,
    ) -> Result<(), amethyst::Error> {
        // let readerid = resources
        //     .get_mut::<EventChannel<InputEvent>>()
        //     .unwrap()
        //     .register_reader();

        builder.add_thread_local(Box::new(|| camera_move_system(/*readerid*/)));
        Ok(())
    }
}
