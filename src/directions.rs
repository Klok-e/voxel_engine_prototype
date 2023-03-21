use bevy::prelude::{IVec3, Vec3};
use bitflags::bitflags;

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
    pub fn to_ivec(self) -> IVec3 {
        IVec3::from(self)
    }

    #[inline]
    pub fn to_fvec(self) -> Vec3 {
        Vec3::from(self)
    }

    pub fn invert(self) -> Self {
        (-self.to_ivec()).into()
    }
}

impl From<Directions> for IVec3 {
    #[inline]
    fn from(dir: Directions) -> Self {
        let mut res = IVec3::ZERO;
        if dir.intersects(Directions::UP) {
            res += IVec3::Y;
        }
        if dir.intersects(Directions::DOWN) {
            res -= IVec3::Y;
        }
        if dir.intersects(Directions::WEST) {
            res -= IVec3::X;
        }
        if dir.intersects(Directions::EAST) {
            res += IVec3::X;
        }
        if dir.intersects(Directions::NORTH) {
            res -= IVec3::Z;
        }
        if dir.intersects(Directions::SOUTH) {
            res += IVec3::Z;
        }
        res
    }
}

impl From<Directions> for Vec3 {
    #[inline]
    fn from(dir: Directions) -> Self {
        let mut res = Vec3::ZERO;
        if dir.intersects(Directions::UP) {
            res += Vec3::Y;
        }
        if dir.intersects(Directions::DOWN) {
            res -= Vec3::Y;
        }
        if dir.intersects(Directions::WEST) {
            res -= Vec3::X;
        }
        if dir.intersects(Directions::EAST) {
            res += Vec3::X;
        }
        if dir.intersects(Directions::NORTH) {
            res -= Vec3::Z;
        }
        if dir.intersects(Directions::SOUTH) {
            res += Vec3::Z;
        }
        res
    }
}

impl From<IVec3> for Directions {
    #[inline]
    fn from(vec: IVec3) -> Self {
        let mut res = Directions::empty();
        if vec.x == IVec3::from(Directions::EAST).x {
            res |= Directions::EAST;
        } else if vec.x == IVec3::from(Directions::WEST).x {
            res |= Directions::WEST;
        }
        if vec.y == IVec3::from(Directions::UP).y {
            res |= Directions::UP;
        } else if vec.y == IVec3::from(Directions::DOWN).y {
            res |= Directions::DOWN;
        }
        if vec.z == IVec3::from(Directions::NORTH).z {
            res |= Directions::NORTH;
        } else if vec.z == IVec3::from(Directions::SOUTH).z {
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
