use super::chunk::Chunk;
use super::chunk::{ChunkPosition, CHUNK_SIZE, CHUNK_SIZEI};
use super::world::VoxelWorld;
use crate::core::to_vecf;
use crate::core::{Vec3f, Vec3i};
use amethyst::assets::{AssetLoaderSystemData, AssetStorage, Handle, Loader};
use amethyst::core::math::{Quaternion, UnitQuaternion};
use amethyst::core::num::real::Real;
use amethyst::renderer::palette::LinSrgba;
use amethyst::renderer::types::MeshData;
use amethyst::renderer::{loaders, Material, MaterialDefaults, Mesh, Texture};
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
        AssetLoaderSystemData<'a, Mesh>,
        WriteStorage<'a, Handle<Mesh>>,
        AssetLoaderSystemData<'a, Texture>,
        AssetLoaderSystemData<'a, Material>,
        WriteStorage<'a, Handle<Material>>,
        ReadExpect<'a, MaterialDefaults>,
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
            tex_loader,
            mat_loader,
            mut mats,
            mat_default,
        ): Self::SystemData,
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
            let index: Vec3f = transform.translation() - to_vecf(pos * CHUNK_SIZEI);
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

        for to_load_pos in chunks_to_load.difference(&loaded_chunks) {
            // create mesh
            let chunk = voxel_world.chunk_at_or_create(&to_load_pos);
            let mesh = MeshData(chunk.write().unwrap().mesh().into_owned());

            let mesh: Handle<Mesh> = mesh_loader.load_from_data(mesh, ());
            let albedo = tex_loader.load_from_data(
                loaders::load_from_linear_rgba(LinSrgba::new(0.0, 1.0, 0.0, 1.0)).into(),
                (),
            );
            let mat = mat_loader.load_from_data(
                Material {
                    albedo,
                    ..mat_default.0.clone()
                },
                (),
            );

            let mut transform = Transform::default();
            transform.set_translation(to_vecf(to_load_pos.pos * CHUNK_SIZEI));
            // create entity
            ents.build_entity()
                .with(*to_load_pos, &mut chunk_positions)
                .with(transform, &mut transforms)
                .with(mat, &mut mats)
                .with(mesh, &mut meshes)
                .build();
        }
    }

    fn setup(&mut self, world: &mut World) {
        <Self as System<'_>>::SystemData::setup(world);
        world.register::<RenderAround>();
        world.register::<ChunkPosition>();
    }
}
