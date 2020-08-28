use super::chunk::Chunk;
use super::chunk::{ChunkPosition, CHSIZE, CHSIZEF, CHSIZEI};
use super::materials::Materials;
use super::{dirty_around_system::RenderAround, world::VoxelWorld};
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
            mut ents,
            mesh_loader,
            mut meshes,
            mut mats,
            mut debugs,
            mut bound_spheres,
            mut materials,
        ): Self::SystemData,
    ) {
        let mut chunk_entities = HashMap::new();
        for (chunk_pos, ent) in (&chunk_positions, &ents).join() {
            chunk_entities.insert(*chunk_pos, ent);
        }

        let guard = pin();
        for to_clean in voxel_world.dirty().iter(&guard) {
            let chunk = voxel_world
                .chunk_at_or_create(&to_clean, &guard)
                .read()
                .unwrap();

            // create mesh
            let mesh = chunk
                .mesh()
                .build_mesh()
                .map(|m| MeshData(m.into_owned()))
                .map(|m| mesh_loader.load_from_data(m, ()));

            if let Some(m) = mesh {
                let entity = chunk_entities.get(to_clean).map(|v| *v).unwrap_or_else(|| {
                    let mut transform = Transform::default();
                    transform.set_translation(to_vecf(to_clean.pos * CHSIZEI));

                    // draw debug lines
                    let mut debug_lines = DebugLinesComponent::new();
                    debug_lines.add_box(
                        (to_vecf(to_clean.pos) * CHSIZEF).into(),
                        ((to_vecf(to_clean.pos) + Vec3f::from([1., 1., 1.])) * CHSIZEF).into(),
                        Srgba::new(0.1, 0.1, 0.1, 0.5),
                    );

                    let def_mat = materials.chunks.clone();

                    // create entity
                    ents.build_entity()
                        .with(*to_clean, &mut chunk_positions)
                        .with(transform, &mut transforms)
                        .with(def_mat, &mut mats)
                        .with(debug_lines, &mut debugs)
                        .with(
                            BoundingSphere::new(
                                (Vec3f::from([1., 1., 1.]) * CHSIZEF / 2.).into(),
                                // distance from center to outermost vertex of a cube
                                CHSIZEF * 3. / 2.,
                            ),
                            &mut bound_spheres,
                        )
                        .build()
                });
                meshes.insert(entity, m).unwrap();
            }
        }
        voxel_world.dirty().clear(&guard);
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
