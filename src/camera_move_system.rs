use bevy::{
    input::mouse::MouseMotion,
    math,
    prelude::{
        Camera3d, EulerRot, EventReader, Input, KeyCode, Quat, Query, Res, Resource, Transform,
        With,
    },
    window::{CursorGrabMode, Window},
};

#[derive(Resource)]
pub struct CameraMoveSensitivity {
    pub mouse: f32,
    pub translation: f32,
    pub boost_translation: f32,
}
impl Default for CameraMoveSensitivity {
    fn default() -> Self {
        Self {
            mouse: 0.001,
            translation: 0.1,
            boost_translation: 10.,
        }
    }
}

pub fn camera_move_system(
    mut cameras: Query<&mut Transform, With<Camera3d>>,
    keyboard: Res<Input<KeyCode>>,
    sensitivity: Res<CameraMoveSensitivity>,
    mut cursor_moved_events: EventReader<MouseMotion>,
    mut windows: Query<&mut Window>,
) {
    let mut window = windows.single_mut();
    window.cursor.visible = false;
    window.cursor.grab_mode = CursorGrabMode::Locked;

    let mut translation = math::Vec3::ZERO;
    if keyboard.pressed(KeyCode::W) {
        translation -= math::Vec3::Z;
    }
    if keyboard.pressed(KeyCode::S) {
        translation += math::Vec3::Z;
    }
    if keyboard.pressed(KeyCode::A) {
        translation -= math::Vec3::X;
    }
    if keyboard.pressed(KeyCode::D) {
        translation += math::Vec3::X;
    }
    if keyboard.pressed(KeyCode::V) {
        translation += math::Vec3::Y;
    }
    if keyboard.pressed(KeyCode::C) {
        translation -= math::Vec3::Y;
    }
    let boost = if keyboard.pressed(KeyCode::LShift) {
        sensitivity.boost_translation
    } else {
        1.
    };

    let mut delta = math::Vec2::ZERO;
    for event in cursor_moved_events.iter() {
        delta += event.delta;
    }

    for mut cam_trans in cameras.iter_mut() {
        let vec3 = cam_trans.rotation * translation;
        cam_trans.translation += vec3 * sensitivity.translation * boost;

        let mut angles = cam_trans.rotation.to_euler(EulerRot::default());
        angles.2 = 0.0;
        cam_trans.rotation = Quat::from_euler(EulerRot::default(), angles.0, angles.1, angles.2);
        cam_trans.rotate_local_x(sensitivity.mouse * -delta.y);
        cam_trans.rotate_local_y(sensitivity.mouse * -delta.x);
    }
}
