use bevy::prelude::{IVec3, Vec3};

pub trait ConvertVecExtension<T> {
    fn convert_vec(self) -> T;
}

impl ConvertVecExtension<Vec3> for IVec3 {
    fn convert_vec(self) -> Vec3 {
        [self.x as f32, self.y as f32, self.z as f32].into()
    }
}

pub trait VecExtensions {
    fn to_usize(self) -> [usize; 3];
}

impl VecExtensions for IVec3 {
    fn to_usize(self) -> [usize; 3] {
        self.to_array().map(|x| x as usize)
    }
}
