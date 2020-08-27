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
pub struct ChunkRenderSystem;

impl ChunkRenderSystem {
    fn init_materials(
        &self,
        tex_loader: AssetLoaderSystemData<Texture>,
        mat_loader: AssetLoaderSystemData<Material>,
    ) -> Materials {
        use amethyst::renderer::pod::Environment;
        use amethyst::renderer::{mtl::TextureOffset, ImageFormat};

        //let albedo = loaders::load_from_srgba(Srgba::new(0.5, 0.7, 0.5, 1.0));
        let emission = loaders::load_from_srgba(Srgba::new(0.0, 0.0, 0.0, 0.0));
        let normal = loaders::load_from_linear_rgba(LinSrgba::new(0.5, 0.5, 1.0, 1.0));
        let metallic_roughness = loaders::load_from_linear_rgba(LinSrgba::new(0.0, 0.5, 0.0, 0.0));
        let ambient_occlusion = loaders::load_from_linear_rgba(LinSrgba::new(1.0, 1.0, 1.0, 1.0));
        let cavity = loaders::load_from_linear_rgba(LinSrgba::new(1.0, 1.0, 1.0, 1.0));

        //let albedo = tex_loader.load_from_data(albedo.into(), ());
        let albedo = tex_loader.load("blocks/dirt.png", ImageFormat::default(), ());
        let emission = tex_loader.load_from_data(emission.into(), ());
        let normal = tex_loader.load_from_data(normal.into(), ());
        let metallic_roughness = tex_loader.load_from_data(metallic_roughness.into(), ());
        let ambient_occlusion = tex_loader.load_from_data(ambient_occlusion.into(), ());
        let cavity = tex_loader.load_from_data(cavity.into(), ());

        let chunks = mat_loader.load_from_data(
            Material {
                alpha_cutoff: 0.01,
                albedo,
                emission,
                normal,
                metallic_roughness,
                ambient_occlusion,
                cavity,
                uv_offset: TextureOffset::default(),
            },
            (),
        );
        Materials { chunks }
    }
}

impl<'a> System<'a> for ChunkRenderSystem {
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

            // create mesh
            let mesh = chunk
                .mesh()
                .build_mesh()
                .map(|m| MeshData(m.into_owned()))
                .map(|m| mesh_loader.load_from_data(m, ()));

            let mut transform = Transform::default();
            transform.set_translation(to_vecf(to_load_pos.pos * CHSIZEI));

            // draw debug lines
            let mut debug_lines = DebugLinesComponent::new();
            debug_lines.add_box(
                (to_vecf(to_load_pos.pos) * CHSIZEF).into(),
                ((to_vecf(to_load_pos.pos) + Vec3f::from([1., 1., 1.])) * CHSIZEF).into(),
                Srgba::new(0.1, 0.1, 0.1, 0.5),
            );

            let def_mat = materials.chunks.clone();

            // create entity
            ents.build_entity()
                .with(*to_load_pos, &mut chunk_positions)
                .with(transform, &mut transforms)
                .with(def_mat, &mut mats)
                .with_opt(mesh, &mut meshes)
                .with(debug_lines, &mut debugs)
                .with(
                    BoundingSphere::new(
                        (Vec3f::from([1., 1., 1.]) * CHSIZEF / 2.).into(),
                        // distance from center to outermost vertex of a cube
                        CHSIZEF * 3. / 2.,
                    ),
                    &mut bound_spheres,
                )
                .build();
        }
    }

    fn setup(&mut self, world: &mut World) {
        <Self as System<'_>>::SystemData::setup(world);
        world.register::<RenderAround>();
        world.register::<ChunkPosition>();

        world.insert(AmbientColor(Srgba::new(0.5, 0.5, 0.5, 1.0)));

        let mats = world.exec(|(tex, mat)| self.init_materials(tex, mat));
        world.insert(mats)
    }
}
