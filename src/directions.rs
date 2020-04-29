use crate::core::{Vec3f, Vec3i};
use amethyst::core::math::{self, base::Scalar, Vector3};
use amethyst::core::num::{NumAssignRef, PrimInt};

bitflags! {
    pub struct Directions: u8 {
        const NORTH = 1 << 0;
        const SOUTH = 1 << 1;
        const WEST =  1 << 2;
        const EAST =  1 << 3;
        const UP =    1 << 4;
        const DOWN =  1 << 5;
    }
}

impl Directions {
    //#[deprecated]
    pub fn to_vec<T>(&self) -> Vector3<T>
    where
        T: NumAssignRef + Scalar,
    {
        Vector3::<T>::from(*self)
    }
}

impl<T> From<Directions> for Vector3<T>
where
    T: NumAssignRef + Scalar,
{
    fn from(dir: Directions) -> Self {
        let mut res = Vector3::<T>::zeros();
        if dir.intersects(Directions::UP) {
            res += Vector3::<T>::y();
        }
        if dir.intersects(Directions::DOWN) {
            res -= Vector3::<T>::y();
        }
        if dir.intersects(Directions::WEST) {
            res -= Vector3::<T>::x();
        }
        if dir.intersects(Directions::EAST) {
            res += Vector3::<T>::x();
        }
        if dir.intersects(Directions::NORTH) {
            res -= Vector3::<T>::z();
        }
        if dir.intersects(Directions::SOUTH) {
            res += Vector3::<T>::z();
        }
        res
    }
}

impl<T> From<Vector3<T>> for Directions
where
    T: PrimInt + Scalar + NumAssignRef,
{
    fn from(vec: Vector3<T>) -> Self {
        let mut res = Directions::empty();
        if vec.x == Vector3::<T>::from(Directions::EAST).x {
            res |= Directions::EAST;
        } else if vec.x == Vector3::<T>::from(Directions::WEST).x {
            res |= Directions::WEST;
        }
        if vec.y == Vector3::<T>::from(Directions::UP).y {
            res |= Directions::UP;
        } else if vec.y == Vector3::<T>::from(Directions::DOWN).y {
            res |= Directions::DOWN;
        }
        if vec.z == Vector3::<T>::from(Directions::NORTH).z {
            res |= Directions::NORTH;
        } else if vec.z == Vector3::<T>::from(Directions::SOUTH).z {
            res |= Directions::SOUTH;
        }
        res
    }
}
