use amethyst::{
    core::math::{Vector2, Vector3},
    utils::application_root_dir,
};
use flurry;
use std::path::PathBuf;

pub type Vec3f = Vector3<f32>;
pub type Vec3i = Vector3<i32>;
pub type Vec2f = Vector2<f32>;
pub type Vec2d = Vector2<f64>;
pub type Vec2i = Vector2<i32>;
pub type ConcurrentHashMap<K, V> = flurry::HashMap<K, V>;
pub type ConcurrentHashSet<T> = flurry::HashSet<T>;

lazy_static! {
    pub static ref APP_ROOT: PathBuf = application_root_dir().unwrap();
}

pub fn to_vecf(veci: Vec3i) -> Vec3f {
    Vec3f::from([veci.x as f32, veci.y as f32, veci.z as f32])
}

pub fn to_vec2f(veci: Vec2i) -> Vec2f {
    Vec2f::from([veci.x as f32, veci.y as f32])
}

pub fn to_vec2d(veci: Vec2i) -> Vec2d {
    Vec2d::from([veci.x as f64, veci.y as f64])
}

pub fn to_uarr(veci: Vec3i) -> [usize; 3] {
    [veci.x as usize, veci.y as usize, veci.z as usize]
}

// pub trait EntityBuildExt: Sized {
//     fn with_opt<T: Component>(self, c: Option<T>, storage: &mut WriteStorage<T>) -> Self;
// }

// impl<'a> EntityBuildExt for EntityResBuilder<'a> {
//     fn with_opt<T: Component>(self, c: Option<T>, storage: &mut WriteStorage<T>) -> Self {
//         match c {
//             Some(c) => self.with(c, storage),
//             None => self,
//         }
//     }
// }
