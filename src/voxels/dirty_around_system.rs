use super::chunk::Chunk;
use super::chunk::{ChunkPosition, CHSIZE, CHSIZEF, CHSIZEI};
use super::materials::Materials;
use super::world::VoxelWorld;
use crate::core::to_vecf;
use crate::core::{EntityBuildExt, Vec3f, Vec3i};
use crate::directions::Directions;
use amethyst::assets::{AssetLoaderSystemData, AssetStorage, Handle, Loader};
use amethyst::core::math::{one, zero, Quaternion, UnitQuaternion};
use amethyst::core::num::real::Real;
use amethyst::renderer::palette::LinSrgba;
use amethyst::renderer::resources::AmbientColor;
use amethyst::renderer::types::MeshData;
use amethyst::renderer::{
    debug_drawing::DebugLinesComponent, loaders, palette::Srgba, visibility::BoundingSphere,
    Material, MaterialDefaults, Mesh, Texture,
};
use amethyst::{core::components::Transform, derive::SystemDesc, ecs::prelude::*, prelude::*};
use flurry::epoch::pin;
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
pub struct DirtyAroundSystem;

impl<'a> System<'a> for DirtyAroundSystem {
    type SystemData = (
        Write<'a, VoxelWorld>,
        ReadStorage<'a, RenderAround>,
        WriteStorage<'a, ChunkPosition>,
        WriteStorage<'a, Transform>,
        Entities<'a>,
        AssetLoaderSystemData<'a, Mesh>,
        WriteStorage<'a, Handle<Mesh>>,
        WriteStorage<'a, Handle<Material>>,
        WriteStorage<'a, DebugLinesComponent>,
        WriteStorage<'a, BoundingSphere>,
        WriteExpect<'a, Materials>,
    );

    fn run(
        &mut self,
        (
            mut voxel_world,
            load_around,
            mut chunk_positions,
            mut transforms,
            ents,
            mesh_loader,
            mut meshes,
            mut mats,
            mut debugs,
            mut bound_spheres,
            mut materials,
        ): Self::SystemData,
    ) {
        let mut loaded_chunks = HashSet::new();
        let mut chunks_to_load = HashSet::new();
        for (loader, transform) in (&load_around, &transforms).join() {
            let pos = transform.translation() / CHSIZE as f32;
            let pos = Vec3i::new(
                pos.x.floor() as i32,
                pos.y.floor() as i32,
                pos.z.floor() as i32,
            );
            let index: Vec3f = transform.translation() - to_vecf(pos * CHSIZEI);
            let index = [
                index.x.floor() as usize,
                index.y.floor() as usize,
                index.z.floor() as usize,
            ];
            //dbg!(voxel_world.voxel_at(&ChunkPosition::new(pos), &index));

            for (chunk_pos,) in (&chunk_positions,).join() {
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

        let guard = pin();
        for to_load_pos in chunks_to_load.difference(&loaded_chunks) {
            let mut chunk = voxel_world
                .chunk_at_or_create(&to_load_pos, &guard)
                .write()
                .unwrap();
            for (dir, dirvec) in Directions::all()
                .into_iter()
                .map(|d| (d, d.to_vec::<i32>()))
            {
                let neighb = voxel_world
                    .chunk_at_or_create(&(to_load_pos.pos + dirvec).into(), &guard)
                    .read()
                    .unwrap();

                chunk.copy_borders(&*neighb, dir);
            }

            voxel_world.dirty().insert(*to_load_pos, &guard);
        }
    }

    fn setup(&mut self, world: &mut World) {
        <Self as System<'_>>::SystemData::setup(world);
        world.register::<RenderAround>();
        world.register::<ChunkPosition>();
    }
}
