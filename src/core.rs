use nalgebra::{Vector, Vector2};

pub trait ConvertVecExtension<T> {
    fn convert_vec(self) -> T;
}

impl ConvertVecExtension<Vector2<f64>> for Vector2<i32> {
    fn convert_vec(self) -> Vector2<f64> {
        [self.x() as f64, self.y() as f64, self.z() as f64].into()
    }
}
