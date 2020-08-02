use amethyst::{core::math::Vector3, prelude::*, utils::application_root_dir};
use flurry;
use std::path::PathBuf;

pub type Vec3f = Vector3<f32>;
pub type Vec3i = Vector3<i32>;
pub type ConcurrentHashMap<K, V> = flurry::HashMap<K, V>;
pub type ConcurrentHashSet<T> = flurry::HashSet<T>;

lazy_static! {
    pub static ref APP_ROOT: PathBuf = application_root_dir().unwrap();
}
