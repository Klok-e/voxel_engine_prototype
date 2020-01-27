use amethyst::{core::math, prelude::*, utils::application_root_dir};
use std::path::PathBuf;

pub type Vec3f = math::Vector3<f32>;

lazy_static! {
    pub static ref APP_ROOT: PathBuf = application_root_dir().unwrap();
}
