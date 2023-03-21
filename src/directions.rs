use std::iter::from_fn;

use bitflags::bitflags;

use nalgebra::{Scalar, Vector3};
use num::{traits::NumAssignRef, PrimInt};

bitflags! {
    #[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, PartialOrd, Ord)]
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
    #[inline]
    pub fn to_vec<T>(&self) -> Vector3<T>
    where
        T: NumAssignRef + Scalar,
    {
        Vector3::<T>::from(*self)
    }

    pub fn into_iter(self) -> impl Iterator<Item = Self> {
        let mut i = 0u8;
        const MAX: u8 = 6u8;
        from_fn(move || {
            let mut res = Directions::from_bits_truncate(1 << i);
            while !self.contains(res) && i < MAX {
                i += 1;
                res = Directions::from_bits_truncate(1 << i);
            }
            let prev_i = i;
            i += 1;
            if prev_i >= MAX {
                None
            } else {
                Some(res)
            }
        })
    }

    pub fn invert(self) -> Self {
        (-self.to_vec::<i32>()).into()
    }
}

impl<T> From<Directions> for Vector3<T>
where
    T: NumAssignRef + Scalar,
{
    #[inline]
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
    #[inline]
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

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest(dir, expected_vec,
        case::north_west(Directions::NORTH | Directions::WEST, vec![Directions::NORTH, Directions::WEST]),
        case::up_down(Directions::UP | Directions::DOWN, vec![Directions::UP, Directions::DOWN]),
        case::all(Directions::all(), vec![Directions::NORTH, Directions::SOUTH, Directions::WEST, Directions::EAST, Directions::UP, Directions::DOWN]),
    )]
    fn direction_iter(dir: Directions, expected_vec: Vec<Directions>) {
        dbg!(&dir);
        dbg!(&expected_vec);
        assert_eq!(dir.into_iter().collect::<Vec<_>>(), expected_vec);
    }
}
