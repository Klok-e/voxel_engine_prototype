use super::{
    chunk::{ChunkPosition, CHSIZEF, CHSIZEI},
    dirty_around_system::RenderAround,
    materials::Materials,
    world::VoxelWorld,
};
use crate::{
    core::{to_vecf, Vec3f},
    game_config::GameConfig,
};
use amethyst::{
    assets::{AssetLoaderSystemData, Handle},
    core::components::Transform,
    derive::SystemDesc,
    ecs::prelude::*,
    renderer::{
        debug_drawing::DebugLinesComponent, palette::Srgba, types::MeshData,
        visibility::BoundingSphere, Material, Mesh,
    },
};
use flurry::epoch::pin;
use std::collections::HashMap;

#[derive(SystemDesc)]
pub struct ChunkRenderSystem;

impl<'a> System<'a> for ChunkRenderSystem {
    type SystemData = (
        Read<'a, VoxelWorld>,
        WriteStorage<'a, ChunkPosition>,
        WriteStorage<'a, Transform>,
        AssetLoaderSystemData<'a, Mesh>,
        WriteStorage<'a, Handle<Mesh>>,
        WriteStorage<'a, Handle<Material>>,
        WriteStorage<'a, DebugLinesComponent>,
        WriteStorage<'a, BoundingSphere>,
        ReadExpect<'a, GameConfig>,
        Entities<'a>,
        ReadExpect<'a, Materials>,
    );

    fn run(
        &mut self,
        (
            voxel_world,
            mut chunk_positions,
            mut transforms,
            mesh_loader,
            mut meshes,
            mut mats,
            mut debugs,
            mut bound_spheres,
            config,
            ents,
            materials,
        ): Self::SystemData,
    ) {
        let mut chunk_entities = HashMap::new();
        for (chunk_pos, ent) in (&chunk_positions, &ents).join() {
            chunk_entities.insert(*chunk_pos, ent);
        }

        let guard = pin();
        for to_clean in voxel_world
            .dirty()
            .iter(&guard)
            .take(config.chunks_render_per_frame)
        {
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

            // get entity from hashmap or create a new one
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
            if let Some(m) = mesh {
                meshes.insert(entity, m).unwrap();
            }

            voxel_world.dirty().remove(to_clean, &guard);
        }
    }

    fn setup(&mut self, world: &mut World) {
        <Self as System<'_>>::SystemData::setup(world);
        world.register::<RenderAround>();
        world.register::<ChunkPosition>();
    }
}
