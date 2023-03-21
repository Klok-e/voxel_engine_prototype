#[derive(Default, Debug, Copy, Clone)]
pub struct Voxel {
    pub id: u16,
}

impl Voxel {
    #[inline]
    pub fn is_transparent(&self) -> bool {
        matches!(self.id, 0)
    }
}

impl From<u16> for Voxel {
    fn from(v: u16) -> Self {
        Voxel { id: v }
    }
}
