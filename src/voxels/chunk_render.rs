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

pub enum ChunkPrepareStage {
    TerrainGeneration,
    Meshing,
}

impl Default for ChunkPrepareStage {
    fn default() -> Self {
        ChunkPrepareStage::TerrainGeneration
    }
}

impl Component for ChunkPrepareStage {
    type Storage = DenseVecStorage<Self>;
}

#[derive(SystemDesc)]
pub struct ChunkRenderSystem;

impl<'a> System<'a> for ChunkRenderSystem {
    type SystemData = (
        Write<'a, VoxelWorld>,
        WriteStorage<'a, Chunk>,
        ReadStorage<'a, RenderAround>,
        WriteStorage<'a, ChunkPosition>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, ChunkPrepareStage>,
        Entities<'a>,
    );

    fn run(
        &mut self,
        (
            mut voxel_world,
            mut chunks,
            load_around,
            mut chunk_positions,
            transforms,
            mut chunk_stages,
            ents,
        ): Self::SystemData,
    ) {
        let mut maintained = HashSet::new();
        for (loader, transform) in (&load_around, &transforms).join() {
            let pos = transform.translation() / CHUNK_SIZE as f32;
            let pos = Vec3i::new(
                pos.x.floor() as i32,
                pos.y.floor() as i32,
                pos.z.floor() as i32,
            );
            for z in -loader.distance..=loader.distance {
                for y in -loader.distance..=loader.distance {
                    for x in -loader.distance..=loader.distance {
                        let coord: Vec3i = Vec3i::new(x, y, z) + pos.clone();
                        maintained.insert(coord);
                    }
                }
            }
        }
        let occupied = voxel_world.chunks.keys().cloned().collect::<HashSet<_>>();
        let to_delete = occupied.difference(&maintained);
        for coord in to_delete {
            let ent = voxel_world.chunks.remove(coord).unwrap();
            ents.delete(ent).unwrap();
        }
        let to_create = maintained.difference(&occupied);
        for &coord in to_create {
            let ent = ents.create();
            chunks.insert(ent, Chunk::new()).unwrap();
            chunk_positions.insert(ent, ChunkPosition::new(coord.into()));
            chunk_stages.insert(ent, ChunkPrepareStage::default());
            voxel_world.chunks.insert(coord.into(), ent);
        }
    }

    fn setup(&mut self, world: &mut World) {
        <Self as System<'_>>::SystemData::setup(world);
        world.register::<RenderAround>();
        world.register::<Chunk>();
        world.register::<ChunkPosition>();
    }
}
