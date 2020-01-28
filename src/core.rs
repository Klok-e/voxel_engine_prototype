use amethyst::{core::math::Vector3, prelude::*, utils::application_root_dir};
use std::path::PathBuf;

pub type Vec3f = Vector3<f32>;
pub type Vec3i = Vector3<i32>;

lazy_static! {
    pub static ref APP_ROOT: PathBuf = application_root_dir().unwrap();
}
