use amethyst::{assets::Handle, renderer::Material};

#[derive(Debug, Clone)]
pub struct Materials {
    pub chunks: Handle<Material>,
}
