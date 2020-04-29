use super::chunk::{Chunk, ChunkPosition};
use super::world::ChunkPrepareStage;
use amethyst::{core::components::Transform, derive::SystemDesc, ecs::prelude::*, prelude::*};

#[derive(SystemDesc)]
pub struct TerrainSystem;

impl<'a> System<'a> for TerrainSystem {
    type SystemData = (
        WriteStorage<'a, ChunkPrepareStage>,
        WriteStorage<'a, ChunkPosition>,
        WriteStorage<'a, Chunk>,
    );

    fn run(&mut self, (mut stages, mut positions, mut chunks): Self::SystemData) {
        for (stage, position, chunk) in (&stages, &positions, &chunks).join() {}
    }
}
