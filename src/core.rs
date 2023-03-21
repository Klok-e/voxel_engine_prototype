use bevy::prelude::{IVec3, Vec3};
use nalgebra::{Vector2, Vector3};

pub trait ConvertVecExtension<T> {
    fn convert_vec(self) -> T;
}

impl ConvertVecExtension<Vector2<f64>> for Vector2<i32> {
    fn convert_vec(self) -> Vector2<f64> {
        [self.x as f64, self.y as f64].into()
    }
}

impl ConvertVecExtension<Vector3<f32>> for Vector3<i32> {
    fn convert_vec(self) -> Vector3<f32> {
        [self.x as f32, self.y as f32, self.z as f32].into()
    }
}

impl ConvertVecExtension<Vec3> for IVec3 {
    fn convert_vec(self) -> Vec3 {
        [self.x as f32, self.y as f32, self.z as f32].into()
    }
}

impl ConvertVecExtension<Vec3> for Vector3<i32> {
    fn convert_vec(self) -> Vec3 {
        [self.x as f32, self.y as f32, self.z as f32].into()
    }
}

impl ConvertVecExtension<IVec3> for Vector3<i32> {
    fn convert_vec(self) -> IVec3 {
        [self.x, self.y, self.z].into()
    }
}

impl ConvertVecExtension<[usize; 3]> for Vector3<i32> {
    fn convert_vec(self) -> [usize; 3] {
        [self.x as usize, self.y as usize, self.z as usize]
    }
}

impl ConvertVecExtension<Vec3> for Vector3<f32> {
    fn convert_vec(self) -> Vec3 {
        [self.x, self.y, self.z].into()
    }
}

impl ConvertVecExtension<Vector3<f32>> for Vec3 {
    fn convert_vec(self) -> Vector3<f32> {
        [self.x, self.y, self.z].into()
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
