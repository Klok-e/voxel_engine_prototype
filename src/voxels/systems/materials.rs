use bevy::prelude::{Handle, Resource, StandardMaterial};

#[derive(Debug, Clone, Resource)]
pub struct Materials {
    pub material: Handle<StandardMaterial>,
}
