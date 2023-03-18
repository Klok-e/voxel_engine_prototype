use bevy::prelude::{Handle, Material, Resource, StandardMaterial};

#[derive(Debug, Clone, Resource)]
pub struct Materials {
    pub material: Handle<StandardMaterial>,
}
