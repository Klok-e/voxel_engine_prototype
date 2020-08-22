#[derive(Default, Debug, Copy, Clone)]
pub struct Voxel {
    pub id: u16,
}

impl Voxel {
    pub fn is_transparent(&self) -> bool {
        match self.id {
            0 => true,
            _ => false,
        }
    }
}

impl From<u16> for Voxel {
    fn from(v: u16) -> Self {
        Voxel { id: v }
    }
}
