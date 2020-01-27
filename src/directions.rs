use crate::core::Vec3f;
use amethyst::core::math;

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
    pub fn to_vec(&self) -> Vec3f {
        let mut res = Vec3f::zeros();
        if self.intersects(Directions::UP) {
            res += Vec3f::y();
        }
        if self.intersects(Directions::DOWN) {
            res -= Vec3f::y();
        }
        if self.intersects(Directions::WEST) {
            res -= Vec3f::x();
        }
        if self.intersects(Directions::EAST) {
            res += Vec3f::x();
        }
        if self.intersects(Directions::NORTH) {
            res -= Vec3f::z();
        }
        if self.intersects(Directions::SOUTH) {
            res += Vec3f::z();
        }
        res
    }
}
