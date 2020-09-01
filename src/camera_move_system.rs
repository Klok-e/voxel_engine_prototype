use crate::{
    core::Vec3f, destroy_on_touch_system::DestroyVoxOnTouch,
    voxels::{dirty_around_system::RenderAround,generate_map_around_system::GenerateMapAround},
};
use amethyst::{
    core::{math, Transform},
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
    input::{InputEvent, InputHandler, StringBindings, VirtualKeyCode},
    prelude::*,
    renderer::{
        light::{DirectionalLight, Light},
        palette::Srgb,
        Camera,
    },
    shrev::{EventChannel, ReaderId},
    utils::auto_fov::AutoFov,
};

type GameInputEvent = InputEvent<StringBindings>;

#[derive(SystemDesc, Default)]
pub struct CameraMoveSystem {
    readerid: Option<ReaderId<GameInputEvent>>,
}

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

impl<'a> System<'a> for CameraMoveSystem {
    type SystemData = (
        Read<'a, InputHandler<StringBindings>>,
        ReadStorage<'a, Camera>,
        WriteStorage<'a, Transform>,
        Read<'a, EventChannel<GameInputEvent>>,
        Read<'a, CameraMoveSensitivity>,
    );

    fn run(&mut self, (input, cameras, mut transforms, events, sensitivity): Self::SystemData) {
        let mut translation = math::Vector3::<f32>::zeros();
        if input.key_is_down(VirtualKeyCode::W) {
            println!("W down");
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
        for event in events.read(self.readerid.as_mut().unwrap()) {
            if let InputEvent::MouseMoved { delta_x, delta_y } = *event {
                d_x -= delta_x;
                d_y -= delta_y;
            }
        }

        for (cam_trans, _) in (&mut transforms, &cameras).join() {
            cam_trans.append_translation(translation * sensitivity.translation);
            cam_trans.append_rotation_x_axis(d_y * sensitivity.mouse);
            cam_trans.append_rotation_y_axis(d_x * sensitivity.mouse);

            let right = cam_trans.rotation() * Vec3f::x();
            let angle = right.angle(&Vec3f::y()) - std::f32::consts::FRAC_PI_2;
            cam_trans.append_rotation_z_axis(-angle);
        }
    }

    fn setup(&mut self, world: &mut World) {
        <Self as System<'_>>::SystemData::setup(world);
        self.readerid = Some(
            world
                .fetch_mut::<EventChannel<GameInputEvent>>()
                .register_reader(),
        );
    }
}

pub fn init_camera(world: &mut World) {
    world.register::<RenderAround>();

    let mut light = DirectionalLight::default();
    light.color = Srgb::new(1., 1., 1.);
    world
        .create_entity()
        .with(Light::Directional(light))
        .build();

    let mut transform = Transform::default();
    transform.set_translation_xyz(0., 0., 2.);

    world
        .create_entity()
        .with(Camera::standard_3d(10., 10.))
        .with(AutoFov::new())
        .with(transform)
        .with(RenderAround::new(1))
        .with(GenerateMapAround::new(10))
        .with(DestroyVoxOnTouch)
        .build();
}
