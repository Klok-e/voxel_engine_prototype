use super::chunk::Chunk;
use super::chunk::{ChunkPosition, CHUNK_SIZE};
use super::world::VoxelWorld;
use crate::core::{Vec3f, Vec3i};
use amethyst::{core::components::Transform, derive::SystemDesc, ecs::prelude::*, prelude::*};
use std::collections::{HashMap, HashSet};

pub struct RenderAround {
    pub distance: i32,
}

impl RenderAround {
    pub fn new(distance: i32) -> Self {
        RenderAround { distance }
    }
}

impl Component for RenderAround {
    type Storage = DenseVecStorage<Self>;
}

#[derive(SystemDesc)]
pub struct ChunkRenderSystem;

impl<'a> System<'a> for ChunkRenderSystem {
    type SystemData = (
        Write<'a, VoxelWorld>,
        ReadStorage<'a, RenderAround>,
        WriteStorage<'a, ChunkPosition>,
        WriteStorage<'a, Transform>,
        Entities<'a>,
    );

    fn run(
        &mut self,
        (mut voxel_world, load_around, mut chunk_positions, transforms, ents): Self::SystemData,
    ) {
        let mut chunk_ents = HashSet::new();
        for (loader, transform) in (&load_around, &transforms).join() {
            let pos = transform.translation() / CHUNK_SIZE as f32;
            let pos = Vec3i::new(
                pos.x.floor() as i32,
                pos.y.floor() as i32,
                pos.z.floor() as i32,
            );

            for (chunk_pos) in (&mut chunk_positions,).join() {
                chunk_ents.insert(chunk_pos.clone())
            }

            for z in -loader.distance..=loader.distance {
                for y in -loader.distance..=loader.distance {
                    for x in -loader.distance..=loader.distance {
                        let coord: Vec3i = Vec3i::new(x, y, z) + pos.clone();
                        if !chunk_ents.contains(&coord) {
                            // create an entity representing this chunk
                            //ents.build_entity().with()
                        }
                    }
                }
            }
        }
    }

    fn setup(&mut self, world: &mut World) {
        <Self as System<'_>>::SystemData::setup(world);
        world.register::<RenderAround>();
        world.register::<ChunkPosition>();
    }
}
