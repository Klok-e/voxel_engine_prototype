use super::chunk::Chunk;
use super::chunk::{ChunkPosition, CHUNK_SIZE};
use super::world::VoxelWorld;
use crate::core::{Vec3f, Vec3i};
use amethyst::{core::components::Transform, derive::SystemDesc, ecs::prelude::*, prelude::*};
use std::collections::{HashMap, HashSet};
use amethyst::core::math::{Quaternion, UnitQuaternion};
use amethyst::core::num::real::Real;
use crate::core::to_vecf;

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
        (mut voxel_world, load_around, mut chunk_positions, mut transforms, ents): Self::SystemData,
    ) {
        let mut loaded_chunks = HashSet::new();
        let mut chunks_to_load = HashSet::new();
        for (loader, transform) in (&load_around, &transforms).join() {
            let pos = transform.translation() / CHUNK_SIZE as f32;
            let pos = Vec3i::new(
                pos.x.floor() as i32,
                pos.y.floor() as i32,
                pos.z.floor() as i32,
            );

            for (chunk_pos, ) in (&chunk_positions, ).join() {
                loaded_chunks.insert(*chunk_pos);
            }

            for z in -loader.distance..=loader.distance {
                for y in -loader.distance..=loader.distance {
                    for x in -loader.distance..=loader.distance {
                        let pos = ChunkPosition::new(Vec3i::new(x, y, z) + pos.clone());
                        chunks_to_load.insert(pos);
                    }
                }
            }
        }

        for to_load_pos in chunks_to_load.difference(&loaded_chunks) {
            // create mesh
            let chunk = voxel_world.chunk_at_or_create(&to_load_pos);

            // create entity
            ents.build_entity()
                .with(*to_load_pos, &mut chunk_positions)
                .with(Transform::new(to_vecf(to_load_pos.pos.clone()).into(),
                                     UnitQuaternion::identity(),
                                     Vec3f::identity().into()),
                      &mut transforms)
                .build();
        }
    }

    fn setup(&mut self, world: &mut World) {
        <Self as System<'_>>::SystemData::setup(world);
        world.register::<RenderAround>();
        world.register::<ChunkPosition>();
    }
}
